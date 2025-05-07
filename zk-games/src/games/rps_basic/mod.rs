use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sp1_sdk::{HashableKey, Prover, ProverClient, SP1Stdin};
use zk_games_types::{GameResult, RpsBasicInput};

use crate::zk::get_secret;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl TryFrom<u8> for Choice {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Choice::Rock as u8 => Ok(Choice::Rock),
            x if x == Choice::Paper as u8 => Ok(Choice::Paper),
            x if x == Choice::Scissors as u8 => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

pub fn generate_basic_choice_hash(
    username: &str,
    client_pubkey: &str,
    game_id: u64,
    choice: u8,
) -> Result<[u8; 32], String> {
    let secret = get_secret(username)?;

    Ok(Sha256::new()
        .chain_update(secret)
        .chain_update(client_pubkey)
        .chain_update(game_id.to_string())
        .chain_update(choice.to_string())
        .finalize()
        .into())
}

pub fn generate_basic_game_proof<'a>(
    username: &str,
    client_pubkey: &str,
    game_id: u64,
    choice_hash: [u8; 32],
) -> Result<(Vec<u8>, Vec<u8>, String), String> {
    let secret = get_secret(username)?;

    let rps_basic_input = RpsBasicInput {
        client_pubkey: client_pubkey.to_string(),
        game_id,
        choice_hash,
        secret,
    };

    // Start generating the proof
    let client = ProverClient::builder()
        .network()
        .private_key("8f7dfd6e9520ef786fc6f332f24f7f508bef46a5e71e289f4a17445e5004f29a")
        .rpc_url("https://rpc.production.succinct.xyz")
        .build();
    let rps_basic_elf = fs::read(Path::new(
        "/mnt/extra/Projects/solana/zk-games/zk-games-programs/rps-basic/elf/rps-basic-zk-program",
    ))
    .unwrap();

    let (pk, vk) = client.setup(rps_basic_elf.as_slice());
    println!("vk: {:?}", vk.bytes32());
    let mut stdin = SP1Stdin::new();
    stdin.write(&rps_basic_input);

    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("Groth16 proof generation failed");

    let proof_bytes = proof.bytes();
    println!("proof: {proof_bytes:?}");
    let public_values = proof.public_values.to_vec();

    Ok((proof_bytes, public_values, vk.bytes32()))
}

pub fn calculate_result(choice_1: u8, choice_2: u8) -> GameResult {
    match (choice_1, choice_2) {
        (1, 0) | (2, 1) | (0, 2) => GameResult::Player1,
        (0, 1) | (1, 2) | (2, 0) => GameResult::Player2,
        _ => GameResult::Draw,
    }
}

// TODO: Save on chain the result

#[cfg(test)]
mod test {
    use crate::games::rps_basic::Choice;

    #[test]
    fn test_results() {
        // Test the player 1 wins when it should
        assert_eq!(
            super::calculate_result(Choice::Rock as u8, Choice::Scissors as u8),
            super::GameResult::Player1
        );
        assert_eq!(
            super::calculate_result(Choice::Paper as u8, Choice::Rock as u8),
            super::GameResult::Player1
        );
        assert_eq!(
            super::calculate_result(Choice::Scissors as u8, Choice::Paper as u8),
            super::GameResult::Player1
        );

        // Test player 2 wins when it should
        assert_eq!(
            super::calculate_result(Choice::Scissors as u8, Choice::Rock as u8),
            super::GameResult::Player2
        );
        assert_eq!(
            super::calculate_result(Choice::Rock as u8, Choice::Paper as u8),
            super::GameResult::Player2
        );
        assert_eq!(
            super::calculate_result(Choice::Paper as u8, Choice::Scissors as u8),
            super::GameResult::Player2
        );

        // Test draw
        assert_eq!(
            super::calculate_result(Choice::Rock as u8, Choice::Rock as u8),
            super::GameResult::Draw
        );
        assert_eq!(
            super::calculate_result(Choice::Paper as u8, Choice::Paper as u8),
            super::GameResult::Draw
        );
        assert_eq!(
            super::calculate_result(Choice::Scissors as u8, Choice::Scissors as u8),
            super::GameResult::Draw
        );
    }
}

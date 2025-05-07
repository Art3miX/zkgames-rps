#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use zk_games_types::{RpsBasicInput, RpsBasicPublic};

fn main() {
    let rps_basic_input = sp1_zkvm::io::read::<RpsBasicInput>();

    // We brute force the choice by creating a hash that matches
    for choice in 0..3 {
        let choice_hash = Sha256::new()
            .chain_update(rps_basic_input.secret)
            .chain_update(rps_basic_input.client_pubkey.clone())
            .chain_update(rps_basic_input.game_id.to_string())
            .chain_update(choice.to_string())
            .finalize()
            .into();

        if choice_hash == rps_basic_input.choice_hash {
            // We found the right choice
            sp1_zkvm::io::commit(&RpsBasicPublic {
                game_id: rps_basic_input.game_id,
                choice_hash,
                choice,
            });
            return;
        }
    }

    panic!("No matching choice found");
}

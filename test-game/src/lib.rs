#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufWriter, Write},
        path::Path,
    };

    use sp1_sdk::{include_elf, Prover, ProverClient, SP1Stdin};

    use zk_games::user;
    use zk_games_types::{LoginInput, LoginPublic};

    const LOGIN_ELF: &[u8] = include_elf!("login-zk-program");
    // const RPS_BASIC_ELF: &[u8] = include_elf!("rps-basic-zk-program");

    fn save_public_login_hash(username: &str, login_hash: [u8; 32]) {
        let user_local_path = Path::new(user::PUBLIC_PLAYERS_PATH).join(username);

        if !user_local_path.exists() {
            std::fs::create_dir_all(&user_local_path).unwrap();
        }

        let filename = user_local_path.join(user::LOGIN_HASH_FILENAME);

        // Save games to a file or database
        let file = File::create(filename).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&login_hash).unwrap();
        writer.flush().unwrap();
    }

    #[test]
    fn example() {
        let username = "username";
        let password = "some_password1234";

        // Generate hashes for username
        let login_hash = user::create_account(username, password).unwrap();
        // Save the public login_hash to our local data
        save_public_login_hash(username, login_hash);

        // try to generate proof
        let client = ProverClient::builder().mock().build();
        let (pk, vk) = client.setup(LOGIN_ELF);

        println!(
            "Login Program Verification Key Bytes {:?}",
            sp1_sdk::HashableKey::bytes32(&vk)
        );

        let secret_path = Path::new(user::LOCAL_PLAYERS_PATH)
            .join(username)
            .join(user::SECRET_FILENAME);
        let my_secret = std::fs::read(secret_path).unwrap();

        let login_input = LoginInput {
            secret: *my_secret
                .as_slice()
                .first_chunk::<32>()
                .expect("secret should be length of 32"),
            login_hash,
            random_string: "1234".to_string(),
        };

        let mut stdin = SP1Stdin::new();
        stdin.write(&login_input);

        let mut proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("Groth16 proof generation failed");

        proof
            .save("/mnt/extra/Projects/solana/zk-games/data/proofs/username-1234.bin")
            .unwrap();

        client
            .verify(&proof, &vk)
            .expect("Groth16 proof verification failed");

        let res = proof.public_values.read::<LoginPublic>();

        println!("Login Result: {:?}", res);
    }
}

use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use sp1_sdk::{Prover, ProverClient, SP1Stdin};
use strum::{Display, EnumString, FromRepr, VariantArray};
use zk_games::{user, zk::LoginPublic};

use rand::{distr::Alphanumeric, Rng};

#[derive(Debug, PartialEq, Eq, VariantArray, EnumString, Display, FromRepr)]
enum UserMenu {
    #[strum(to_string = "Log in")]
    Login,
    #[strum(to_string = "Log in with password")]
    LoginPass,
    #[strum(to_string = "Register")]
    Register,
}

pub(crate) fn handle_user_not_logged_in() -> String {
    let selection = UserMenu::from_repr(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Not logged in:")
            .default(0)
            .items(UserMenu::VARIANTS)
            .interact()
            .unwrap(),
    );

    match selection {
        Some(UserMenu::Login) => {
            let username = get_user("Username:");
            log_in(username)
        }
        Some(UserMenu::LoginPass) => {
            let username = get_user("Username:");
            let password = get_password(false);
            log_in_with_pass(username, password)
        }
        Some(UserMenu::Register) => {
            let username = get_user("Username:");
            let password = get_password(true);
            register(username, password)
        }
        None => {
            println!("Invalid selection. Please try again.");
            handle_user_not_logged_in()
        }
    }
}

fn get_user(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

fn get_password(with_confirm: bool) -> String {
    let theme = &ColorfulTheme::default();
    let pass_editor = Password::with_theme(theme);
    let pass_editor = pass_editor.with_prompt("Password:");

    let pass_editor = if with_confirm {
        pass_editor.with_confirmation("Repeat password", "Error: the passwords don't match.")
    } else {
        pass_editor
    };

    pass_editor.interact().unwrap()
}

// TODO: Implement the login logic
fn log_in(username: String) -> String {
    println!("Logging in...");

    // Read secret from local file
    let secret_path = Path::new(user::LOCAL_PLAYERS_PATH)
        .join(&username)
        .join(user::SECRET_FILENAME);
    if !secret_path.exists() {
        println!("Error: Secret doesn't exists, please try to recover");
        return handle_user_not_logged_in();
    }
    let my_secret = *std::fs::read(secret_path)
        .unwrap()
        .as_slice()
        .first_chunk::<32>()
        .expect("secret should be length of 32");

    // Read login_hash from public file (chain)
    let login_hash = match get_login_hash(&username) {
        Ok(hash) => hash,
        Err(e) => {
            println!("Error: {}", e);
            return handle_user_not_logged_in();
        }
    };

    // Generate random string for login session
    let random_string: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let login_proof_input = zk_games::zk::LoginInput {
        secret: my_secret,
        login_hash,
        random_string: random_string.clone(),
    };

    // Start generating the proof
    let client = ProverClient::builder().mock().build();
    let login_elf = fs::read(Path::new(
        "/mnt/extra/Projects/solana/zk-games/zk-games-programs/login/elf/login-zk-program",
    ))
    .unwrap();
    let (pk, vk) = client.setup(login_elf.as_slice());

    let mut stdin = SP1Stdin::new();
    stdin.write(&login_proof_input);

    // Generate the proof
    let mut proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("Groth16 proof generation failed");

    // Verify the proof locally
    client
        .verify(&proof, &vk)
        .expect("Groth16 proof verification failed");

    // Get proof public values
    let res = proof.public_values.read::<LoginPublic>();

    // Make sure proof public info is correct
    if res.login_hash != login_hash && res.random_string != random_string {
        println!("Error: Login failed");
        return handle_user_not_logged_in();
    }

    // The proof is verified and the login is successful
    username
}

fn log_in_with_pass(username: String, password: String) -> String {
    println!("Logging in with password...");

    let _ = match user::create_account(username.as_str(), password.as_str()) {
        Ok(hash) => hash,
        Err(e) => {
            println!("Error: {}", e);
            return handle_user_not_logged_in();
        }
    };

    log_in(username)
}

fn register(username: String, password: String) -> String {
    let login_hash = match user::create_account(&username, &password) {
        Ok(hash) => hash,
        Err(e) => {
            println!("Error: {}", e);
            return handle_user_not_logged_in();
        }
    };

    save_public_login_hash(&username, login_hash);
    username
}

// TODO: Save login_hash and username on Solana
fn save_public_login_hash(username: &str, login_hash: [u8; 32]) {
    let user_public_path = Path::new(user::PUBLIC_PLAYERS_PATH).join(username);

    if !user_public_path.exists() {
        std::fs::create_dir_all(&user_public_path).unwrap();
    }

    let filename = user_public_path.join(user::LOGIN_HASH_FILENAME);

    // Save games to a file or database
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&login_hash).unwrap();
    writer.flush().unwrap();
}

// TODO: get login_hash from Solana
fn get_login_hash(username: &str) -> Result<[u8; 32], String> {
    let user_public_path = Path::new(user::PUBLIC_PLAYERS_PATH).join(username);

    if !user_public_path.exists() {
        std::fs::create_dir_all(&user_public_path).unwrap();
    }

    let filename = user_public_path.join(user::LOGIN_HASH_FILENAME);

    if !filename.exists() {
        return Err("Error: User doesn't exists".to_string());
    }

    Ok(*std::fs::read(filename)
        .unwrap()
        .as_slice()
        .first_chunk::<32>()
        .unwrap())
}

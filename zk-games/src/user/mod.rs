use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use sha2::{Digest, Sha256};

pub const LOCAL_PLAYERS_PATH: &str = "/mnt/extra/Projects/solana/zk-games/data/local-players";
pub const PUBLIC_PLAYERS_PATH: &str = "/mnt/extra/Projects/solana/zk-games/data/public-players";
pub const SECRET_FILENAME: &str = "secret";
pub const LOGIN_HASH_FILENAME: &str = "login-hash";

/// Very simple example to generate HASH from username and password
fn generate_secret(username: &str, password: &str) -> [u8; 32] {
    Sha256::new()
        .chain_update(username)
        .chain_update(password)
        .finalize()
        .into()
}

/// Takes secret and password and generates a public login hash
fn generate_login_hash(secret: [u8; 32]) -> [u8; 32] {
    Sha256::digest(secret).into()
}

/// Takes username and password and generates a secret and login hash
fn create_account_hashes(username: &str, password: &str) -> ([u8; 32], [u8; 32]) {
    let secret = generate_secret(username, password);
    println!("secret: {secret:?}");
    let login_hash = generate_login_hash(secret);
    println!("login hash: {login_hash:?}");
    (secret, login_hash)
}

fn save_secret(username: &str, secret: [u8; 32]) -> Result<(), String> {
    let user_local_path = Path::new(LOCAL_PLAYERS_PATH).join(username);

    if !user_local_path.exists() {
        std::fs::create_dir_all(&user_local_path).unwrap();
    }

    let filename = user_local_path.join(SECRET_FILENAME);

    if filename.exists() {
        return Err("Error: User already exists".to_string());
    }

    // Save games to a file or database
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&secret).unwrap();
    writer.flush().unwrap();
    Ok(())
}

pub fn create_account(username: &str, password: &str) -> Result<[u8; 32], String> {
    let (secret, login_hash) = create_account_hashes(username, password);

    save_secret(username, secret)?;
    Ok(login_hash)
}



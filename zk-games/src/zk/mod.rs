use std::path::Path;

use crate::user::{LOCAL_PLAYERS_PATH, SECRET_FILENAME};

pub fn get_secret(username: &str) -> Result<[u8; 32], String> {
    let secret_path = Path::new(LOCAL_PLAYERS_PATH)
        .join(username)
        .join(SECRET_FILENAME);
    if !secret_path.exists() {
        return Err("Secret doesn't exists, please try to recover".to_string());
    }
    Ok(*std::fs::read(secret_path)
        .unwrap()
        .as_slice()
        .first_chunk::<32>()
        .expect("secret should be length of 32"))
}

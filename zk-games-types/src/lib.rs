use serde::{Deserialize, Serialize};
use sp1_primitives::types::Buffer;

/// Once the game is finished, we set who the winner is
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GameResult {
    Player1,
    Player2,
    Draw,
}

/// Struct to provide as input values to the ZK login program
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInput {
    /// The login hash we try to prove
    pub login_hash: [u8; 32],
    /// A random string provided by the game client
    /// to confirm login attempt
    pub random_string: String,
    /// The secret to prove with
    pub secret: [u8; 32],
}

/// login result we get after verifying the proof
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPublic {
    pub login_hash: [u8; 32],
    pub random_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpsBasicInput {
    /// Client ID
    pub client_pubkey: String,
    /// Game ID
    pub game_id: u64,
    /// The choice hash we try to prove
    pub choice_hash: [u8; 32],
    /// The secret to prove with
    pub secret: [u8; 32],
}

/// login result we get after verifying the proof
#[derive(Serialize, Deserialize, Debug)]
pub struct RpsBasicPublic {
    pub game_id: u64,
    pub choice_hash: [u8; 32],
    pub choice: u8,
}

impl From<Vec<u8>> for RpsBasicPublic {
    fn from(bytes: Vec<u8>) -> Self {
        Buffer::from(&bytes).read::<RpsBasicPublic>()
        // .unwrap_or_else(|_| panic!("Failed to read RpsBasicPublic from bytes"))
        // SP1PublicValues::from().read::<RpsBasicPublic>()
    }
}

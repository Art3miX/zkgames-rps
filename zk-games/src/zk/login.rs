use serde::{Deserialize, Serialize};

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

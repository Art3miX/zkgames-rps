#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use zk_games::zk::{LoginInput, LoginPublic};

fn main() {
    let login_input = sp1_zkvm::io::read::<LoginInput>();

    // Create the login hash from the secret
    let login_hash: [u8; 32] = Sha256::digest(&login_input.secret).into();

    // Confirm the login hash with the provided hash
    assert_eq!(login_hash, login_input.login_hash);

    // Commit the login result to confirm the
    // right login hash and random string were used
    sp1_zkvm::io::commit(&LoginPublic {
        login_hash,
        random_string: login_input.random_string,
    });
}

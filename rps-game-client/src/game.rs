use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, FromRepr, VariantArray};
use zk_games::games::rps_basic::generate_basic_choice_hash;
use zk_games_types::GameResult;

use crate::{games_data::GamesData, GAME_CLIENT_ID};

#[derive(
    Serialize, Deserialize, Debug, PartialEq, Eq, VariantArray, EnumString, Display, FromRepr, Clone,
)]
pub enum Choice {
    #[strum(to_string = "Rock")]
    Rock,
    #[strum(to_string = "Paper")]
    Paper,
    #[strum(to_string = "Scissors")]
    Scissors,
}

/// Player 1 starts the game, so we want to get his username and the hash of his choice
/// To finilize the game, player1 must send proof of his choice after player2 chose his choice
#[derive(Serialize, Deserialize, Clone)]
pub struct Player1Info {
    pub username: String,
    pub choice_hash: [u8; 32],
}

/// Player2 joins a game, his choice can be sent as simple string
#[derive(Serialize, Deserialize, Clone)]
pub struct Player2Info {
    pub username: String,
    pub choice: Choice,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: u128,
    pub player1: Player1Info,
    pub player2: Option<Player2Info>,
    pub timeout: Option<u64>,
    pub result: Option<GameResult>,
}

impl Game {
    pub fn new(games_data: &GamesData, username: String, choice: Choice) -> Self {
        let id = games_data.get_next_id();
        let choice_hash = generate_basic_choice_hash(&username, GAME_CLIENT_ID, id, choice as u8).unwrap();

        Game {
            id,
            player1: Player1Info {
                username,
                choice_hash,
            },
            player2: None,
            timeout: None,
            result: None,
        }
    }
}

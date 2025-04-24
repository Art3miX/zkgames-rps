use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, FromRepr, VariantArray};

use crate::games_data::GamesData;

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

#[derive(Serialize, Deserialize, Clone)]
pub enum GameResult {
    Player1,
    Player2,
    Draw,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: u128,
    pub player1: String,
    pub player2: Option<String>,
    pub player1_choice: Choice,
    pub player2_choice: Option<Choice>,
    /// Result can either be "Player1", "Player2" or "Draw"
    /// If the game is not finished yet, it will be None
    pub result: Option<GameResult>,
}

impl Game {
    pub fn new(games_data: &GamesData, player1: String, choice: Choice) -> Self {
        let id = games_data.get_next_id();

        Game {
            id,
            player1,
            player2: None,
            player1_choice: choice,
            player2_choice: None,
            result: None,
        }
    }
}

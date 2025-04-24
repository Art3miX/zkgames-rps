use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde::{Deserialize, Serialize};

use crate::game::{Choice, Game, GameResult};

#[derive(Serialize, Deserialize)]
pub struct GamesData {
    pub games: Vec<Game>,
}

const GAMES_DATA_FILE: &str = "data/games.json";

impl Default for GamesData {
    fn default() -> Self {
        let file_path = std::env::current_dir().unwrap().join(GAMES_DATA_FILE);

        // Get game data file create if doesn't exists
        let file =
            File::open(file_path.clone()).unwrap_or_else(|_| File::create(file_path).unwrap());

        // Parse game data json if exists, create new one if doesn't
        serde_json::from_reader(file).unwrap_or_else(|_| GamesData { games: vec![] })
    }
}

impl GamesData {
    pub fn get_next_id(&self) -> u128 {
        if self.games.is_empty() {
            0
        } else {
            self.games.last().unwrap().id + 1
        }
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn _get_game(&self, id: u128) -> Option<&Game> {
        self.games.iter().find(|game| game.id == id)
    }

    pub fn get_game_mut(&mut self, id: u128) -> Option<&mut Game> {
        self.games.iter_mut().find(|game| game.id == id)
    }

    pub fn get_games(&self) -> &Vec<Game> {
        &self.games
    }

    pub fn join_game(&mut self, id: u128, player2: String, choice: Choice) {
        if let Some(game) = self.get_game_mut(id) {
            if game.player2.is_none() && game.player1 != player2 {
                game.player2 = Some(player2.clone());
                game.player2_choice = Some(choice);
            }
        }
    }

    pub fn save(&self) {
        let file_path = std::env::current_dir().unwrap().join(GAMES_DATA_FILE);

        // Save games to a file or database
        let file = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self).unwrap();
        writer.flush().unwrap();
    }

    pub fn calculate_result(&mut self, id: u128) {
        if let Some(game) = self.get_game_mut(id) {
            if let Some(player2_choice) = game.player2_choice.clone() {
                let player1_choice = game.player1_choice.clone();

                if player1_choice == player2_choice {
                    game.result = Some(GameResult::Draw);
                } else if (player1_choice == Choice::Rock && player2_choice == Choice::Scissors)
                    || (player1_choice == Choice::Paper && player2_choice == Choice::Rock)
                    || (player1_choice == Choice::Scissors && player2_choice == Choice::Paper)
                {
                    game.result = Some(GameResult::Player1);
                } else {
                    game.result = Some(GameResult::Player2);
                }
            }
        }
    }
}

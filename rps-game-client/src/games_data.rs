use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde::{Deserialize, Serialize};
use zk_games::games::rps_basic::{calculate_result, generate_basic_game_proof};
use zk_games_types::{GameResult, RpsBasicPublic};

use crate::{
    game::{Choice, Game, Player2Info},
    GAME_CLIENT_ID,
};

#[derive(Serialize, Deserialize)]
pub struct GamesData {
    pub games: Vec<Game>,
}

const GAMES_DATA_FILE: &str = "data/games.json";
const GAME_TIMEOUT: u64 = 600000; 

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

    pub fn get_game(&self, id: u128) -> Option<&Game> {
        self.games.iter().find(|game| game.id == id)
    }

    pub fn get_game_mut(&mut self, id: u128) -> Option<&mut Game> {
        self.games.iter_mut().find(|game| game.id == id)
    }

    pub fn get_games(&self) -> &Vec<Game> {
        &self.games
    }

    pub fn join_game(&mut self, id: u128, player2_username: String, choice: Choice) {
        if let Some(game) = self.get_game_mut(id) {
            if game.player2.is_none() && game.player1.username != player2_username {
                game.player2 = Some(Player2Info {
                    username: player2_username,
                    choice,
                });
                let curr_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                game.timeout = Some(curr_time + GAME_TIMEOUT);
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

    pub fn calculate_result(&mut self, id: u128) -> Result<GameResult, String> {
        if let Some(game) = self.get_game_mut(id) {
            // Generate choice proof locally
            let (_proof, public_values, _vk) = generate_basic_game_proof(
                &game.player1.username,
                GAME_CLIENT_ID,
                id,
                game.player1.choice_hash,
            )
            .unwrap();

            // TODO: Verify the proof
            // Groth16Verifier::verify(&proof, &public_values, &vk, *sp1_verifier::GROTH16_VK_BYTES)
            //     .unwrap();

            // Get the game result
            let public_values: RpsBasicPublic = public_values.into();
            let game_result = calculate_result(
                public_values.choice as u8,
                game.player2.clone().unwrap().choice as u8,
            );

            // Update the game result
            game.result = Some(game_result.clone());
            Ok(game_result)
        } else {
            return Err(format!("Game with ID {} not found", id));
        }
    }
}

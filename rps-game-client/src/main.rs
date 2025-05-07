mod game;
mod games_data;
mod menu;
mod user;

use game::{Choice, Game};
use games_data::GamesData;
use menu::show_main_menu;
use user::handle_user_not_logged_in;
use zk_games_types::GameResult;

pub const GAME_CLIENT_PUBKEY: &str = "5tBPvVcG2nn7jUQUW47WDbgUx96TZZ2qfzyfayZDDkbJ";

#[derive(Default)]
struct Data {
    user: String,
    game_data: GamesData,
}

impl Data {
    pub fn init() -> Self {
        dotenv::dotenv().ok();

        let mut data = Data::default();

        let username = handle_user_not_logged_in();

        data.set_user(username);
        data
    }

    fn set_user(&mut self, username: String) {
        self.user = username;
    }

    fn get_user(&self) -> String {
        self.user.clone()
    }

    fn get_games(&self) -> &Vec<Game> {
        self.game_data.get_games()
    }

    fn create_game(&mut self, choice: Choice) -> Game {
        let game = Game::new(&self.game_data, self.user.clone(), choice);
        self.game_data.add_game(game.clone());
        self.game_data.save();
        game
    }

    fn join_game(&mut self, id: u64, choice: Choice) {
        self.game_data.join_game(id, self.user.clone(), choice);
        self.game_data.save();
    }

    fn calculate_game_result(&mut self, id: u64) -> Result<GameResult, String> {
        let result = self.game_data.calculate_result(id)?;
        self.game_data.save();
        Ok(result)
    }
}

fn main() {
    println!("Welcome to ZK RPS CLI!");

    let mut data = Data::init();

    println!("You are logged in as: {}", data.user);

    show_main_menu(&mut data);
}

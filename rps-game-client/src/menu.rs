use std::collections::BTreeMap;

use dialoguer::{theme::ColorfulTheme, Select};
use strum::{Display, EnumString, FromRepr, VariantArray};
use zk_games_types::GameResult;

use crate::{game::Choice, Data};

#[derive(Debug, PartialEq, Eq, VariantArray, EnumString, Display, FromRepr)]
enum MainMenu {
    #[strum(to_string = "Create new game")]
    CreateGame,
    #[strum(to_string = "Join game")]
    JoinGame,
    #[strum(to_string = "Complete game")]
    CompleteGame,
    #[strum(to_string = "Exit")]
    Exit,
}

pub fn show_main_menu(data: &mut Data) {
    let selection = MainMenu::from_repr(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(MainMenu::VARIANTS)
            .interact()
            .unwrap(),
    );

    match selection {
        Some(MainMenu::CreateGame) => create_new_game(data),
        Some(MainMenu::JoinGame) => join_game(data),
        Some(MainMenu::CompleteGame) => complete_game(data),
        Some(MainMenu::Exit) => std::process::exit(0),
        None => println!("Invalid selection. Please try again."),
    }
}

fn create_new_game(data: &mut Data) {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter your choice")
        .default(0)
        .items(Choice::VARIANTS)
        .interact_opt()
        .unwrap();

    match selection {
        Some(choice) => {
            let choice = Choice::from_repr(choice).unwrap();
            let game = data.create_game(choice);
            println!("Game was created with id: {}", game.id);
        }
        None => show_main_menu(data),
    }

    show_main_menu(data)
}

fn join_game(data: &mut Data) {
    let available_games = data
        .get_games()
        .iter()
        .filter(|g| {
            g.result.is_none() && g.player2.is_none() && g.player1.username != data.get_user()
        })
        .enumerate()
        .collect::<BTreeMap<_, _>>();

    if available_games.is_empty() {
        println!("No games available to join.");
        show_main_menu(data);
        return;
    }

    let selections = available_games
        .iter()
        .map(|(num, g)| (num, format!("Id: {}, Player: {}", g.id, g.player1.username)))
        .collect::<BTreeMap<_, _>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a game to join")
        .default(0)
        .items(&selections.values().cloned().collect::<Vec<String>>()[..])
        .interact_opt()
        .unwrap();

    match selection {
        Some(game_num) => {
            let game_num = selections.len() - game_num - 1;
            let game_id = available_games.get(&game_num).unwrap().id;
            let choice_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please enter your choice")
                .default(0)
                .items(Choice::VARIANTS)
                .interact_opt()
                .unwrap();

            match choice_selection {
                Some(choice) => {
                    let choice = Choice::from_repr(choice).unwrap();
                    data.join_game(game_id, choice.clone());

                    println!("Game id {} joined with \"{}\" choice", game_id, choice);
                    show_main_menu(data);
                }
                None => show_main_menu(data),
            }
        }
        None => show_main_menu(data),
    }
}

fn complete_game(data: &mut Data) {
    let available_games = data
        .get_games()
        .iter()
        .filter(|g| {
            g.result.is_none() && g.player1.username == data.get_user() && g.player2.is_some()
        })
        .enumerate()
        .collect::<BTreeMap<_, _>>();

    if available_games.is_empty() {
        println!("No games available to complete yet.");
        show_main_menu(data);
        return;
    }

    let selections = available_games
        .iter()
        .map(|(num, g)| {
            (
                num,
                format!(
                    "Id: {}, Player2: {}",
                    g.id,
                    g.player2.clone().unwrap().username
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a game to join")
        .default(0)
        .items(&selections.values().rev().cloned().collect::<Vec<String>>()[..])
        .interact_opt()
        .unwrap();

    match selection {
        Some(game_num) => {
            println!("Completing game...");

            let game_num = selections.len() - game_num - 1;
            let game_id = available_games.get(&game_num).unwrap().id;

            // Perform checks before completing the game
            {
                let game = data.game_data.get_game(game_id).unwrap();

                // Make sure game is not finished
                if game.result.is_some() {
                    println!("Game with ID {} is already finished", game_id);
                    return show_main_menu(data);
                }

                // Make sure game is not timed out
                if game.timeout.is_some() {
                    let curr_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    if curr_time > game.timeout.unwrap() {
                        // TODO: Handle timeout on chain
                        println!("Game with ID {} has timed out", game_id);
                        return show_main_menu(data);
                    }
                }

                // Make sure that current user is player1
                if game.player1.username != data.get_user() {
                    println!("Current user is not player1");
                    return show_main_menu(data);
                }

                // Make sure we have player2 choice
                if game.player2.is_none() {
                    println!("Player2 has not made a choice yet");
                    return show_main_menu(data);
                }
            }

            // After all checks, we calculate the result
            match data.calculate_game_result(game_id) {
                Ok(result) => {
                    println!("Game with ID {} completed successfully", game_id);
                    match result {
                        GameResult::Player1 => println!("Player1 wins!"),
                        GameResult::Player2 => println!("Player2 wins!"),
                        GameResult::Draw => println!("It's a draw!"),
                    }
                }
                Err(e) => {
                    println!("Error completing game with ID {}: {}", game_id, e);
                }
            }

            return show_main_menu(data);
        }
        None => show_main_menu(data),
    }
}

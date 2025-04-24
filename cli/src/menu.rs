use std::collections::HashMap;

use dialoguer::{theme::ColorfulTheme, Select};
use strum::{Display, EnumString, FromRepr, VariantArray};

use crate::{game::Choice, Data};

#[derive(Debug, PartialEq, Eq, VariantArray, EnumString, Display, FromRepr)]
enum MainMenu {
    #[strum(to_string = "Create new game")]
    CreateGame,
    #[strum(to_string = "Join game")]
    JoinGame,
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
        .filter(|g| g.result.is_none() && g.player2.is_none() && g.player1 != data.get_user())
        .enumerate()
        .collect::<HashMap<_, _>>();

    if available_games.is_empty() {
        println!("No games available to join.");
        show_main_menu(data);
        return;
    }

    let selections = available_games
        .iter()
        .map(|(num, g)| (num, format!("Id: {}, Player: {}", g.id, g.player1)))
        .collect::<HashMap<_, _>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a game to join")
        .default(0)
        .items(&selections.values().cloned().collect::<Vec<String>>()[..])
        .interact_opt()
        .unwrap();

    match selection {
        Some(game_num) => {
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
                }
                None => show_main_menu(data),
            }
        }
        None => show_main_menu(data),
    }
}

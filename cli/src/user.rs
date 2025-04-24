use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use strum::{Display, EnumString, FromRepr, VariantArray};

#[derive(Debug, PartialEq, Eq, VariantArray, EnumString, Display, FromRepr)]
enum UserMenu {
    #[strum(to_string = "Log in")]
    Login,
    #[strum(to_string = "Register")]
    Register,
}

pub(crate) fn handle_user_not_logged_in() -> String {
    let selection = UserMenu::from_repr(
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Not logged in:")
            .default(0)
            .items(UserMenu::VARIANTS)
            .interact()
            .unwrap(),
    );

    match selection {
        Some(UserMenu::Login) => {
            let username = get_user("Username:");
            let password = get_password(false);
            log_in(username, password)
        }
        Some(UserMenu::Register) => {
            let username = get_user("Username:");
            let password = get_password(true);
            register(username, password)
        }
        None => {
            println!("Invalid selection. Please try again.");
            handle_user_not_logged_in()
        }
    }
}

fn get_user(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

fn get_password(with_confirm: bool) -> String {
    let theme = &ColorfulTheme::default();
    let pass_editor = Password::with_theme(theme);
    let pass_editor = pass_editor.with_prompt("Password:");

    let pass_editor = if with_confirm {
        pass_editor.with_confirmation("Repeat password", "Error: the passwords don't match.")
    } else {
        pass_editor
    };

    pass_editor.interact().unwrap()
}

// TODO: Implement the login logic
fn log_in(_user: String, _pass: String) -> String {
    // Implement the login logic here
    println!("Logging in...");

    _user
}

// TODO: Implement the register logic
fn register(_user: String, _pass: String) -> String {
    // Implement the login logic here
    println!("Logging in...");
    _user
}

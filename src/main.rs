mod games;
mod helper;
mod daemons;

use std::{collections::HashMap, sync::mpsc};

use daemons::{input::input_daemon, tui::tui_daemon, robot::robot_daemon};
use games::{siege::Siege, Game};
use anyhow::{ Context, Result };

const DEBUG: bool = false;




#[tokio::main]
async fn main() -> Result<()> {
    if DEBUG {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    /* 
        Game and Message Channel Initialization
     */
    let mut games: HashMap<&str, Box<dyn Game + Send>> = HashMap::new();
    let (input_sender, input_reciever) = mpsc::channel();
    let (game_select_sender, game_select_reciever) = mpsc::channel();

    // Initialize the various games
    games.insert("siege", Box::new(
        Siege::init(input_sender.clone())
            .context("Failed to set up the Siege robot!")?
    ));
    games.insert("apex", Box::new(
        Siege::init(input_sender.clone())
            .context("Failed to set up the Siege robot!")?
    ));
    games.insert("xdefiant", Box::new(
        Siege::init(input_sender.clone())
            .context("Failed to set up the Siege robot!")?
    ));

    // Create an indexable list of the games
    let mut menu_titles: Vec<String> = games.keys()
        .map(|&name| name.to_owned())
        .collect();
    menu_titles.sort();

    /*
        TUI and Input Daemon Initialization
     */
    tokio::spawn(tui_daemon(
        menu_titles,
        input_reciever,
        game_select_sender
    ));
    tokio::spawn(input_daemon(
        input_sender.clone()
    ));
    
    robot_daemon(
        games,
        game_select_reciever,
        input_sender.clone())
        .await?;

    Ok(())
}

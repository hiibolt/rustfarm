use std::{collections::HashMap, sync::mpsc};

use anyhow::{ Context, Result };

use crate::{games::Game, helper::lib::AppEvent, sleep};

pub async fn robot_daemon(
    mut games: HashMap<&str, Box<dyn Game + Send>>,
    game_select_reciever: mpsc::Receiver<String>,
    input_sender: mpsc::Sender<AppEvent>
) -> Result<()> {
    let mut game;
    loop {
        game = game_select_reciever.recv()
            .context("Failed to recieve the selected game!")?;

        // Check for unsupported games
        if game == "apex" || game == "xdefiant" {
            input_sender.send(AppEvent::UpdateStatus(("Error".into(), "This game is not yet supported!".into())))
                .context("Failed to send the status update!")?;
            continue;
        }

        // If we've made it this far, we have a valid game
        break;
    }
    
    input_sender.send(AppEvent::UpdateStatus(("Starting".into(), "Please quickly tab into the game you have selected.".into())))
        .context("Failed to send the status update!")?;

    let game = games.get_mut(&game.as_str())
        .context("Failed to find the game you're looking for!")?;

    // Wait 5 seconds to start the game
    for i in (1..=5).rev() {
        input_sender.send(AppEvent::AddDebug(format!("Starting in {}...", i)))
            .context("Failed to send the debug message!")?;
        sleep!(1000);
    }

    // Start the game
    game.startup()
        .context("Failed to start the game!")?;

    // Run the game loop
    loop {
        game.game_loop()
            .context("Failed to run the game loop!")?;
    }
}
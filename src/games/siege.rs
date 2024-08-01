use crate::{helper::{lib::{is_on_screen, load_images, AppEvent}, rsauto::tap}, sleep};

use std::{collections::HashMap, sync::mpsc};

use rand::{ Rng, prelude::SliceRandom };
use image::DynamicImage;
use anyhow::{ Context, Result };

use super::Game;


const SIEGE_IMAGES: [&str; 4] = [
    "siege/cogs.png",
    "siege/in_game_error.png",
    "siege/in_queue_alt.png",
    "siege/in_queue.png"
];

pub struct Siege {
    pub images:       HashMap<&'static str, DynamicImage>,
    pub input_sender: mpsc::Sender<AppEvent>
}
impl Siege {
    fn press_buttons(
        &self,
        button:           char,
        amount:           u32,
        delay_between:    Option<u64>,
        delay_on_the_end: Option<u64>
    ) {
        for i in 0..amount {
            self.input_sender.send(AppEvent::AddDebug(format!("Pressing {:?} ({}/{})", button, i + 1, amount)))
                .expect("Failed to send the debug message!");
            tap(button);
            if let Some(delay) = delay_between {
                sleep!(delay);
            }
        }
        if let Some(delay) = delay_on_the_end {
            sleep!(delay);
        }
    }
    fn press_button(
        &self,
        button:           char,
        delay_on_the_end: Option<u64>
    ) {
        self.input_sender.send(AppEvent::AddDebug(format!("Pressing {:?}", button)))
            .expect("Failed to send the debug message!");
        tap(button);
        if let Some(delay) = delay_on_the_end {
            sleep!(delay);
        }
    }

    pub fn init(
        input_sender: mpsc::Sender<AppEvent>
    ) -> Result<Siege> {
        let images = load_images(&SIEGE_IMAGES)
            .context("Failed to load Rainbow Six Siege's assets!")?;

        Ok(Siege {
            images,
            input_sender
        })
    }
    fn is_in_queue(&self) -> bool {
        is_on_screen(
            &self.images["siege/in_queue.png"],
            Some(0.85),
            Some(2)
        ) || is_on_screen(
            &self.images["siege/in_queue_alt.png"],
            Some(0.85),
            Some(2)
        )
    }
    fn is_in_menu(&self) -> bool {
        is_on_screen(
            &self.images["siege/cogs.png"],
            Some(0.9),
            Some(10)
        )
    }
    fn is_error(&self) -> bool {
        is_on_screen(
            &self.images["siege/in_game_error.png"],
            Some(1.0),
            Some(1)
        )
    }
}
impl Game for Siege {
    fn startup(&mut self) -> Result<()> {
        // Update status to starting
        self.input_sender.send(AppEvent::UpdateStatus(("Starting".into(), "Starting the game...".into())))
            .context("Failed to send the status update!")?;

        // First, check if you're already in queue
        if self.is_in_queue() {
            self.input_sender.send(AppEvent::UpdateStatus(("In Queue".into(), "Waiting to find a game...".into())))
                .context("Failed to send the status update!")?;

            return Ok(());
        }

        // If you're not in queue, check if you're in the menu
        if self.is_in_menu() {
            // If you're in the menu, start the queue
            self.input_sender.send(AppEvent::UpdateStatus(("Queuing for Game".into(), "Navigating to start a new match...".into())))
                .context("Failed to send the status update!")?;

            // Press the buttons to start the queue
            self.press_buttons('W', 3, Some(500), Some(500));
            self.press_buttons('A', 3, Some(500), Some(500));
            self.press_button('S', Some(500));
            self.press_buttons('D', 3, Some(500), Some(500));
            self.press_buttons('A', 2, Some(500), Some(500));
            self.press_button('\n', Some(500));
            self.press_button('D', Some(500));
            self.press_button('\n', Some(500));
            self.press_button('\n', Some(500));

            // Update the status to reflect the queue
            self.input_sender.send(AppEvent::UpdateStatus(("In Queue".into(), "Waiting to find a game...".into())))
                .context("Failed to send the status update!")?;

            return Ok(());
        }

        // Update the status to reflect the queue
        self.input_sender.send(AppEvent::UpdateStatus(("Unusual".into(), "You weren't in the queue or the menu! Continuing...".into())))
            .context("Failed to send the status update!")?;

        Ok(())
    }
    fn game_loop(&mut self) -> Result<()> {
        // First, check if you're already in queue
        if self.is_in_queue() {
            self.input_sender.send(AppEvent::UpdateStatus(("In Queue".into(), "Waiting to find a game...".into())))
                .context("Failed to send the status update!")?;

            sleep!(5000);

            return Ok(());
        }

        // Secondly, check if you're in the menus
        if self.is_in_menu() {
            // If you're in the menu, start the queue
            self.input_sender.send(AppEvent::UpdateStatus(("Queuing for Game".into(), "Navigating to start a new match...".into())))
                .context("Failed to send the status update!")?;

            // Press the buttons to start the queue
            self.press_buttons('W', 3, Some(500), Some(500));
            self.press_buttons('A', 3, Some(500), Some(500));
            self.press_button('S', Some(500));
            self.press_buttons('D', 3, Some(500), Some(500));
            self.press_buttons('A', 2, Some(500), Some(500));
            self.press_button('\n', Some(500));
            self.press_button('D', Some(500));
            self.press_button('\n', Some(500));
            self.press_button('\n', Some(500));

            // Update the status to reflect the queue
            self.input_sender.send(AppEvent::UpdateStatus(("In Queue".into(), "Waiting to find a game...".into())))
                .context("Failed to send the status update!")?;

            sleep!(30000);

            return Ok(());
        }

        // Thirdly, check if you're in an error screen
        if self.is_error() {
            // Update the status to reflect the error
            self.input_sender.send(AppEvent::UpdateStatus(("Error".into(), "You're in an error screen!".into())))
                .context("Failed to send the status update!")?;

            // Close the error
            self.press_button('\n', Some(1500));

            sleep!(1500);

            return Ok(());
        }

        // Lastly, we're probably in game
        self.input_sender.send(AppEvent::UpdateStatus(("Playing".into(), "You are in game.".into())))
            .context("Failed to send the status update!")?;
        
        let valid_buttons = [
            'W',
            'A',
            'S',
            'D'
        ];
        let button = valid_buttons.choose(&mut rand::thread_rng())
            .unwrap();
        let delay_on_the_end = rand::thread_rng().gen_range(200..900) as u64;

        self.press_button(*button, Some(delay_on_the_end));

        Ok(())
    }
}
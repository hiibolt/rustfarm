use std::{sync::mpsc, time::Duration};

use crossterm::event::{self, Event};
use tokio::time::Instant;

use crate::{Result, helper::lib::AppEvent};


pub async fn input_daemon(
    tx: mpsc::Sender<AppEvent>
) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)
            .expect("Failed to read from poll!")
        {
            if let Event::Key(key) = event::read()
                .expect("Failed to read an event!")
            {
                tx.send(AppEvent::KeyPressed(key))
                    .expect("Failed to send an event!");
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        } 
    }
}
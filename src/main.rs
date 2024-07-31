

use std::{collections::HashMap, sync::mpsc};
use std::time::{Duration, Instant};
use std::thread;
use std::path::Path;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use spectrust::*;
use rand::{ Rng, prelude::SliceRandom };
use image::DynamicImage;
use anyhow::{ Context, Result };
use tui::{backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::{Block, BorderType, Borders, Paragraph, Tabs}, Terminal};


const DEBUG: bool = false;

macro_rules! sleep {
    ($time:expr) => {
        thread::sleep(Duration::from_millis($time));
    };
}



fn tap(
    ch: char
) {
    let virtual_keycode: u16 = match ch {
        'W' => 17u16,
        'A' => 30u16,
        'S' => 31u16,
        'D' => 32u16,
        '\n' => 28u16,
        _ => panic!("Invalid character!")
    };

    // Initial input
    let mut initial_input: winapi::um::winuser::INPUT = Default::default();
    initial_input.type_ = winapi::um::winuser::INPUT_KEYBOARD;
    unsafe { 
        let ki: &mut winapi::um::winuser::KEYBDINPUT = &mut initial_input.u.ki_mut();
        ki.wVk = 0;
        ki.wScan = virtual_keycode;
        ki.dwFlags = 0 | winapi::um::winuser::KEYEVENTF_SCANCODE;
    };

    // Send the input
    unsafe {
        let result_byte = winapi::um::winuser::SendInput(
            1,
            &mut initial_input,
            std::mem::size_of::<winapi::um::winuser::INPUT>() as i32
        );

        if DEBUG {
            println!("Resulting byte from key down: {:?}", result_byte);
        }
    }

    // Sleep for a bit
    sleep!(50);

    // Key up
    let mut key_up: winapi::um::winuser::INPUT = Default::default();
    key_up.type_ = winapi::um::winuser::INPUT_KEYBOARD;
    unsafe { 
        key_up.u.ki_mut().wVk = 0;
        key_up.u.ki_mut().wScan = virtual_keycode;
        key_up.u.ki_mut().dwFlags = 0 | winapi::um::winuser::KEYEVENTF_SCANCODE | winapi::um::winuser::KEYEVENTF_KEYUP;
    };

    // Send the input
    unsafe {
        let result_byte = winapi::um::winuser::SendInput(
            1,
            &mut key_up,
            std::mem::size_of::<winapi::um::winuser::INPUT>() as i32
        );

        if DEBUG {
            println!("Resulting byte from key up: {:?}", result_byte);
        }
    }
}


const SIEGE_IMAGES: [&str; 4] = [
    "siege/cogs.png",
    "siege/in_game_error.png",
    "siege/in_queue_alt.png",
    "siege/in_queue.png"
];

fn load_images<'a>(
    paths: &'a[&'a str]
) -> Result<HashMap<&'a str, DynamicImage>> {
    // Create a new hashmap for the game
    let mut hashmap = HashMap::new();

    // Load each image into the hashmap
    for &path in paths {
        let img = image::open(Path::new("assets").join(path))
            .with_context(|| format!("Unable to locate file: `{path}`"))?;
        hashmap.insert(path, img);
    }

    Ok(hashmap)
}
fn is_on_screen(
    image:      &DynamicImage,
    confidence: Option<f32>,
    tolerance:  Option<u8>
) -> bool {
    let location_of_image = locate_image(
        image,
        None,
        confidence,
        tolerance,
    );

    if DEBUG {
        println!("Location of image: {:?}", location_of_image);
    }
    
    location_of_image.is_some()
}

trait Game 
{
    fn startup(&mut self) -> Result<()>;
    fn game_loop(&mut self) -> Result<()>;
}

struct Siege {
    images:       HashMap<&'static str, DynamicImage>,
    input_sender: mpsc::Sender<AppEvent>
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

    fn init(
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

enum AppEvent {
    KeyPressed(KeyEvent),
    UpdateStatus((String, String)),
    AddDebug(String)
}

async fn input_daemon(
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
async fn tui_daemon(
    mut menu_titles: Vec<String>,
    input_reciever: mpsc::Receiver<AppEvent>,
    game_select_sender: mpsc::Sender<String>
) -> Result<()> {
    // Initialize the terminal
    let stdout = std::io::stdout();
    let mut terminal = Terminal::new(
        CrosstermBackend::new(stdout))
        .context("Failed to initialize the terminal!")?;
    terminal.clear()?;

    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let mut active_menu_item = 0;
    let start_time = Instant::now();

    let mut status = String::from("Inactive");
    let mut status_message = String::from("Please select a game!");
    let mut debug_console: Vec<String> = Vec::new();
    let mut exit = false;
    let mut exit_counter = 4;

    menu_titles.push("exit (esc)".to_string());

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("rustfarm")
                .style(
                    Style::default()
                        .fg(Color::Rgb(255,105,180))
                )
                .borders(Borders::ALL);
            f.render_widget(block, size);
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            /* 
                Header / Menu
             */
            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Rgb(255,105,180)),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .fg(Color::Rgb(255,105,180))
                .          add_modifier(Modifier::UNDERLINED)
                )
                .divider(Span::raw("|"));

            f.render_widget(tabs, chunks[0]);

            /* BODY */
            let console_lines = debug_console
                .iter()
                .map(|line| format!("<= - {}", line))
                .collect::<Vec<String>>()
                .join("\n");
            let body = Paragraph::new(format!("<= Status: {}\n<= Message: {}\n\n<= Debug Console:\n{}", status, status_message, console_lines))
                .style(Style::default().fg(Color::White))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Information")
                        .border_type(BorderType::Plain),
                );

            f.render_widget(body, chunks[1]);

            /* FOOTER */
            let copyright = Paragraph::new("@hiibolt - GNU GENERAL PUBLIC LICENSE")
                .style(Style::default().fg(Color::Rgb(255,105,180)))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            f.render_widget(copyright, chunks[2]);
        })?;

        if exit == true {
            if exit_counter > 0 {
                if exit_counter == 1 {
                    debug_console.push(format!("Stay cozy :3"));
                } else {
                    debug_console.push(format!("Exiting in {}...", exit_counter - 1));
                }
                exit_counter -= 1;

                sleep!(1000);

                continue;
            }

            sleep!(1000);
                    
            // Clear the console
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            // Exit the program
            std::process::exit(0);
        }

        match input_reciever.recv()
            .context("Failed to recieve events!")?
        {
            AppEvent::KeyPressed(key_event) => {
                if key_event.kind == KeyEventKind::Release {
                    match key_event.code {
                        KeyCode::Left => {
                            if active_menu_item > 0 {
                                active_menu_item -= 1;
                            }
                        },
                        KeyCode::Right => {
                            if active_menu_item < menu_titles.len() - 1 {
                                active_menu_item += 1;
                            }
                        },
                        KeyCode::Enter => {
                            // Make sure it's been at least a half second
                            //  since the program started
                            if start_time.elapsed() < Duration::from_secs(1) {
                                continue;
                            }

                            // Check if it was the exit button
                            if active_menu_item == menu_titles.len() - 1 {
                                status = "Exiting".into();
                                status_message = "Have a wonderful day!".into();
                                debug_console = Vec::new();

                                exit = true;
                            }

                            game_select_sender.send(menu_titles[active_menu_item].clone())
                                .context("Failed to send the selected game!")?;
                        },
                        KeyCode::Esc => {
                            status = "Exiting".into();
                            status_message = "Have a wonderful day!".into();
                            debug_console = Vec::new();

                            exit = true;
                        },
                        _ => {}
                    }
                }
            },
            AppEvent::UpdateStatus((new_status, new_message)) => {
                status = new_status;
                status_message = new_message;
                debug_console.clear();
            },
            AppEvent::AddDebug(message) => {
                debug_console.push(message);
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    if DEBUG {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    /* 
        Game and Message Channel Initialization
     */
    let mut games: HashMap<&str, Box<dyn Game>> = HashMap::new();
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
    tokio::spawn(tui_daemon(menu_titles, input_reciever, game_select_sender));
    tokio::spawn(input_daemon(input_sender.clone()));

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

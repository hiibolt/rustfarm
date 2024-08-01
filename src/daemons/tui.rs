use std::sync::mpsc;
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::{event::EnableMouseCapture, execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
use anyhow::{ Context, Result };
use tui::{backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::{Block, BorderType, Borders, Paragraph, Tabs}, Terminal};

use crate::helper::lib::AppEvent;
use crate::sleep;


pub async fn tui_daemon(
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

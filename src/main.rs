use acer_projector_rs::Projector;
use acer_projector_rs::commands::Command;
use acer_projector_rs::enums::{Action, Key};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::env;
use std::process;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <port_name>");
        eprintln!("Example: cargo run /dev/ttyUSB0");
        process::exit(1);
    }

    let port_name = &args[1];
    let baud_rate = 9600;
    let timeout_ms = 500;

    println!(
        "Attempting to connect to projector on {} at {} baud...",
        port_name, baud_rate
    );

    let mut projector = match Projector::connect(port_name, baud_rate, timeout_ms) {
        Ok(p) => {
            println!("Successfully opened serial port: {}", port_name);
            p
        }
        Err(e) => {
            eprintln!("Failed to connect: {:?}", e);
            process::exit(1);
        }
    };

    println!("\n========================================");
    println!("     ACER PROJECTOR KEYBOARD REMOTE     ");
    println!("========================================");
    println!("  ▲ / ▼ / ◀ / ▶   ->  Navigate");
    println!("  Enter           ->  Select / Enter");
    println!("  M               ->  Open OSD Menu");
    println!("  Backspace / Esc ->  Back");
    println!("  O / P           ->  Zoom In / Out");
    println!("  Q               ->  Quit Remote");
    println!("========================================\n");

    if let Err(e) = enable_raw_mode() {
        eprintln!("Failed to enable terminal raw mode: {:?}", e);
        process::exit(1);
    }

    loop {
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.kind == KeyEventKind::Release {
                    continue;
                }

                let projector_command = match key_event.code {
                    KeyCode::Up => Some(Command::Press(Key::Up)),
                    KeyCode::Down => Some(Command::Press(Key::Down)),
                    KeyCode::Left => Some(Command::Press(Key::Left)),
                    KeyCode::Right => Some(Command::Press(Key::Right)),
                    KeyCode::Enter => Some(Command::Press(Key::Enter)),
                    KeyCode::Char('m') | KeyCode::Char('M') => Some(Command::Press(Key::Menu)),
                    KeyCode::Esc | KeyCode::Backspace => Some(Command::Press(Key::Back)),
                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        Some(Command::Toggle(Action::ZoomIn))
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        Some(Command::Toggle(Action::ZoomOut))
                    }

                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        break;
                    }
                    _ => None,
                };

                if let Some(command) = projector_command {
                    let _ = disable_raw_mode();
                    println!("\rSending command: {}", command);

                    match projector.send_command(command) {
                        Ok(_) => println!("Command sent successfully!"),
                        Err(e) => println!("Error: {:?}", e),
                    }
                    if let Err(e) = enable_raw_mode() {
                        eprintln!("Failed to re-enable terminal raw mode: {:?}", e);
                        break;
                    }
                }
            }
        }
    }

    let _ = disable_raw_mode();
    println!("\rDisconnected from remote interface.");
}

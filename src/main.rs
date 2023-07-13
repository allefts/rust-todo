use std::{
    io::{stdout, Stdout},
    process::exit,
};

use crossterm::{
    cursor::{self, MoveDown, MoveUp},
    event::{read, Event},
    execute,
    style::Stylize,
    terminal::{self},
};

fn main() {
    let mut stdout = stdout();

    prepare_terminal(&mut stdout);
    display_menu(&mut stdout);

    read().expect("Placeholder read");

    loop {
        match read().expect("Could not read") {
            Event::Key(event) => match event.code {
                crossterm::event::KeyCode::Up => {
                    execute!(stdout, MoveUp(1)).expect("Could not move cursor up");
                }
                crossterm::event::KeyCode::Down => {
                    execute!(stdout, MoveDown(1)).expect("Could not move cursor down");
                }
                crossterm::event::KeyCode::Enter => clear_terminal(&mut stdout),
                crossterm::event::KeyCode::Char('q') => exit(0),
                _ => {}
            },
            _ => {}
        }
    }
}

fn display_menu(stdout: &mut Stdout) {
    println!("{}", "Todo list built with Rust".blue());
    println!("{}", "> Add a todo\n".grey());
    println!("{}", "> View todos\n".grey());
    println!("{}", "> Remove a todo\n".grey());
    println!("{}", "> (q)Exit".grey());
    execute!(stdout, cursor::MoveTo(0, 1)).expect("Could not move cursor");
}

fn prepare_terminal(stdout: &mut Stdout) {
    clear_terminal(stdout);
    execute!(stdout, crossterm::cursor::SetCursorStyle::BlinkingBlock)
        .expect("Error setting cursor style");

    execute!(stdout, cursor::MoveTo(0, 0)).expect("Could not move cursor");
}

fn clear_terminal(stdout: &mut Stdout) {
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).expect("Could not clear terminal");
}

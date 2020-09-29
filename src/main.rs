use crate::tui_util::{App, Event, Events};
use std::error::Error;
use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

mod header_widget;
mod section_widget;
mod tui_util;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: ./peachelf <file-path>");
        std::process::exit(1);
    }

    let elf_file = elf_utilities::parser::read_elf64(&args[1])?;
    let events = Events::new();

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application initialization
    let mut app = App::new(&elf_file);
    app.items.borrow_mut().previous();
    app.items.borrow_mut().previous();

    // Main loop
    loop {
        terminal.draw(|f| app.draw(f, args[1].to_string(), &elf_file))?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Up => app.items.borrow_mut().next(),
                Key::Down => app.items.borrow_mut().previous(),
                _ => {}
            }
        }
    }
    Ok(())
}

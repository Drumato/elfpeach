use crate::tui_util::{App, AppState, Event, Events};
use std::error::Error;
use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

mod tui_util;
mod widgets;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: ./elfpeach <file-path>");
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

    // Main loop
    loop {
        terminal.draw(|f| app.draw(f, args[1].to_string(), &elf_file))?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') | Key::Esc => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                Key::Up => match app.state() {
                    AppState::Header => {}
                    AppState::Section => app.sections.borrow_mut().previous(),
                    AppState::Segment => app.segments.borrow_mut().previous(),
                    AppState::Symbol => app.symbol_table.borrow_mut().previous(),
                    AppState::DynSym => app.dynamic_symbol_table.borrow_mut().previous(),
                    AppState::Dynamics => app.dynamic_table.borrow_mut().previous(),
                },
                Key::Down => match app.state() {
                    AppState::Header => {}
                    AppState::Section => app.sections.borrow_mut().next(),
                    AppState::Segment => app.segments.borrow_mut().next(),
                    AppState::Symbol => app.symbol_table.borrow_mut().next(),
                    AppState::DynSym => app.dynamic_symbol_table.borrow_mut().next(),
                    AppState::Dynamics => app.dynamic_table.borrow_mut().next(),
                },
                _ => {}
            }
        }
    }
    Ok(())
}

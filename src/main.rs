mod header_widget;
mod section_widget;
mod tui_util;

use crate::tui_util::{Event, StatefulList};
use elf_utilities::file;
use std::cell::RefCell;
use std::{error::Error, io};
use termion::raw::RawTerminal;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Terminal,
};
use tui_util::{Events, TabsState};

struct App<'a> {
    tabs: TabsState<'a>,
    items: RefCell<StatefulList<&'a str>>,
    terminal:
        RefCell<Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<io::Stdout>>>>>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: ./peachelf <file-path>");
        std::process::exit(1);
    }

    let elf_file = elf_utilities::parser::read_elf64(&args[1])?;

    // Terminal initialization
    let events = Events::new();

    // Application initialization
    let mut app = initialize_application(&elf_file)?;
    app.items.borrow_mut().previous();
    app.items.borrow_mut().previous();

    // Main loop
    loop {
        app.terminal.borrow_mut().draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
            f.render_widget(block, size);

            let titles = app
                .tabs
                .titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(first, Style::default().fg(Color::Yellow)),
                        Span::styled(rest, Style::default().fg(Color::Green)),
                    ])
                })
                .collect();
            let tabs = Tabs::new(titles)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(args[1].to_string()),
                )
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);

            match app.tabs.index {
                0 => {
                    let inner = header_widget::header_paragraphs(&elf_file);
                    f.render_widget(inner, chunks[1]);
                }
                1 => {
                    let inner = section_widget::section_list(&elf_file);
                    f.render_stateful_widget(inner, chunks[1], &mut app.items.borrow_mut().state);
                }
                _ => unreachable!(),
            }
        })?;

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

fn initialize_application(elf_file: &file::ELF64) -> Result<App, Box<dyn std::error::Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(App {
        tabs: TabsState::new(vec!["Header", "Sections", "Segments"]),
        items: RefCell::new(StatefulList::with_items(section_widget::section_names(
            elf_file,
        ))),
        terminal: RefCell::new(terminal),
    })
}

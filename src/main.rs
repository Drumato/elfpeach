mod tui_util;
mod header_widget;

use tui_util::{Events, TabsState};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Terminal,
};
use crate::tui_util::Event;
use tui::widgets::Paragraph;

struct App<'a> {
    tabs: TabsState<'a>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: ./peachelf <file-path>");
        std::process::exit(1);
    }

    let elf_file = elf_utilities::parser::read_elf64(&args[1])?;

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // App
    let mut app = App {
        tabs: TabsState::new(vec!["Header", "Sections", "Segments"]),
    };

    // Main loop
    loop {
        terminal.draw(|f| {
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
                .block(Block::default().borders(Borders::ALL).title(args[1].to_string()))
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Black),
                );
            f.render_widget(tabs, chunks[0]);

            let inner = match app.tabs.index {
                0 => header_widget::header_paragraph(&elf_file),
                1 => Paragraph::new(vec![]).block(Block::default().title("Sections").borders(Borders::ALL)),
                2 => Paragraph::new(vec![]).block(Block::default().title("Segments").borders(Borders::ALL)),
                _ => unreachable!(),
            };
            f.render_widget(inner, chunks[1]);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            }
        }
    }
    Ok(())
}


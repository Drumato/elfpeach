use tui::layout::Corner;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

pub fn list<'a>(title: &'a str, items: Vec<ListItem<'a>>) -> List<'a> {
    List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .start_corner(Corner::TopLeft)
}

use elf_utilities::file;
use tui::layout::Corner;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};

pub fn section_list(elf_file: &file::ELF64) -> List {
    let items = section_items(elf_file);

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Sections"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .start_corner(Corner::BottomLeft)
}

fn section_items(elf_file: &file::ELF64) -> Vec<ListItem> {
    section_names(elf_file)
        .iter()
        .map(|name| ListItem::new(vec![Spans::from(vec![Span::raw(*name)])]))
        .collect()
}
pub fn section_names<'a>(elf_file: &'a file::ELF64) -> Vec<&'a str> {
    elf_file
        .sections
        .iter()
        .rev()
        .map(|sct| sct.name.as_str())
        .collect()
}

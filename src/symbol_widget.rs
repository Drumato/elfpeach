use elf_utilities::{section, file, symbol};
use tui::widgets::{Block, List, Borders, ListItem, Paragraph};
use tui::style::{Style, Color, Modifier};
use tui::layout::Corner;
use tui::text::{Span, Spans};

pub fn symbol_table_list(elf_file: &file::ELF64) -> List {
    let items : Vec<ListItem> = symbol_names(elf_file).iter()
        .map(|name| ListItem::new(
            vec![
                Spans::from(vec![Span::raw(name.to_string())])
            ])).collect();

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Symbols"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .start_corner(Corner::TopLeft)
}

pub fn symbol_information(
    elf_file: & file::ELF64,
    sym_idx: usize,
) -> Paragraph {
    let symbol_tables: Vec<&section::Section64> = elf_file.sections.iter()
        .filter(|sct| sct.header.get_type() == section::TYPE::SYMTAB)
        .collect();
    if symbol_tables.is_empty(){
       return Paragraph::new(vec![
            Spans::from(vec![Span::raw("Cannot get any symbol information")]),
        ]).block(Block::default().borders(Borders::ALL).title("Symbols"));
    }
    let sym = &symbol_tables[0].symbols.as_ref().unwrap()[sym_idx];

    Paragraph::new(vec![
        Spans::from(vec![Span::raw("Name: "), Span::raw(sym.symbol_name.as_ref().unwrap().to_string())]),
        Spans::from(vec![Span::raw("Value: "), Span::raw(format!("0x{:x}", sym.st_value))]),
        Spans::from(vec![Span::raw("Size: "), Span::raw(format!("{} (bytes)", sym.st_size))]),
        Spans::from(vec![Span::raw("Type: "), Span::raw(sym_type_string(sym.get_type()))]),
        Spans::from(vec![Span::raw("Bind: "), Span::raw(sym_bind_string(sym.get_bind()))]),
        Spans::from(vec![Span::raw("Visibility: "), Span::raw(sym_vis_string(sym.get_visibility()))]),
        Spans::from(vec![Span::raw("Section: "), Span::raw(sym_ndx_string(elf_file, sym.st_shndx))]),
    ]).block(Block::default().borders(Borders::ALL).title("Symbols"))
}

pub fn symbol_names(elf_file: &file::ELF64) -> Vec<String> {
    let symbol_tables: Vec<&section::Section64> = elf_file.sections.iter()
        .filter(|sct| sct.header.get_type() == section::TYPE::SYMTAB)
        .collect();
    if symbol_tables.is_empty(){
        return vec!["Symbol table Not Found".to_string()];
    }

   symbol_tables[0].symbols.as_ref().unwrap().iter().enumerate().map(|(i, sym)| {
        let name = sym.symbol_name.as_ref().unwrap().to_string();
        if name.is_empty(){
            format!("NO NAME SYMBOL[{}]", i)
        } else {
            name
        }
    }).collect()

}

fn sym_type_string<'a>(sym_type: symbol::TYPE) -> &'a str {
    match sym_type {
        symbol::TYPE::NOTYPE => "NOTYPE",
        symbol::TYPE::OBJECT => "OBJECT",
        symbol::TYPE::FUNC => "FUNC",
        symbol::TYPE::SECTION => "SECTION",
        symbol::TYPE::NUM => "NUM",
        symbol::TYPE::COMMON => "COMMON",
        symbol::TYPE::FILE => "FILE",
        symbol::TYPE::TLS => "TLS",
        _ => "unknown",
    }
}
fn sym_bind_string<'a>(sym_bind: symbol::BIND) -> &'a str {
    match sym_bind {
        symbol::BIND::LOCAL => "LOCAL",
        symbol::BIND::GLOBAL => "GLOBAL",
        symbol::BIND::WEAK => "WEAK",
        symbol::BIND::NUM => "NUM",
        symbol::BIND::GNUUNIQUE => "UNIQUE",
        _ => "unknown",
    }
}
fn sym_vis_string<'a>(sym_vis: symbol::VISIBILITY) -> &'a str {
    match sym_vis {
        symbol::VISIBILITY::DEFAULT => "DEFAULT",
        symbol::VISIBILITY::HIDDEN => "HIDDEN",
        symbol::VISIBILITY::INTERNAL => "INTERNAL",
        symbol::VISIBILITY::PROTECTED => "PROTECTED",
        _ => "unknown",
    }
}

fn sym_ndx_string<'a>(elf_file: &'a file::ELF64, ndx: u16) -> String {
    match ndx{
        section::SHN_UNDEF => "UND".to_string(),
        section::SHN_ABS => "ABS".to_string(),
        section::SHN_COMMON => "COMMON".to_string(),
        section::SHN_XINDEX => "XINDEX".to_string(),
        _ => elf_file.sections[ndx as usize].name.to_string(),
        //
    }
}

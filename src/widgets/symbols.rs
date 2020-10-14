use crate::widgets::list;
use elf_utilities::{file, section, symbol};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn symbol_table_list(symbol_table: Option<&section::Section64>) -> List {
    list(
        "Symbols",
        symbol_names(symbol_table)
            .iter()
            .map(|name| ListItem::new(vec![Spans::from(vec![Span::raw(name.to_string())])]))
            .collect(),
    )
}

pub fn symbol_information<'a>(
    elf_file: &'a file::ELF64,
    symbol_table: &'a section::Section64,
    sym_idx: usize,
) -> Paragraph<'a> {
    let sym = &symbol_table.symbols.as_ref().unwrap()[sym_idx];

    Paragraph::new(vec![
        Spans::from(vec![
            Span::raw("Name: "),
            Span::raw(sym.symbol_name.as_ref().unwrap().to_string()),
        ]),
        Spans::from(vec![
            Span::raw("Value: "),
            Span::raw(format!("0x{:x}", sym.st_value)),
        ]),
        Spans::from(vec![
            Span::raw("Size: "),
            Span::raw(format!("{} (bytes)", sym.st_size)),
        ]),
        Spans::from(vec![
            Span::raw("Type: "),
            Span::raw(sym_type_string(sym.get_type())),
        ]),
        Spans::from(vec![
            Span::raw("Bind: "),
            Span::raw(sym_bind_string(sym.get_bind())),
        ]),
        Spans::from(vec![
            Span::raw("Visibility: "),
            Span::raw(sym_vis_string(sym.get_visibility())),
        ]),
        Spans::from(vec![
            Span::raw("Section: "),
            Span::raw(sym_ndx_string(elf_file, sym.st_shndx)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Symbols"))
}

pub fn symbol_names(symbol_table: Option<&section::Section64>) -> Vec<String> {
    if symbol_table.is_none() {
        return Vec::new();
    }

    symbol_table
        .as_ref()
        .unwrap()
        .symbols
        .as_ref()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, sym)| {
            let name = sym.symbol_name.as_ref().unwrap().to_string();
            if name.is_empty() {
                format!("NO NAME SYMBOL[{}]", i)
            } else {
                name
            }
        })
        .collect()
}

fn sym_type_string<'a>(sym_type: symbol::Type) -> &'a str {
    match sym_type {
        symbol::Type::NoType => "NOTYPE",
        symbol::Type::Object => "OBJECT",
        symbol::Type::Func => "FUNC",
        symbol::Type::Section => "SECTION",
        symbol::Type::Num => "NUM",
        symbol::Type::Common => "COMMON",
        symbol::Type::File => "FILE",
        symbol::Type::TLS => "TLS",
        _ => "unknown",
    }
}
fn sym_bind_string<'a>(sym_bind: symbol::Bind) -> &'a str {
    match sym_bind {
        symbol::Bind::Local => "LOCAL",
        symbol::Bind::Global => "GLOBAL",
        symbol::Bind::Weak => "WEAK",
        symbol::Bind::Num => "NUM",
        symbol::Bind::GNUUnique => "UNIQUE",
        _ => "unknown",
    }
}
fn sym_vis_string<'a>(sym_vis: symbol::Visibility) -> &'a str {
    match sym_vis {
        symbol::Visibility::Default => "DEFAULT",
        symbol::Visibility::Hidden => "HIDDEN",
        symbol::Visibility::Internal => "INTERNAL",
        symbol::Visibility::Protected => "PROTECTED",
        _ => "unknown",
    }
}

fn sym_ndx_string(elf_file: &file::ELF64, ndx: u16) -> String {
    match ndx {
        section::SHN_UNDEF => "UND".to_string(),
        section::SHN_ABS => "ABS".to_string(),
        section::SHN_COMMON => "COMMON".to_string(),
        section::SHN_XINDEX => "XINDEX".to_string(),
        _ => elf_file.sections[ndx as usize].name.to_string(),
    }
}

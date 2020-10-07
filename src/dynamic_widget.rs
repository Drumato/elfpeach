use elf_utilities::{section, file,dynamic};
use tui::widgets::{List, Block, Borders, ListItem, Paragraph};
use tui::style::{Style, Color, Modifier};
use tui::layout::Corner;
use tui::text::{Spans, Span};

pub fn dynamic_list(dynamic_sct: Option<&section::Section64>) -> List {
    let items: Vec<ListItem> = dynamic_names(dynamic_sct).iter()
        .map(|name| ListItem::new(
            vec![
                Spans::from(vec![Span::raw(name.to_string())])
            ])).collect();

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Dynamics"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .start_corner(Corner::TopLeft)
}


pub fn dynamic_information<'a>(
    _elf_file: &'a file::ELF64,
    dynamic_table: &'a section::Section64,
    dyn_idx: usize,
) -> Paragraph<'a> {
    let dyn_entry = &dynamic_table.dynamics.as_ref().unwrap()[dyn_idx];

    Paragraph::new(vec![
        Spans::from(vec![Span::raw("Tag: "), Span::raw(format!("0x{:x}", dyn_entry.d_tag))]),
        Spans::from(vec![Span::raw("Type: "), Span::raw(dyn_type_string(dyn_entry.get_type()))]),
    ]).block(Block::default().borders(Borders::ALL).title("Dynamics"))
}

pub fn dynamic_names(dynamic_sct: Option<&section::Section64>) -> Vec<String> {
    if dynamic_sct.is_none(){
        return Vec::new();
    }

    (0..dynamic_sct.as_ref().unwrap().dynamics.as_ref().unwrap().len()).map(|idx| idx.to_string()).collect()
}

fn dyn_type_string<'a>(dyn_type: dynamic::EntryType) -> &'a str {
    match dyn_type {
        dynamic::EntryType::Null => "NULL",
        dynamic::EntryType::Needed => "NEEDED",
        dynamic::EntryType::PLTRelSz => "PLTRELSZ",
        dynamic::EntryType::PLTRel => "PLTREL",
        dynamic::EntryType::SymTabShNdx => "SYMTAB_SHNDX",
        dynamic::EntryType::PreInitArray =>"PREINIT_ARRAY",
        dynamic::EntryType::InitArray => "INIT_ARRAY",
        dynamic::EntryType::FiniArray => "FINI_ARRAY",
        dynamic::EntryType::Init => "INIT",
        dynamic::EntryType::Fini => "FINI",
        dynamic::EntryType::InitArraySz => "INIT_ARRAYSZ",
        dynamic::EntryType::FiniArraySz => "FINI_ARRAYSZ",
        dynamic::EntryType::PreInitArraySz => "PREINIT_ARRAYSZ",
        dynamic::EntryType::Rel => "REL",
        dynamic::EntryType::Rela => "RELA",
        dynamic::EntryType::RelCount => "RELCOUNT",
        dynamic::EntryType::RelSz => "RELSZ",
        dynamic::EntryType::RelaSz => "RELASZ",
        dynamic::EntryType::RelEnt => "RELENT",
        dynamic::EntryType::RelaEnt => "RELAENT",
        dynamic::EntryType::RelaCount => "RELACOUNT",
        dynamic::EntryType::GNUHash => "GNU_HASH",
        dynamic::EntryType::StrTab => "STRTAB",
        dynamic::EntryType::VerNeed => "VERNEED",
        dynamic::EntryType::VerNeedNum => "VERNEED_NUM",
        dynamic::EntryType::VerSym => "VERSYM",
        dynamic::EntryType::PLTGOT => "PLTGOT",
        dynamic::EntryType::Debug => "DEBUG",
        dynamic::EntryType::TextRel => "TEXTREL",
        dynamic::EntryType::JmpRel => "JMPREL",
        dynamic::EntryType::BindNow => "BINDNOW",
        dynamic::EntryType::RPath => "RPATH(deprecated)",
        dynamic::EntryType::RunPath => "RUNPATH",
        dynamic::EntryType::Symbolic => "SYMBOLIC",
        dynamic::EntryType::SOName => "SONAME",
        dynamic::EntryType::SymTab => "SYMTAB",
        dynamic::EntryType::StrSz => "STRSZ",
        dynamic::EntryType::SymEnt => "SYMENT",
        dynamic::EntryType::Flags => "FLAGS",
        dynamic::EntryType::Flags1 => "FLAGS_1",

        _ => "unknown",
    }
}

use crate::widgets::list;
use elf_utilities::{dynamic, file, section, symbol};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn dynamic_list(dynamic_sct: Option<&section::Section64>) -> List {
    list(
        "Dynamics",
        dynamic_names(dynamic_sct)
            .iter()
            .map(|name| ListItem::new(vec![Spans::from(vec![Span::raw(name.to_string())])]))
            .collect(),
    )
}

pub fn dynamic_information<'a>(
    elf_file: &'a file::ELF64,
    dynamic_table: &'a section::Section64,
    symbol_table: &'a section::Section64,
    dyn_idx: usize,
) -> Paragraph<'a> {
    let dyn_entry = &dynamic_table.dynamics.as_ref().unwrap()[dyn_idx];

    Paragraph::new(vec![
        Spans::from(vec![
            Span::raw("Tag: "),
            Span::raw(format!("0x{:x}", dyn_entry.d_tag)),
        ]),
        Spans::from(vec![
            Span::raw("Type: "),
            Span::raw(dyn_type_string(dyn_entry.get_type())),
        ]),
        dyn_value_spans(
            elf_file,
            dynamic_table,
            symbol_table,
            dyn_entry.get_type(),
            dyn_entry.d_un,
        ),
    ])
    .block(Block::default().borders(Borders::ALL).title("Dynamics"))
}

pub fn dynamic_names(dynamic_sct: Option<&section::Section64>) -> Vec<String> {
    if dynamic_sct.is_none() {
        return Vec::new();
    }

    (0..dynamic_sct
        .as_ref()
        .unwrap()
        .dynamics
        .as_ref()
        .unwrap()
        .len())
        .map(|idx| idx.to_string())
        .collect()
}
fn dyn_type_string<'a>(dyn_type: dynamic::EntryType) -> &'a str {
    match dyn_type {
        dynamic::EntryType::Null => "NULL",
        dynamic::EntryType::Needed => "NEEDED",
        dynamic::EntryType::PLTRelSz => "PLTRELSZ",
        dynamic::EntryType::PLTRel => "PLTREL",
        dynamic::EntryType::SymTabShNdx => "SYMTAB_SHNDX",
        dynamic::EntryType::PreInitArray => "PREINIT_ARRAY",
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

fn dyn_value_spans<'a>(
    elf_file: &'a file::ELF64,
    dynamic_table: &'a section::Section64,
    symbol_table: &'a section::Section64,
    dyn_type: dynamic::EntryType,
    value: u64,
) -> Spans<'a> {
    let (attribute, value_string) = match dyn_type {
        dynamic::EntryType::Needed => (
            "Needed: ",
            dyn_library_string(elf_file, dynamic_table, value),
        ),
        dynamic::EntryType::VerNeed
        | dynamic::EntryType::StrTab
        | dynamic::EntryType::SymTab
        | dynamic::EntryType::VerSym
        | dynamic::EntryType::PLTGOT
        | dynamic::EntryType::Rela
        | dynamic::EntryType::Rel
        | dynamic::EntryType::JmpRel
        | dynamic::EntryType::GNUHash => {
            ("Related Section: ", find_section_by_value(elf_file, value))
        }

        dynamic::EntryType::InitArray | dynamic::EntryType::FiniArray => {
            ("Related Symbol: ", find_array_by_value(symbol_table, value))
        }
        dynamic::EntryType::Init | dynamic::EntryType::Fini => (
            "Related Symbol: ",
            find_symbol_by_value(symbol_table, value),
        ),
        dynamic::EntryType::Flags => (
            "Flag: ",
            dyn_flag_string(dynamic::Flag::from_def(value)).to_string(),
        ),
        dynamic::EntryType::Flags1 => ("Flag1:", dyn_flag1_string(value)),
        dynamic::EntryType::InitArraySz
        | dynamic::EntryType::FiniArraySz
        | dynamic::EntryType::PLTRelSz
        | dynamic::EntryType::RelaSz
        | dynamic::EntryType::StrSz
        | dynamic::EntryType::SymEnt
        | dynamic::EntryType::RelaEnt
        | dynamic::EntryType::RelEnt => ("Size: ", format!("{} (bytes)", value)),
        _ => ("Address: ", format!("0x{:x}", value)),
    };

    Spans::from(vec![Span::raw(attribute), Span::raw(value_string)])
}

fn find_symbol_by_value(symbol_table: &section::Section64, value: u64) -> String {
    for sym in symbol_table.symbols.as_ref().unwrap().iter() {
        if sym.st_value == value && sym.get_type() == symbol::Type::Func {
            return sym.symbol_name.as_ref().unwrap().to_string();
        }
    }

    String::from("unknown")
}
fn find_array_by_value(symbol_table: &section::Section64, value: u64) -> String {
    for sym in symbol_table.symbols.as_ref().unwrap().iter() {
        if sym.st_value == value && sym.get_type() == symbol::Type::NoType {
            return sym.symbol_name.as_ref().unwrap().to_string();
        }
    }

    String::from("unknown")
}
fn find_section_by_value(elf_file: &file::ELF64, value: u64) -> String {
    for section in elf_file.sections.iter() {
        if section.header.sh_addr == value {
            return section.name.to_string();
        }
    }
    String::from("unknown")
}

fn dyn_flag_string<'a>(flag: dynamic::Flag) -> &'a str {
    match flag {
        dynamic::Flag::Origin => "ORIGIN",
        dynamic::Flag::Symbolic => "SYMBOLIC",
        dynamic::Flag::TextRel => "TEXTREL",
        dynamic::Flag::BindNow => "BIND_NOW",
        dynamic::Flag::StaticTLS => "STATIC_TLS",
        dynamic::Flag::NoCommon1 => "NOCOMMON",
        dynamic::Flag::WeakFilter1 => "WEAK_FILTER",
        dynamic::Flag::KMod1 => "KMOD",
        dynamic::Flag::PIE1 => "PIE",
        dynamic::Flag::Stub1 => "STUB",
        dynamic::Flag::Singleton1 => "SINGLETON",
        dynamic::Flag::GlobalAudit1 => "GLOBAL_AUDIT",
        dynamic::Flag::NoReloc1 => "NO_RELOC",
        dynamic::Flag::Edited1 => "EDITED",
        dynamic::Flag::NoHdr1 => "NO_HEADER",
        dynamic::Flag::NokSyms1 => "NOK_SYM",
        dynamic::Flag::IGNMulDef1 => "IGN_MUL_DEF",
        dynamic::Flag::SymInterpose1 => "SYM_INTERPOSE",
        dynamic::Flag::NoDirect1 => "NO_DIRECT",
        dynamic::Flag::EndFiltee1 => "END_FILTEE",
        dynamic::Flag::ConfAlt1 => "CONFIG_ALT",
        dynamic::Flag::NoDefLib1 => "NO_DEFAULT_LIB",
        dynamic::Flag::NoDump1 => "NO_DUMP",
        dynamic::Flag::Trans1 => "TRANS",
        dynamic::Flag::Origin1 => "ORIGIN",
        dynamic::Flag::Global1 => "GLOBAL",
        dynamic::Flag::Now1 => "NOW",
        _ => "unknown",
    }
}

fn dyn_flag1_string(value: u64) -> String {
    let mut base = 0x1;
    let mut s = String::new();

    loop {
        if base == 0 {
            break;
        }

        let f = base & value;
        if f != 0 {
            s += &format!(" {}", dyn_flag_string(dynamic::Flag::from_1(f)));
        }
        base = base << 1;
    }

    s
}
fn dyn_library_string<'a>(
    elf_file: &'a file::ELF64,
    dynamic_table: &'a section::Section64,
    value: u64,
) -> String {
    let table_index = dynamic_table.header.sh_link;
    let strtab = &elf_file.sections[table_index as usize];
    let strtab = strtab.bytes.as_ref().unwrap();

    let library_name = strtab[value as usize..]
        .to_vec()
        .iter()
        .take_while(|byte| **byte != 0x00)
        .map(|byte| *byte)
        .collect();

    String::from_utf8(library_name).unwrap()
}

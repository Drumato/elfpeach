use elf_utilities::{file, section};
use tui::layout::Corner;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn section_list(elf_file: &file::ELF64) -> List {
    let items = section_items(elf_file);

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Sections"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .start_corner(Corner::TopLeft)
}

pub fn section_names(elf_file: &file::ELF64) -> Vec<String> {
    let names=  elf_file
        .sections
        .iter()
        .enumerate()
        .map(|(i, sct)|
            if sct.name.is_empty(){
                format!("NO NAME SECTION{}", i)
            } else {
                sct.name.to_string()
            }
        ).collect();

    names
}

pub fn section_information<'a>(
    elf_file: &'a file::ELF64,
    sct: &'a section::Section64,
) -> Paragraph<'a> {
    let sct_info = match sct.header.get_type() {
        section::TYPE::DYNAMIC => dynamic_info(elf_file, sct),
        section::TYPE::HASH | section::TYPE::SYMTABSHNDX => hash_info(elf_file, sct),
        section::TYPE::SYMTAB | section::TYPE::DYNSYM => symtab_info(elf_file, sct),
        section::TYPE::GROUP => group_info(elf_file, sct),
        section::TYPE::REL | section::TYPE::RELA => relocation_info(elf_file, sct),
        _ => common_section_info(sct),
    };

    Paragraph::new(sct_info).block(Block::default().borders(Borders::ALL).title("Sections"))
}

fn common_section_info(sct: &section::Section64) -> Vec<Spans> {
    vec![
        Spans::from(vec![Span::raw("Name: "), Span::raw(&sct.name)]),
        section_attribute_spans("Type", sct_type_string, sct.header.get_type()),
        Spans::from(vec![
            Span::raw("Address: "),
            Span::raw(format!("0x{:x}", sct.header.sh_addr)),
        ]),
        Spans::from(vec![
            Span::raw("Offset: "),
            Span::raw(format!("0x{:x}", sct.header.sh_offset)),
        ]),
        Spans::from(vec![
            Span::raw("Size: "),
            Span::raw(format!("0x{:x}", sct.header.sh_size)),
        ]),
        Spans::from(vec![
            Span::raw("Entry Size: "),
            Span::raw(format!("0x{:x}", sct.header.sh_entsize)),
        ]),
        Spans::from(vec![
            Span::raw("Flags: "),
            Span::raw(sct_flag_string(sct.header.sh_flags)),
        ]),
        Spans::from(vec![
            Span::raw("Link: "),
            Span::raw(format!("{}", sct.header.sh_link)),
        ]),
        Spans::from(vec![
            Span::raw("Info: "),
            Span::raw(format!("{}", sct.header.sh_info)),
        ]),
        Spans::from(vec![
            Span::raw("Align: "),
            Span::raw(format!("{}", sct.header.sh_addralign)),
        ]),
    ]
}

fn symtab_info<'a>(elf_file: &'a file::ELF64, sct: &'a section::Section64) -> Vec<Spans<'a>> {
    let mut base_info = common_section_info(sct);
    let first_sym_name = get_first_globsym_name_from_sh_info(sct);

    let strtab_sct = &elf_file.sections[sct.header.sh_link as usize];

    base_info.append(&mut vec![
        Spans::from(vec![
            Span::raw("First Global Symbol(from sh_info): "),
            Span::raw(first_sym_name),
        ]),
        Spans::from(vec![
            Span::raw("Related String Table(from sh_link): "),
            Span::raw(&strtab_sct.name),
        ]),
    ]);

    base_info
}
fn dynamic_info<'a>(elf_file: &'a file::ELF64, sct: &'a section::Section64) -> Vec<Spans<'a>> {
    let mut base_info = common_section_info(sct);
    let strtab_sct = &elf_file.sections[sct.header.sh_link as usize];

    base_info.push(Spans::from(vec![
        Span::raw("Related String Table(from sh_link): "),
        Span::raw(&strtab_sct.name),
    ]));

    base_info
}
fn hash_info<'a>(elf_file: &'a file::ELF64, sct: &'a section::Section64) -> Vec<Spans<'a>> {
    let mut base_info = common_section_info(sct);
    let symtab_sct = &elf_file.sections[sct.header.sh_link as usize];

    base_info.push(Spans::from(vec![
        Span::raw("Related Symbol Table(from sh_link): "),
        Span::raw(&symtab_sct.name),
    ]));

    base_info
}
fn relocation_info<'a>(elf_file: &'a file::ELF64, sct: &'a section::Section64) -> Vec<Spans<'a>> {
    let mut base_info = common_section_info(sct);
    let symtab_sct = &elf_file.sections[sct.header.sh_link as usize];
    let reloc_sct = &elf_file.sections[sct.header.sh_info as usize];

    base_info.push(Spans::from(vec![
        Span::raw("Related Symbol Table(from sh_link): "),
        Span::raw(&symtab_sct.name),
    ]));

    if sct.header.sh_flags & section::SHF_INFO_LINK != 0 {
        base_info.push(Spans::from(vec![
            Span::raw("Relocation Target Section (from sh_info): "),
            Span::raw(&reloc_sct.name),
        ]));
    }

    base_info
}
fn group_info<'a>(elf_file: &'a file::ELF64, sct: &'a section::Section64) -> Vec<Spans<'a>> {
    let mut base_info = common_section_info(sct);
    let symtab_sct = &elf_file.sections[sct.header.sh_link as usize];
    let signature_sym = &symtab_sct.symbols.as_ref().unwrap()[sct.header.sh_info as usize];
    let signature_sym_name = signature_sym.symbol_name.as_ref().unwrap();

    base_info.push(Spans::from(vec![
        Span::raw("Related Symbol Table(from sh_link): "),
        Span::raw(&symtab_sct.name),
    ]));

    if sct.header.sh_flags & section::SHF_INFO_LINK != 0 {
        base_info.push(Spans::from(vec![
            Span::raw("Section Group Signature (from sh_info): "),
            Span::raw(signature_sym_name),
        ]));
    }

    base_info
}

fn get_first_globsym_name_from_sh_info<'a>(symtab_sct: &'a section::Section64) -> &'a str {
    assert!(symtab_sct.symbols.is_some());

    let first_sym = &symtab_sct.symbols.as_ref().unwrap()[symtab_sct.header.sh_info as usize];
    let first_sym_name = &first_sym.symbol_name;

    assert!(first_sym_name.is_some());

    first_sym_name.as_ref().unwrap()
}

fn section_attribute_spans<'a, T>(
    attribute: &'a str,
    to_str_f: fn(T) -> &'a str,
    value: T,
) -> Spans<'a> {
    Spans::from(vec![
        Span::raw(attribute),
        Span::raw(": "),
        Span::raw(to_str_f(value)),
    ])
}

fn sct_type_string<'a>(sct_type: section::TYPE) -> &'a str {
    match sct_type {
        section::TYPE::NULL => "NULL",
        section::TYPE::PROGBITS => "PROGBITS",
        section::TYPE::SYMTAB => "SYMTAB",
        section::TYPE::STRTAB => "STRTAB",
        section::TYPE::RELA => "RELA",
        section::TYPE::REL => "REL",
        section::TYPE::HASH => "HASH",
        section::TYPE::DYNAMIC => "DYNAMIC",
        section::TYPE::NOTE => "NOTE",
        section::TYPE::NOBITS => "NOBITS",
        section::TYPE::SHLIB => "SHLIB",
        section::TYPE::DYNSYM => "DYNSYM",
        section::TYPE::INITARRAY => "INIT_ARRAY",
        section::TYPE::FINIARRAY => "FINI_ARRAY",
        section::TYPE::PREINITARRAY => "PREINIT_ARRAY",
        section::TYPE::GROUP => "GROUP",
        section::TYPE::SYMTABSHNDX => "SYMTAB SECTION INDEX",
        _ => "unknown",
    }
}
fn sct_flag_string(sct_flag: u64) -> String {
    let write_str_with = |s: &mut String, c: char, const_flag: u64| {
        if sct_flag & const_flag != 0 {
            s.push(c);
        }
    };
    let mut s = String::new();

    write_str_with(&mut s, 'A', section::SHF_ALLOC);
    write_str_with(&mut s, 'X', section::SHF_EXECINSTR);
    write_str_with(&mut s, 'I', section::SHF_INFO_LINK);

    s
}

fn section_items(elf_file: &file::ELF64) -> Vec<ListItem> {
    section_names(elf_file)
        .iter()
        .map(|name| ListItem::new(vec![Spans::from(vec![Span::raw(name.to_string())])]))
        .collect()
}

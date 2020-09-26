use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use elf_utilities::{file, header, symbol};

pub fn header_paragraphs(elf_file: &file::ELF64) -> Paragraph {
    Paragraph::new(vec![
        header_attribute_spans("Class", elf_class_string, elf_file.ehdr.get_class()),
        header_attribute_spans("Data", elf_data_string, elf_file.ehdr.get_data()),
        header_attribute_spans(
            "ObjectVersion",
            elf_version_string,
            elf_file.ehdr.get_object_version(),
        ),
        header_attribute_spans("OS/ABI", elf_osabi_string, elf_file.ehdr.get_osabi()),
        header_attribute_spans("Type", elf_type_string, elf_file.ehdr.get_type()),
        header_attribute_spans("Machine", elf_machine_string, elf_file.ehdr.get_machine()),
        header_attribute_spans(
            "FileVersion",
            elf_version_string,
            elf_file.ehdr.get_file_version(),
        ),
        Spans::from(vec![
            Span::raw("Entry point address: "),
            Span::raw(elf_entry_string(elf_file)),
        ]),
        Spans::from(vec![
            Span::raw("Start of program headers: "),
            Span::raw(format!("{} (bytes into file)", elf_file.ehdr.e_phoff)),
        ]),
        Spans::from(vec![
            Span::raw("Start of section headers: "),
            Span::raw(format!("{} (bytes into file)", elf_file.ehdr.e_shoff)),
        ]),
        Spans::from(vec![
            Span::raw("Flags: "),
            Span::raw(format!("0x{:x}", elf_file.ehdr.e_flags)),
        ]),
        Spans::from(vec![
            Span::raw("Size of this header: "),
            Span::raw(format!("{} (bytes)", elf_file.ehdr.e_ehsize)),
        ]),
        Spans::from(vec![
            Span::raw("Size of program header: "),
            Span::raw(format!("{} (bytes)", elf_file.ehdr.e_phentsize)),
        ]),
        Spans::from(vec![
            Span::raw("Number of program header: "),
            Span::raw(format!("{}", elf_file.ehdr.e_phnum)),
        ]),
        Spans::from(vec![
            Span::raw("Size of section headers: "),
            Span::raw(format!("{} (bytes)", elf_file.ehdr.e_shentsize)),
        ]),
        Spans::from(vec![
            Span::raw("Number of section headers: "),
            Span::raw(format!("{}", elf_file.ehdr.e_shnum)),
        ]),
        Spans::from(vec![
            Span::raw("Section header string table index: "),
            Span::raw(format!(
                "{} ({})",
                elf_file.ehdr.e_shstrndx, elf_file.sections[elf_file.ehdr.e_shstrndx as usize].name
            )),
        ]),
    ])
    .block(Block::default().title("Header").borders(Borders::ALL))
}

fn header_attribute_spans<'a, T>(
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

fn elf_class_string<'a>(class: header::ELFCLASS) -> &'a str {
    match class {
        header::ELFCLASS::CLASS64 => "ELF64",
        header::ELFCLASS::CLASS32 => "ELF32",
        header::ELFCLASS::CLASSNone => "None",
        _ => "INVALID",
    }
}
fn elf_data_string<'a>(data: header::ELFDATA) -> &'a str {
    match data {
        header::ELFDATA::DATA2LSB => "2's complement little endian",
        header::ELFDATA::DATA2MSB => "2's complement big endian",
        header::ELFDATA::DATA2NUM => "2's complement arch-specific:",
        _ => "invalid data encoding",
    }
}
fn elf_version_string<'a>(version: header::ELFVERSION) -> &'a str {
    match version {
        header::ELFVERSION::VERSIONCURRENT => "1 (current)",
        _ => "invalid version",
    }
}
fn elf_osabi_string<'a>(osabi: header::ELFOSABI) -> &'a str {
    match osabi {
        header::ELFOSABI::NONE | header::ELFOSABI::SYSV => "UNIX - System V",
        header::ELFOSABI::HPUX => "UNIX - HP-UX",
        header::ELFOSABI::NETBSD => "UNIX - NetBSD",
        header::ELFOSABI::GNU | header::ELFOSABI::LINUX => "UNIX - GNU",
        header::ELFOSABI::SOLARIS => "UNIX - Solaris",
        header::ELFOSABI::AIX => "UNIX - AIX",
        header::ELFOSABI::IRIX => "UNIX - IRIX",
        header::ELFOSABI::FREEBSD => "UNIX - FreeBSD",
        header::ELFOSABI::TRU64 => "UNIX - TRU64",
        header::ELFOSABI::MODESTO => "Novell - Modesto",
        header::ELFOSABI::OPENBSD => "UNIX - OpenBSD",
        header::ELFOSABI::ARM => "ARM",
        header::ELFOSABI::STANDALONE => "STANDALONE App",
        _ => "unsupported os/abi",
    }
}
fn elf_type_string<'a>(elf_type: header::ELFTYPE) -> &'a str {
    match elf_type {
        header::ELFTYPE::CORE => "CORE (Core file)",
        header::ELFTYPE::NONE => "NONE (None)",
        header::ELFTYPE::DYN => "DYN (Shared object file)",
        header::ELFTYPE::EXEC => "EXEC (Executable file)",
        header::ELFTYPE::REL => "REL (Relocatable file)",
        _ => "unknown",
    }
}
fn elf_machine_string<'a>(machine: header::ELFMACHINE) -> &'a str {
    match machine {
        header::ELFMACHINE::EMX8664 => "Advanced Micro Devices X86-64",
        _ => "unknown",
    }
}
fn elf_entry_string(elf_file: &file::ELF64) -> String {
    let entry_point = format!("0x{:x}", elf_file.ehdr.e_entry);
    let symbol_table = elf_file.first_section_by(|sct| sct.name == ".symtab");
    if symbol_table.is_none() {
        return entry_point;
    }

    let symbol_table = symbol_table.unwrap();
    for sym in symbol_table.symbols.as_ref().unwrap() {
        if sym.st_value == elf_file.ehdr.e_entry {
            if sym.symbol_name.is_none() || sym.get_type() != symbol::STT_FUNC {
                continue;
            }
            return format!("{} ({})", sym.symbol_name.as_ref().unwrap(), entry_point);
        }
    }

    entry_point
}

use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use elf_utilities::{file, header, section::Contents64, symbol};

pub fn header_information(elf_file: &file::ELF64) -> Paragraph {
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

fn elf_class_string<'a>(class: header::Class) -> &'a str {
    match class {
        header::Class::Bit64 => "ELF64",
        header::Class::Bit32 => "ELF32",
        header::Class::None => "None",
        _ => "INVALID",
    }
}
fn elf_data_string<'a>(data: header::Data) -> &'a str {
    match data {
        header::Data::LSB2 => "2's complement little endian",
        header::Data::MSB2 => "2's complement big endian",
        header::Data::Num => "2's complement arch-specific:",
        _ => "invalid data encoding",
    }
}
fn elf_version_string<'a>(version: header::Version) -> &'a str {
    match version {
        header::Version::Current => "1 (current)",
        _ => "invalid version",
    }
}
fn elf_osabi_string<'a>(osabi: header::OSABI) -> &'a str {
    match osabi {
        header::OSABI::None | header::OSABI::SysV => "UNIX - System V",
        header::OSABI::HPUX => "UNIX - HP-UX",
        header::OSABI::NetBSD => "UNIX - NetBSD",
        header::OSABI::GNU | header::OSABI::Linux => "UNIX - GNU",
        header::OSABI::Solaris => "UNIX - Solaris",
        header::OSABI::AIX => "UNIX - AIX",
        header::OSABI::Irix => "UNIX - IRIX",
        header::OSABI::FreeBSD => "UNIX - FreeBSD",
        header::OSABI::TRU64 => "UNIX - TRU64",
        header::OSABI::Modesto => "Novell - Modesto",
        header::OSABI::OPENBSD => "UNIX - OpenBSD",
        header::OSABI::Arm => "Arm",
        header::OSABI::Standalone => "STANDALONE App",
        _ => "unsupported os/abi",
    }
}
fn elf_type_string<'a>(elf_type: header::Type) -> &'a str {
    match elf_type {
        header::Type::Core => "CORE (Core file)",
        header::Type::None => "NONE (None)",
        header::Type::Dyn => "DYN (Shared object file)",
        header::Type::Exec => "EXEC (Executable file)",
        header::Type::Rel => "REL (Relocatable file)",
        _ => "unknown",
    }
}
fn elf_machine_string<'a>(machine: header::Machine) -> &'a str {
    match machine {
        header::Machine::X8664 => "Advanced Micro Devices X86-64",
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
    if let Contents64::Symbols(symbols) = &symbol_table.contents {
        for sym in symbols {
            if sym.st_value == elf_file.ehdr.e_entry {
                if sym.symbol_name.is_none() || sym.get_type() != symbol::Type::Func {
                    continue;
                }
                return format!("{} ({})", sym.symbol_name.as_ref().unwrap(), entry_point);
            }
        }
    }

    entry_point
}

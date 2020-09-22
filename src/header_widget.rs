use tui::widgets::{Paragraph, Block, Borders};
use tui::text::{Span, Spans};

use elf_utilities::{file, header};

pub fn header_paragraph<'a>(elf_file: &'a file::ELF64) -> Paragraph<'a>{
    Paragraph::new(vec![
        elf_class_spans(elf_file),
        elf_data_spans(elf_file),
    ]).block(Block::default().title("Header").borders(Borders::ALL))
}

fn elf_class_spans(elf_file: &file::ELF64) -> Spans{
    Spans::from(vec![
        Span::raw("Class: "),
        Span::raw(match elf_file.ehdr.get_class() {
            header::ELFCLASS::CLASS64 => "ELF64",
            header::ELFCLASS::CLASS32 => "ELF32",
            header::ELFCLASS::CLASSNone => "None",
            _ => "INVALID",
        }),
    ])
}

fn elf_data_spans(elf_file: &file::ELF64) -> Spans{
    Spans::from(vec![
        Span::raw("Data: "),
        Span::raw(match elf_file.ehdr.get_data() {
            header::ELFDATA::DATA2LSB => "2's complement little endian",
            header::ELFDATA::DATA2MSB => "2's complement big endian",
            header::ELFDATA::DATA2NUM => "2's complement arch-specific:",
            _ => "invalid data encoding",
        }),
    ])
}
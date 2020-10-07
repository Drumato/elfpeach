use elf_utilities::{file, segment};
use tui::layout::Corner;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn segment_list(elf_file: &file::ELF64) -> List {
    let items = segment_items(elf_file);

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Segments"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .start_corner(Corner::TopLeft)
}

pub fn segment_information<'a>(seg: &'a segment::Segment64) -> Paragraph<'a> {
    Paragraph::new(vec![
        Spans::from(vec![
            Span::raw("Type: "),
            Span::raw(seg_type_string(seg.header.get_type())),
        ]),
        Spans::from(vec![
            Span::raw("Offset: "),
            Span::raw(format!("0x{:x}", seg.header.p_offset)),
        ]),
        Spans::from(vec![
            Span::raw("Virtual Address: "),
            Span::raw(format!("0x{:x}", seg.header.p_vaddr)),
        ]),
        Spans::from(vec![
            Span::raw("Physical Address: "),
            Span::raw(format!("0x{:x}", seg.header.p_paddr)),
        ]),
        Spans::from(vec![
            Span::raw("File Size: "),
            Span::raw(format!("0x{:x}", seg.header.p_filesz)),
        ]),
        Spans::from(vec![
            Span::raw("Memory Size: "),
            Span::raw(format!("0x{:x}", seg.header.p_memsz)),
        ]),
        Spans::from(vec![
            Span::raw("Flags: "),
            Span::raw(seg_flag_string(seg.header.p_flags)),
        ]),
        Spans::from(vec![
            Span::raw("Align: "),
            Span::raw(format!("0x{:x}", seg.header.p_align)),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Segments"))
}

fn seg_type_string<'a>(seg_type: segment::Type) -> &'a str {
    match seg_type {
        segment::Type::Null => "NULL",
        segment::Type::TLS => "TLS",
        segment::Type::Note => "NOTE",
        segment::Type::Load => "LOAD",
        segment::Type::Phdr => "PHDR",
        segment::Type::Dynamic  => "DYNAMIC",
        segment::Type::Num => "NUM",
        segment::Type::ShLib => "SHLIB",
        segment::Type::Interp => "INTERP",
        segment::Type::GNUEHFrame => "GNU_EH_FRAME",
        segment::Type::GNUStack => "GNU_STACK",
        segment::Type::GNURelRO => "GNU_RELRO",
        _ => "unknown",
    }
}
fn seg_flag_string(seg_flag: u32) -> String {
    let write_str_with = |s: &mut String, c: char, const_flag: u32| {
        s.push(if seg_flag & const_flag != 0 { c } else { ' ' });
    };
    let mut s = String::new();

    write_str_with(&mut s, 'R', segment::PF_R);
    write_str_with(&mut s, 'W', segment::PF_W);
    write_str_with(&mut s, 'E', segment::PF_X);

    s
}

fn segment_items(elf_file: &file::ELF64) -> Vec<ListItem> {
    (0..elf_file.ehdr.e_phnum)
        .map(|name| ListItem::new(vec![Spans::from(vec![Span::raw(name.to_string())])]))
        .collect()
}

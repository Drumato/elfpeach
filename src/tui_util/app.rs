use std::cell::RefCell;

use crate::tui_util::{StatefulList, TabsState};
use crate::{
    widgets::dynamics, widgets::elf_header, widgets::sections, widgets::segments, widgets::symbols,
};

use elf_utilities::{file, section};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Tabs};
use tui::Frame;

pub struct App<'a> {
    pub tabs: TabsState<'a>,
    pub sections: RefCell<StatefulList<String>>,
    pub segments: RefCell<StatefulList<String>>,
    pub symbol_table: RefCell<StatefulList<String>>,
    pub dynamic_symbol_table: RefCell<StatefulList<String>>,
    pub dynamic_table: RefCell<StatefulList<String>>,

    // 描画のたびにテーブルを探索すると無駄なので,
    // ファイル読み込み時に保持してしまう.
    symtab_sct: Option<&'a section::Section64>,
    dynsym_sct: Option<&'a section::Section64>,
    dynamic_sct: Option<&'a section::Section64>,
}

impl<'a> App<'a> {
    pub fn state(&self) -> AppState {
        AppState::from(self.tabs.current.as_str())
    }

    /// 最も大枠のレイアウトを描画する
    pub fn draw<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        file_path: String,
        elf_file: &'a file::ELF64,
    ) {
        let outline = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(outline);

        let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
        frame.render_widget(block, outline);

        let tabs = Tabs::new(self.tabs.titles.clone())
            .block(Block::default().borders(Borders::ALL).title(file_path))
            .select(self.tabs.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        frame.render_widget(tabs, chunks[0]);

        match self.state() {
            AppState::Header => self.draw_header_tab(frame, &elf_file, chunks[1]),
            AppState::Section => self.draw_section_tab(frame, &elf_file, chunks[1]),
            AppState::Segment => self.draw_segment_tab(frame, &elf_file, chunks[1]),
            AppState::Symbol => self.draw_symbol_tab(frame, &elf_file, chunks[1], self.state()),
            AppState::DynSym => self.draw_symbol_tab(frame, &elf_file, chunks[1], self.state()),
            AppState::Dynamics => self.draw_dynamic_tab(frame, &elf_file, chunks[1]),
        }
    }

    fn draw_header_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
    ) {
        let inner = elf_header::header_information(&elf_file);
        frame.render_widget(inner, area);
    }
    fn draw_section_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
    ) {
        let chunks = self.split_list_and_detail(area);

        let scts = sections::section_list(&elf_file);
        let selected_sct = self.sections.borrow().state.selected().unwrap();

        frame.render_stateful_widget(scts, chunks[0], &mut self.sections.borrow_mut().state);

        let selected_sct = &elf_file.sections[selected_sct];
        let sct_info = sections::section_information(elf_file, selected_sct);
        frame.render_widget(sct_info, chunks[1]);
    }
    fn draw_segment_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
    ) {
        let chunks = self.split_list_and_detail(area);

        let segs = segments::segment_list(&elf_file);
        let selected_seg = self.segments.borrow().state.selected().unwrap();

        frame.render_stateful_widget(segs, chunks[0], &mut self.segments.borrow_mut().state);

        let selected_seg = elf_file.segments[selected_seg];
        let seg_info = segments::segment_information(&selected_seg);
        frame.render_widget(seg_info, chunks[1]);
    }
    fn draw_symbol_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
        state: AppState,
    ) {
        let chunks = self.split_list_and_detail(area);

        match state {
            AppState::Symbol => {
                let symbols = symbols::symbol_table_list(self.symtab_sct);
                frame.render_stateful_widget(
                    symbols,
                    chunks[0],
                    &mut self.symbol_table.borrow_mut().state,
                );

                let sym_info = symbols::symbol_information(
                    elf_file,
                    self.symtab_sct.unwrap(),
                    self.symbol_table.borrow().state.selected().unwrap(),
                );
                frame.render_widget(sym_info, chunks[1]);
            }
            AppState::DynSym => {
                let symbols = symbols::symbol_table_list(self.dynsym_sct);
                frame.render_stateful_widget(
                    symbols,
                    chunks[0],
                    &mut self.dynamic_symbol_table.borrow_mut().state,
                );

                let sym_info = symbols::symbol_information(
                    elf_file,
                    self.dynsym_sct.unwrap(),
                    self.dynamic_symbol_table.borrow().state.selected().unwrap(),
                );
                frame.render_widget(sym_info, chunks[1]);
            }
            _ => unreachable!(),
        }
    }
    fn draw_dynamic_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
    ) {
        let chunks = self.split_list_and_detail(area);

        let dynamics = dynamics::dynamic_list(self.dynamic_sct);
        frame.render_stateful_widget(
            dynamics,
            chunks[0],
            &mut self.dynamic_table.borrow_mut().state,
        );

        let dyn_info = dynamics::dynamic_information(
            elf_file,
            self.dynamic_sct.unwrap(),
            self.symtab_sct.unwrap(),
            self.dynamic_table.borrow().state.selected().unwrap(),
        );
        frame.render_widget(dyn_info, chunks[1]);
    }
    fn split_list_and_detail(&self, area: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(area)
    }

    pub fn new(elf_file: &'a file::ELF64) -> Self {
        let symtab_sct =
            elf_file.first_section_by(|sct| sct.header.get_type() == section::Type::SymTab);
        let dynsym_sct =
            elf_file.first_section_by(|sct| sct.header.get_type() == section::Type::DynSym);
        let dynamic_sct =
            elf_file.first_section_by(|sct| sct.header.get_type() == section::Type::Dynamic);

        let mut sections = StatefulList::with_items(sections::section_names(elf_file));
        sections.next();

        let mut segments =
            StatefulList::with_items((0..elf_file.ehdr.e_phnum).map(|n| n.to_string()).collect());
        segments.next();

        let mut symbols = StatefulList::with_items(symbols::symbol_names(symtab_sct));
        symbols.next();

        let mut dynamic_symbols = StatefulList::with_items(symbols::symbol_names(dynsym_sct));
        dynamic_symbols.next();

        let mut dynamics = StatefulList::with_items(dynamics::dynamic_names(dynamic_sct));
        dynamics.next();

        Self {
            tabs: create_tabs_state(elf_file, symtab_sct, dynsym_sct, dynamic_sct),
            sections: RefCell::new(sections),
            segments: RefCell::new(segments),
            symbol_table: RefCell::new(symbols),
            dynamic_symbol_table: RefCell::new(dynamic_symbols),
            dynamic_table: RefCell::new(dynamics),
            symtab_sct,
            dynsym_sct,
            dynamic_sct,
        }
    }
}

fn create_tabs_state<'a>(
    elf_file: &'a file::ELF64,
    symtab_sct: Option<&'a section::Section64>,
    dynsym_sct: Option<&'a section::Section64>,
    dynamic_sct: Option<&'a section::Section64>,
) -> TabsState<'a> {
    let mut state = TabsState::new(vec!["Header", "Sections"]);

    if elf_file.ehdr.e_phnum != 0 {
        state.push("Segments");
    }

    if symtab_sct.is_some() {
        state.push("Symbols");
    }
    if dynsym_sct.is_some() {
        state.push("DynSyms");
    }
    if dynamic_sct.is_some() {
        state.push("Dynamics");
    }

    state
}

pub enum AppState {
    Header,
    Section,
    Segment,
    Symbol,
    DynSym,
    Dynamics,
}

impl<'a> From<&'a str> for AppState {
    fn from(s: &'a str) -> Self {
        match s {
            "Header" => AppState::Header,
            "Sections" => AppState::Section,
            "Segments" => AppState::Segment,
            "Symbols" => AppState::Symbol,
            "DynSyms" => AppState::DynSym,
            "Dynamics" => AppState::Dynamics,
            _ => panic!("not found such a mode"),
        }
    }
}

use color_eyre::Result;
use crossterm::event::KeyModifiers;
use itertools::Itertools;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
};
use unicode_width::UnicodeWidthStr;

const ITEM_HEIGHT: usize = 4;

struct TableData {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(columns: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        TableData { columns, rows }
    }
}

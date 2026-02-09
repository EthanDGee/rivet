use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
};

use crate::app::App;
use crate::screens::ScreenRenderable;
use crate::ui::table::TableView;
use crate::ui::themes::ColorPalette;

#[derive(Debug, Default)]
pub struct ResultsScreen {
    pub table_view: Option<TableView>,
}

impl ResultsScreen {
    pub fn new() -> Self {
        ResultsScreen { table_view: None }
    }

    pub fn render(&mut self, frame: &mut Frame, app: &App, inner_area: Rect) {
        if let Some(table_view) = &mut self.table_view {
            let theme = &app.theme;
            let data = &table_view.data;
            // Adjust Constraint::Length by adding padding
            let constraints: Vec<Constraint> = data
                .max_lengths
                .iter()
                .map(|length| Constraint::Length(*length as u16 + 2))
                .collect();

            let header = Row::new(data.columns.clone())
                .style(
                    Style::default()
                        .fg(theme.header_text)
                        .add_modifier(Modifier::BOLD),
                )
                .bottom_margin(1); // Add bottom margin to header row for spacing

            let rows: Vec<Row> = data
                .rows
                .iter()
                .map(|row_data| {
                    let cells = row_data
                        .iter()
                        .map(|cell_data| Cell::from(cell_data.as_str()))
                        .collect::<Vec<Cell>>();
                    Row::new(cells)
                })
                .collect();

            let table = Table::new(rows, &constraints)
                .header(header)
                .block(Block::default().padding(Padding::horizontal(1)))
                .row_highlight_style(Style::default().bg(theme.highlight).fg(Color::Black))
                .highlight_symbol(">> ");

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            let table_area = inner_area;

            frame.render_stateful_widget(table, table_area, &mut table_view.state);
            frame.render_stateful_widget(
                scrollbar,
                table_area.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut table_view.scroll_state,
            );
        }
    }
}

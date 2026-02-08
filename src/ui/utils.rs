use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    symbols::border,
    widgets::Block,
};

use crate::ui::themes::ColorPalette;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // return the middle chunk
}

pub fn floating_window(frame: &mut Frame, theme: &ColorPalette) -> Rect {
    let window = centered_rect(75, 75, frame.area());
    let border = Block::bordered()
        .border_style(Style::default().fg(theme.inner_border))
        .border_set(border::THICK);

    frame.render_widget(border, window);

    window
}

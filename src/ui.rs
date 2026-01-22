use crate::app::{App, Screens};
use crate::constants::TOOL_NAME;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};
use std::format;

// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
pub fn ui(frame: &mut Frame, app: &App) {
    // set up screen border
    let title = Line::from(format!("{}({})", TOOL_NAME, app.sql_path).bold());
    let instructions = Line::from(vec![
        " Help ".into(),
        "<C-H>".blue().bold(),
        " Save ".into(),
        "<C-S>".blue().bold(),
        " Quit ".into(),
        "<C-Q> ".blue().bold(),
    ]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    frame.render_widget(block.clone(), frame.area());

    if let Screens::Main = app.current_screen {
        let db_info = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            app.sql_path.to_string().yellow(),
        ])]);

        let paragraph = Paragraph::new(db_info).centered().block(block);
        frame.render_widget(paragraph, frame.area());
        return;
    }

    if let Screens::Help = app.current_screen {
        let floating_window = centered_rect(60, 60, frame.area());

        let commands = Paragraph::new("HELP").centered();

        frame.render_widget(commands, floating_window);
    }
}

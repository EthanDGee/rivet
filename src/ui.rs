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
    if let Screens::Main = app.current_screen {
        let title = Line::from(TOOL_NAME.bold());
        let instructions = Line::from(vec![" Quit ".into(), "<C-Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let db_info = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            app.sql_path.to_string().yellow(),
        ])]);

        let paragraph = Paragraph::new(db_info).centered().block(block);
        frame.render_widget(paragraph, frame.area());
    }
}

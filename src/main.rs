use std::io;
mod app;
mod constants;
mod sql_session;
mod table;
mod terminal;
use app::App;
mod ui;

fn main() -> io::Result<()> {
    let mut app: App = App::new("test.sqlite3".to_string());
    ratatui::run(|terminal| app.run(terminal))
}

use std::io;
mod app;
mod constants;
mod sql_session;
use app::App;
mod ui;

use std::env;
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut read_only: bool = false;
    if !args.is_empty() {
        for arg in args {
            if arg.to_string().eq("-r") {
                read_only = true;
            }
        }
    }

    let mut app: App = App::new("test.sqlite3".to_string(), read_only);
    ratatui::run(|terminal| app.run(terminal))
}

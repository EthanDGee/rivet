use std::io;
mod app;
use app::App;
mod model;
mod ui;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the sqlite file
    file: String,

    /// Open in read-only mode
    #[arg(short, long)]
    read_only: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut app: App = App::new(args.file, args.read_only);
    ratatui::run(|terminal| app.run(terminal))
}

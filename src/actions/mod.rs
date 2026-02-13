mod global_actions;
mod help_actions;
mod quit_actions;
mod results_actions;
mod terminal_actions;
use crate::actions::global_actions::GlobalActions;
use crate::actions::help_actions::HelpActions;
use crate::actions::quit_actions::QuitActions;
use crate::actions::results_actions::ResultActions;
use crate::actions::terminal_actions::TerminalActions;
use crate::app::App;
use crossterm::event::KeyEvent;

pub enum Actions {
    Global(GlobalActions),
    Terminal(TerminalActions),
    Result(ResultActions),
    Help(HelpActions),
    Quit(QuitActions),
    NoAction,
}

impl Default for Actions {
    fn default() -> Self {
        Actions::NoAction
    }
}

pub trait Actionable {
    fn take_action(app: &mut App, key_event: KeyEvent);
}

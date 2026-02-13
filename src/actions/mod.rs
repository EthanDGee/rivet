mod global_actions;
mod terminal_actions;
use crate::actions::global_actions::GlobalActions;
use crate::actions::terminal_actions::TerminalActions;
use crate::app::App;
use crossterm::event::KeyEvent;

pub enum Actions {
    Global(GlobalActions),
    Terminal(TerminalActions),
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

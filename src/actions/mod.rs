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
use crate::ui::screens::Screen;
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

impl Actions {
    pub fn handle_actions(app: &mut App, key_event: KeyEvent) {
        // first handle global actions
        <GlobalActions as Actionable>::take_action(app, key_event);

        // screen specific actions
        match app.screen {
            Screen::Terminal(_) => <TerminalActions as Actionable>::take_action(app, key_event),
            Screen::Results(_) => <ResultActions as Actionable>::take_action(app, key_event),
            Screen::Help(_) => <HelpActions as Actionable>::take_action(app, key_event),
            Screen::Exiting(_) => <QuitActions as Actionable>::take_action(app, key_event),
        }
    }
}

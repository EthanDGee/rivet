use crate::{
    actions::Actionable,
    app::App,
    ui::screens::{Screen, help_screen::HelpScreen, quit_screen::QuitScreen},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum GlobalActions {
    Save,
    Rollback,
    Quit,
    Help,
}

impl Actionable for GlobalActions {
    fn take_action(app: &mut App, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                app.session.commit();
                app.notifications
                    .notify("Save", "Changes to database saved successfully.")
            }
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => {
                app.session.rollback();
                app.notifications
                    .notify("Rollback", "Staged changes successfully reverted.")
            }
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                app.screen = Screen::Exiting(QuitScreen::new())
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                app.screen = Screen::Exiting(QuitScreen::new())
            }
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                app.screen = Screen::Help(HelpScreen::new())
            }
            _ => {}
        }
    }
}

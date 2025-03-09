use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.modifiers, key_event.code) {
        (_, KeyCode::Esc) => {
            app.quit();
        }

        (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
            app.quit();
        }

        _ => {
            app.input.handle_event(&key_event);
        }
    }

    Ok(())
}

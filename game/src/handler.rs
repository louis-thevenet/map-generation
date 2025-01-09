use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => {
            app.map_rendering.position.1 += 1;
        }
        KeyCode::Down => {
            app.map_rendering.position.1 -= 1;
        }
        KeyCode::Right => {
            app.map_rendering.position.0 += 1;
        }
        KeyCode::Left => {
            app.map_rendering.position.0 -= 1;
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

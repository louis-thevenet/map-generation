use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use game_core::map::CHUNK_SIZE;
const CTRL_SPEED_MODIFIER: f64 = 10.;
const SHIFT_SPEED_MODIFIER: f64 = 50.;
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
        KeyCode::Tab => {
            app.map_mode = match app.map_mode {
                crate::app::MapMode::Local => crate::app::MapMode::Global(CHUNK_SIZE),
                crate::app::MapMode::Global(_) => crate::app::MapMode::Local,
            }
        }
        KeyCode::Up => {
            app.position.1 += if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1.
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1.
            } * match app.map_mode {
                crate::app::MapMode::Local => 1.,
                crate::app::MapMode::Global(s) => s,
            };
        }
        KeyCode::Down => {
            app.position.1 -= if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1.
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1.
            } * match app.map_mode {
                crate::app::MapMode::Local => 1.,
                crate::app::MapMode::Global(s) => s,
            };
        }
        KeyCode::Right => {
            app.position.0 += if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1.
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1.
            } * match app.map_mode {
                crate::app::MapMode::Local => 1.,
                crate::app::MapMode::Global(s) => s,
            };
        }
        KeyCode::Left => {
            app.position.0 -= if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1.
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1.
            } * match app.map_mode {
                crate::app::MapMode::Local => 1.,
                crate::app::MapMode::Global(s) => s,
            };
        }
        KeyCode::Enter => {
            if let crate::app::MapMode::Global(s) = app.map_mode {
                if s / 1.1 > 1.0 {
                    app.map_mode = crate::app::MapMode::Global(s / 1.1);
                }
            }
        }
        KeyCode::Backspace => {
            if let crate::app::MapMode::Global(s) = app.map_mode {
                app.map_mode = crate::app::MapMode::Global(s * 1.1);
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

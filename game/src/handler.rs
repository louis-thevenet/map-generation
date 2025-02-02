use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
const CTRL_SPEED_MODIFIER: isize = 10;
const SHIFT_SPEED_MODIFIER: isize = 50;
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
                crate::app::MapMode::Local => crate::app::MapMode::Global,
                crate::app::MapMode::Global => crate::app::MapMode::Local,
            }
        }
        KeyCode::Char('t') => {
            app.visualization_mode = match app.visualization_mode {
                crate::app::VisualizationMode::Normal => crate::app::VisualizationMode::Temperature,
                crate::app::VisualizationMode::Temperature => crate::app::VisualizationMode::Normal,
            };
        }
        KeyCode::Up => {
            app.position.1 += if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1
            } * match app.map_mode {
                crate::app::MapMode::Local => 1,
                crate::app::MapMode::Global => app.map.get_chunk_size(),
            };
        }
        KeyCode::Down => {
            app.position.1 -= if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1
            } * match app.map_mode {
                crate::app::MapMode::Local => 1,
                crate::app::MapMode::Global => app.map.get_chunk_size(),
            };
        }
        KeyCode::Right => {
            app.position.0 += if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1
            } * match app.map_mode {
                crate::app::MapMode::Local => 1,
                crate::app::MapMode::Global => app.map.get_chunk_size(),
            };
        }
        KeyCode::Left => {
            app.position.0 -= if key_event.modifiers == KeyModifiers::CONTROL {
                CTRL_SPEED_MODIFIER
            } else {
                1
            } * if key_event.modifiers == KeyModifiers::SHIFT {
                SHIFT_SPEED_MODIFIER
            } else {
                1
            } * match app.map_mode {
                crate::app::MapMode::Local => 1,
                crate::app::MapMode::Global => app.map.get_chunk_size(),
            };
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

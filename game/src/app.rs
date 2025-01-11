use std::{collections::HashMap, error};

use game_core::map::Map;
use ratatui::style::Style;

use crate::ui::MapRendering;

/// Application result type.
#[allow(clippy::module_name_repetitions)]
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub counter: u8,
    pub map_rendering: MapRendering,
    pub map: Map,
}

impl Default for App {
    fn default() -> Self {
        let mut symbols = HashMap::new();
        symbols.insert(
            game_core::tile::TileType::Water,
            ("≈".into(), Style::new().fg(ratatui::style::Color::Blue)),
        );
        symbols.insert(
            game_core::tile::TileType::Beach,
            ("B".into(), Style::new().fg(ratatui::style::Color::Yellow)),
        );
        symbols.insert(
            game_core::tile::TileType::Land,
            ("L".into(), Style::new().fg(ratatui::style::Color::Green)),
        );
        symbols.insert(
            game_core::tile::TileType::Mountain,
            ("M".into(), Style::new().fg(ratatui::style::Color::White)),
        );

        Self {
            running: true,
            counter: 0,
            map: Map::default(),
            map_rendering: MapRendering {
                symbols,
                position: (1000, 1000),
            },
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub const fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}

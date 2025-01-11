use std::{collections::HashMap, error};

use game_core::{map::Map, tile::TileType};
use ratatui::style::Style;

/// Application result type.
#[allow(clippy::module_name_repetitions)]
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum MapMode {
    Local,
    Global,
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub counter: u8,
    pub symbols: HashMap<TileType, (String, Style)>,
    pub map_mode: MapMode,
    pub position: (isize, isize),
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
            ("░".into(), Style::new().fg(ratatui::style::Color::Yellow)),
        );
        symbols.insert(
            game_core::tile::TileType::Land,
            ("█".into(), Style::new().fg(ratatui::style::Color::Green)),
        );
        symbols.insert(
            game_core::tile::TileType::Mountain,
            ("M".into(), Style::new().fg(ratatui::style::Color::White)),
        );

        Self {
            running: true,
            counter: 0,
            map: Map::new(16.0),
            symbols,
            position: (8000, 8000),
            map_mode: MapMode::Global,
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

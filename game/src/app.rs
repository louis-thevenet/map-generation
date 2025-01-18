use std::{collections::HashMap, error};

use game_core::{map::Map, tile::TileType};
use ratatui::style::Style;

use crate::tile_to_tascii::default_tile_ascii_mapping;

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
    pub symbols: HashMap<TileType, (String, Style)>,
    pub map_mode: MapMode,
    pub position: (isize, isize),
    pub map: Map,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            map: Map::new(16.0),
            symbols: default_tile_ascii_mapping(),
            position: (0, 0),
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
}

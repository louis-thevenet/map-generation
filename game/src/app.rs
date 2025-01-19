use std::error;

use game_core::map::Map;

use crate::fps_counter::FpsCounter;

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
    pub map_mode: MapMode,
    pub position: (isize, isize),
    pub map: Map,
    pub fps_counter: FpsCounter,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            map: Map::new(16.0),
            position: (0, 0),
            map_mode: MapMode::Global,
            fps_counter: FpsCounter::new(),
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
    pub fn tick(&mut self) {
        let _ = self.fps_counter.app_tick();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

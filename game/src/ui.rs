use std::collections::HashMap;

use crate::app::App;
use game_core::{map::Map, tile::TileType};
use rand::Rng;
use ratatui::{
    layout::Position,
    widgets::{Block, Clear, Paragraph, StatefulWidget, Widget},
    Frame,
};
use tracing::debug;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    frame.render_stateful_widget(app.map_rendering.clone(), frame.area(), &mut app.map);
}

#[derive(Debug, Clone)]
pub struct MapRendering {
    pub symbols: HashMap<TileType, String>,
    pub position: (isize, isize),
}
impl StatefulWidget for MapRendering {
    type State = Map;

    #[allow(clippy::cast_possible_wrap)]
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Clear.render(area, buf);
        debug!("Drawing map for position {:?}", self.position);
        for x in 0..area.width {
            for y in 0..area.height {
                let x_map = self.position.0 + x as isize;
                let y_map = self.position.1 - y as isize;
                let symbol = self
                    .symbols
                    .get(&state.get_tile((x_map, y_map)).tile_type)
                    .unwrap();

                let cell = buf.cell_mut(Position::new(x, y));
                if let Some(c) = cell {
                    c.set_symbol(symbol);
                }
            }
        }
        Paragraph::new(format!("{}, {}", self.position.0, self.position.1)).render(area, buf);
    }
}

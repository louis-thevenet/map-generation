use std::{collections::HashMap, f32::consts::FRAC_1_PI};

use game_core::{map::Map, tile::TileType};
use ratatui::{
    layout::Position,
    widgets::{Clear, StatefulWidget, Widget},
    Frame,
};

use crate::app::App;

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

        for x in 0..area.width {
            for y in 0..area.height {
                let x_map = if x < area.width / 2 {
                    self.position.0 - x as isize
                } else {
                    self.position.0 + x as isize
                };

                let y_map = if y < area.height / 2 {
                    self.position.0 - y as isize
                } else {
                    self.position.0 + y as isize
                };
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

        // let chunk_top_left = (
        //     x_top_left % state.get_chunk_size() as isize,
        //     y_top_left % state.get_chunk_size() as isize,
        // );
        // println!(
        //     "{x_top_left } % {} = {:?}",
        //     state.get_chunk_size(),
        //     x_top_left % state.get_chunk_size() as isize
        // );
        // println!("{y_top_left:?}");
        // println!("{chunk_top_left:?}");
        // let x_bottom_right = self.position.0 + area.width as isize / 2;
        // let y_bottom_right = self.position.1 + area.height as isize / 2;
        // let chunk_bottom_right = (
        //     x_bottom_right % state.get_chunk_size() as isize,
        //     y_bottom_right % state.get_chunk_size() as isize,
        // );
        // let mut x_buf = 0;
        // let mut y_buf = 0;
        // for x in chunk_top_left.0..chunk_bottom_right.0 {
        //     for y in chunk_top_left.1..chunk_bottom_right.1 {
        //         for i in 0..state.get_chunk_size() {
        //             for j in 0..state.get_chunk_size() {
        //                 let symbol = self
        //                     .symbols
        //                     .get(&state.get_chunk((x, y)).tiles[j][i].tile_type)
        //                     .unwrap();

        //                 // println!("{x_buf}, {y_buf}");
        //                 let cell = buf.cell_mut(Position::new(x_buf, y_buf));
        //                 if let Some(c) = cell {
        //                     c.set_symbol(symbol);
        //                 }

        //                 x_buf += 1;
        //             }
        //             y_buf += 1;
        //         }
        //         y_buf += state.get_chunk_size() as u16;
        //     }
        //     x_buf += state.get_chunk_size() as u16;
        //     y_buf = 0;
        // }
    }
}

use crate::app::{App, MapMode};
use ratatui::{
    layout::Position,
    style::Style,
    widgets::{Clear, Paragraph, Widget},
    Frame,
};
use tracing::debug;

/// Renders the user interface widgets.
#[allow(clippy::cast_possible_wrap)]
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let buf = frame.buffer_mut();
    Clear.render(area, buf);

    // debug!("Drawing map for position {:?}", app.position);
    // debug!(
    //     "Chunk is {:?}",
    //     app.map
    //         .get_chunk_from_world_coord(app.position)
    //         .average_tile
    // );
    for x in 0..area.width {
        for y in 0..area.height {
            let tile_type = match app.map_mode {
                MapMode::Local => {
                    let x_map = app.position.0 - area.width as isize / 2 + x as isize;
                    let y_map = app.position.1 + area.height as isize / 2 - y as isize;
                    &app.map.get_tile((x_map, y_map)).tile_type
                }
                MapMode::Global => {
                    let x_map = app.position.0 / app.map.get_chunk_size() as isize // Get chunk x coordinate
                        - area.width as isize / 2 // Offset to center app.position
                        + x as isize; // Add offset for current cell
                    let y_map = app.position.1 / app.map.get_chunk_size() as isize
                        + area.height as isize / 2
                        - y as isize;

                    &app.map
                        .get_chunk_from_chunk_coord((x_map, y_map))
                        .average_tile
                }
            };
            let (symbol, style) = if x == area.width / 2 && y == area.height / 2 {
                &("☺".into(), Style::new().fg(ratatui::style::Color::Red))
            } else {
                app.symbols.get(tile_type).unwrap()
            };

            let cell = buf.cell_mut(Position::new(x, y));
            if let Some(c) = cell {
                c.set_symbol(symbol);
                c.set_style(*style);
            }
        }
    }
    Paragraph::new(format!(
        "{:?}\nWorld position: {}, {}\nChunk pos: {}, {}",
        app.map_mode,
        app.position.0,
        app.position.1,
        app.map.chunk_coord_from_world_coord(app.position).0,
        app.map.chunk_coord_from_world_coord(app.position).1
    ))
    .render(area, buf);
}

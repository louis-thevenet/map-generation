use crate::app::{App, MapMode};
use ratatui::{
    layout::Position,
    style::Style,
    widgets::{Clear, Paragraph, Widget},
    Frame,
};
use tracing::error;

/// Renders the user interface widgets.
#[allow(clippy::cast_possible_wrap)]
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let buf = frame.buffer_mut();
    Clear.render(area, buf);

    let half_width = area.width as isize / 2;
    let half_height = area.height as isize / 2;

    for x in 0..area.width as isize {
        for y in 0..area.height as isize {
            let tile_type = match app.map_mode {
                // Take current map position
                // Center it on the screen
                // Add loop offset
                MapMode::Local => {
                    let x_map = app.position.0 - half_width + x;
                    let y_map = app.position.1 + half_height - y;
                    &app.map.get_tile((x_map, y_map)).tile_type
                }
                MapMode::Global => {
                    let chunk_coord = app.map.chunk_coord_from_world_coord(app.position);
                    let x_map = chunk_coord.0 - half_width + x;
                    let y_map = chunk_coord.1 + half_height - y;

                    &app.map
                        .get_chunk_from_chunk_coord((x_map, y_map))
                        .average_tile
                }
            };
            let (symbol, style) = if x == half_width && y == half_height {
                &("☺".into(), Style::new().fg(ratatui::style::Color::Red))
            } else {
                match app.symbols.get(tile_type) {
                    Some(s) => s,
                    None => &{
                        error!("Tile symbol not found");
                        (
                            String::from("X"),
                            Style::new().fg(ratatui::style::Color::Red),
                        )
                    },
                }
            };

            let cell = buf.cell_mut(Position::new(
                x.try_into().unwrap_or_default(),
                y.try_into().unwrap_or_default(),
            ));
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

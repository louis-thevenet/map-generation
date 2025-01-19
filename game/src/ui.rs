use crate::{
    app::{App, MapMode},
    tile_to_ascii::tile_to_ascii,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let [info, _] = Layout::horizontal(Constraint::from_percentages([15, 100 - 15])).areas(area);
    let [fps, info] = Layout::vertical(Constraint::from_lengths([1 + 2, 4 + 2])).areas(info);
    draw_map(app, frame.buffer_mut(), area);

    let _ = app.fps_counter.render_tick();

    Clear.render(info, frame.buffer_mut());
    Clear.render(fps, frame.buffer_mut());

    app.fps_counter
        .to_paragraph()
        .style(Style::reset())
        .block(Block::bordered())
        .render(fps, frame.buffer_mut());

    Paragraph::new(vec![
        format!("{:?}", app.map_mode).into(),
        format!("position: {}, {}", app.position.0, app.position.1,).into(),
        format!(
            "Chunk pos: {}, {}",
            app.map.chunk_coord_from_world_coord(app.position).0,
            app.map.chunk_coord_from_world_coord(app.position).1,
        )
        .into(),
        format!("Generated Chunks: {}", app.map.generated_chunk_count()).into(),
    ])
    .left_aligned()
    .style(Style::reset())
    .block(Block::bordered())
    .render(info, frame.buffer_mut());
}

#[allow(clippy::cast_possible_wrap)]
fn draw_map(app: &mut App, buf: &mut Buffer, area: Rect) {
    Clear.render(area, buf);

    let half_width = area.width as isize / 2;
    let half_height = area.height as isize / 2;

    for x in area.x as isize..area.width as isize {
        for y in area.y as isize..area.height as isize {
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
                ("☺".into(), Style::new().fg(ratatui::style::Color::Red))
            } else {
                tile_to_ascii(tile_type)
            };

            let cell = buf.cell_mut(Position::new(
                x.try_into().unwrap_or_default(),
                y.try_into().unwrap_or_default(),
            ));
            if let Some(c) = cell {
                c.set_symbol(&symbol);
                c.set_style(style);
            }
        }
    }
}

use crate::app::App;
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
    let [fps, info] = Layout::vertical(Constraint::from_lengths([1 + 2, 5 + 2])).areas(info);
    draw_map(app, frame.buffer_mut(), area);

    let position_isize = (app.position.0 as isize, app.position.1 as isize);
    let _ = app.fps_counter.render_tick();

    Clear.render(info, frame.buffer_mut());
    Clear.render(fps, frame.buffer_mut());

    app.fps_counter
        .to_paragraph()
        .style(Style::reset())
        .block(Block::bordered())
        .render(fps, frame.buffer_mut());

    Paragraph::new(vec![
        format!("Scale: {:?}", app.map_mode).into(),
        format!(
            "Current biome: {:?}",
            app.map.get_chunk_from_world_coord(position_isize).biome
        )
        .into(),
        format!("position: {}, {}", position_isize.0, position_isize.1,).into(),
        format!(
            "Chunk pos: {}, {}",
            app.map.chunk_coord_from_world_coord(position_isize).0,
            app.map.chunk_coord_from_world_coord(position_isize).1,
        )
        .into(),
        format!("Generated Chunks: {}", app.map.generated_chunk_count()).into(),
    ])
    .left_aligned()
    .style(Style::reset())
    .block(Block::bordered())
    .render(info, frame.buffer_mut());
}

#[allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn draw_map(app: &mut App, buf: &mut Buffer, area: Rect) {
    Clear.render(area, buf);

    let quarter_width = area.width as isize / 4;
    let half_height = area.height as isize / 2;

    for x in (0..(area.width - area.x) as isize).step_by(2) {
        for y in 0..(area.height - area.y) as isize {
            let chunk_coord = app
                .map
                .chunk_coord_from_world_coord((app.position.0 as isize, app.position.1 as isize));
            let x_map = chunk_coord.0 + x / 2 - quarter_width;

            let y_map = chunk_coord.1 - y + half_height;

            let chunk = &app.map.get_chunk_from_chunk_coord((x_map, y_map));
            let (symbol, style) = if (x_map, y_map) == chunk_coord {
                ("@", Style::new().fg(ratatui::style::Color::Red))
            } else {
                let color = chunk.biome.color();
                (
                    " ",
                    Style::new().bg(ratatui::style::Color::Rgb(color[0], color[1], color[2])),
                )
            };

            let cell = buf.cell_mut(Position::new(x as u16 + area.x, y as u16 + area.y));
            if let Some(c) = cell {
                c.set_symbol(symbol);
                c.set_style(style);
            }
            let cell = buf.cell_mut(Position::new(
                (x + 1).try_into().unwrap_or_default(),
                y.try_into().unwrap_or_default(),
            ));
            if let Some(c) = cell {
                c.set_symbol(symbol);
                c.set_style(style);
            }
        }
    }
}

use crate::app::{App, MapMode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect},
    style::Style,
    widgets::{Block, Clear, Paragraph, Widget},
    Frame,
};

/// Renders the user interface widgets.
#[allow(clippy::cast_possible_truncation)]
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let [info, _] = Layout::horizontal(Constraint::from_percentages([15, 100 - 15])).areas(area);
    let [fps, info] = Layout::vertical(Constraint::from_lengths([1 + 2, 6 + 2])).areas(info);
    draw_map(app, frame.buffer_mut(), area);

    let _ = app.fps_counter.render_tick();

    Clear.render(info, frame.buffer_mut());
    Clear.render(fps, frame.buffer_mut());

    app.fps_counter
        .to_paragraph()
        .style(Style::reset())
        .block(Block::bordered())
        .render(fps, frame.buffer_mut());

    let position = ((app.position.0) as isize, (app.position.1) as isize);
    let chunk_position = app.map.chunk_coords_from_world_coords(app.position);

    Paragraph::new(vec![
        format!("Seed: {}", app.map.seed()).into(),
        format!(
            "Current biome: {:?}",
            app.map.get_concrete_cell(position).biome
        )
        .into(),
        format!("Position: {}, {}", position.0, position.1,).into(),
        format!("Chunk position: {}, {}", chunk_position.0, chunk_position.1,).into(),
        format!("Scale: {:?}", app.map_mode).into(),
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
            let (style, symbol) = match app.map_mode {
                MapMode::Global(s) => {
                    let x_map = (app.position.0 / s) as isize + x / 2 - quarter_width;
                    let y_map = (app.position.1 / s) as isize - y + half_height;

                    let cell = app.map.get_intermediate_cell((x_map, y_map), s).clone();
                    let color = cell.biome.color();
                    let mut style =
                        Style::new().bg(ratatui::style::Color::Rgb(color[0], color[1], color[2]));
                    let symbol = if (x_map, y_map)
                        == ((app.position.0 / s) as isize, (app.position.1 / s) as isize)
                    {
                        style = style.fg(ratatui::style::Color::Red);
                        "@".to_string()
                    } else {
                        " ".to_string()
                    };
                    (style, symbol)
                }
                MapMode::Local => {
                    let x_map = (app.position.0) as isize + x / 2 - quarter_width;
                    let y_map = (app.position.1) as isize - y + half_height;

                    let cell = app.map.get_concrete_cell((x_map, y_map)).clone();
                    let color = cell.biome.color();
                    let mut style =
                        Style::new().bg(ratatui::style::Color::Rgb(color[0], color[1], color[2]));
                    let symbol = if (x_map, y_map)
                        == ((app.position.0) as isize, (app.position.1) as isize)
                    {
                        style = style.fg(ratatui::style::Color::Red);
                        "@".to_string()
                    } else {
                        match cell.building_part {
                            Some(part) => part.to_string(),

                            None => " ".to_string(),
                        }
                    };
                    (style, symbol)
                }
            };

            let cell = buf.cell_mut(Position::new(x as u16 + area.x, y as u16 + area.y));
            if let Some(c) = cell {
                c.set_symbol(&symbol);
                c.set_style(style);
            }
            let cell = buf.cell_mut(Position::new(
                (x + 1).try_into().unwrap_or_default(),
                y.try_into().unwrap_or_default(),
            ));
            if let Some(c) = cell {
                c.set_symbol(&symbol);
                c.set_style(style);
            }
        }
    }
}

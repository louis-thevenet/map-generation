use crate::app::App;
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

    let real_position = (
        (app.position.0 / app.current_scale) as isize,
        (app.position.1 / app.current_scale) as isize,
    );
    Paragraph::new(vec![
        format!("Seed: {}", app.map.seed()).into(),
        format!("Current biome: {:?}", app.map.get_cell(real_position).biome).into(),
        format!("position: {}, {}", real_position.0, real_position.1,).into(),
        format!("Scale: {:.2}", app.current_scale).into(),
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
            let x_map = (app.position.0) as isize + x / 2 - quarter_width;
            let y_map = (app.position.1) as isize - y + half_height;

            let cell = &app.map.get_cell((x_map, y_map));
            let color = cell.biome.color();
            let mut style =
                Style::new().bg(ratatui::style::Color::Rgb(color[0], color[1], color[2]));
            let symbol = if (x_map, y_map) == ((app.position.0) as isize, (app.position.1) as isize)
            {
                style = style.fg(ratatui::style::Color::Red);
                "@"
            } else {
                " "
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

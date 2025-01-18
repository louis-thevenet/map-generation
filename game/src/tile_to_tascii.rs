use std::collections::HashMap;

use game_core::tile::TileType;
use ratatui::style::Style;

pub type TileAsciiMapping = HashMap<TileType, (String, Style)>;

#[must_use]
pub fn default_tile_ascii_mapping() -> TileAsciiMapping {
    let mut symbols = HashMap::new();
    symbols.insert(
        game_core::tile::TileType::Water,
        ("≈".into(), Style::new().fg(ratatui::style::Color::Blue)),
    );
    symbols.insert(
        game_core::tile::TileType::Beach,
        ("░".into(), Style::new().fg(ratatui::style::Color::Yellow)),
    );
    symbols.insert(
        game_core::tile::TileType::Land,
        ("█".into(), Style::new().fg(ratatui::style::Color::Green)),
    );
    symbols.insert(
        game_core::tile::TileType::Mountain,
        ("^".into(), Style::new().fg(ratatui::style::Color::White)),
    );
    symbols
}

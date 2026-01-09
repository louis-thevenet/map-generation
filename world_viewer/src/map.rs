use std::collections::HashMap;

use world_gen::{cell::Cell, WorldGen};
pub const CHUNK_SIZE: f64 = 16.;

#[derive(Debug)]
pub struct Map {
    /// The world generator.
    generator: WorldGen,
    /// The generated cells.
    cells: HashMap<(isize, isize), Cell>,
}

impl Map {
    #[must_use]
    pub fn new(global_scale: f64) -> Self {
        Self {
            generator: WorldGen::new(global_scale, None),
            cells: HashMap::new(),
        }
    }
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.generator.seed
    }

    /// Get chunk coordinates from world coordinates.
    #[must_use]
    pub fn chunk_coords_from_world_coords(&self, position: (f64, f64)) -> (f64, f64) {
        let chunk_size = CHUNK_SIZE;
        let x = if position.0 >= 0. {
            position.0 / chunk_size
        } else {
            (position.0 + 1.) / chunk_size - 1.
        };
        let y = if position.1 >= 0. {
            position.1 / chunk_size
        } else {
            (position.1 + 1.) / chunk_size - 1.
        };
        (x, y)
    }

    /// Get a reference to a `Cell` from its coordinates.
    /// If the cell is not generated, it will be generated and cached.
    ///
    /// # Panics
    ///
    /// Shouldn't panic but clippy is screaming at me.
    pub fn get_cell(&mut self, pos: (isize, isize)) -> &Cell {
        if let std::collections::hash_map::Entry::Vacant(e) = self.cells.entry(pos) {
            e.insert(self.generator.generate_cell(pos));
            self.get_cell(pos)
        } else {
            self.cells.get(&pos).unwrap()
        }
    }
    /// Same as `get_cell` but taskes a scale parameter to control the level of detail.
    /// Will not cache the result.
    pub fn get_chunk(&mut self, scale: f64, pos: (isize, isize)) -> Cell {
        self.generator.generate_cell_scaled(scale, pos)
    }

    /// Check if a cell has already been generated.
    #[must_use]
    pub fn is_generated(&self, pos: (isize, isize)) -> bool {
        self.cells.contains_key(&pos)
    }

    /// Get the number of cached cells.
    #[must_use]
    pub fn generated_cell_count(&self) -> usize {
        self.cells.len()
    }
}

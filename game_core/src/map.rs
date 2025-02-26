use std::collections::HashMap;

use world_gen::{concrete_cell::ConcreteCell, intermediate_cell::IntermediateCell, WorldGen};
pub const CHUNK_SIZE: f64 = 256.;

#[derive(Debug)]
pub struct Map {
    /// The world generator.
    generator: WorldGen,
    /// The generated cells.
    cells: HashMap<(isize, isize), ConcreteCell>,
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

    /// Get the concrete cell at the given position. If the cell has not been generated yet, it will be generated.
    /// The cell will be cached for future use.
    /// # Panics
    /// Should not panic.
    pub fn get_concrete_cell(&mut self, pos: (isize, isize)) -> &ConcreteCell {
        let chunk_pos = self.chunk_coords_from_world_coords((pos.0 as f64, pos.1 as f64));
        if let std::collections::hash_map::Entry::Vacant(e) = self.cells.entry(pos) {
            e.insert(
                self.generator
                    .generate_concrete_cell(pos, (chunk_pos.0 as isize, chunk_pos.1 as isize)),
            );
            self.get_concrete_cell(pos)
        } else {
            self.cells.get(&pos).expect("Shouldn't happen")
        }
    }

    /// Get the intermediate cell at the given position. The cell will not be cached.
    #[must_use]
    pub fn get_intermediate_cell(&self, pos: (isize, isize), scale: f64) -> IntermediateCell {
        self.generator.generate_intermediate_cell(pos, scale)
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

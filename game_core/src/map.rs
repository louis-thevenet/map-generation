use std::collections::HashMap;

use world_gen::{cell::Cell, WorldGen};

#[derive(Debug)]
pub struct Map {
    generator: WorldGen,
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

    pub fn update_scale(&mut self, global_scale: f64) {
        self.cells.clear();
        self.generator.update_scale(global_scale);
    }
    /// Get a reference to a `Cell` from its coordinates.
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
    #[must_use]
    pub fn is_generated(&self, pos: (isize, isize)) -> bool {
        self.cells.contains_key(&pos)
    }

    #[must_use]
    pub fn generated_cell_count(&self) -> usize {
        self.cells.len()
    }
}

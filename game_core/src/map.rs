use std::collections::HashMap;

use world_gen::{cell::Cell, WorldGen};

#[derive(Debug)]
pub struct Map {
    generator: WorldGen,
    chunks: HashMap<(isize, isize), Cell>,
}

impl Map {
    #[must_use]
    pub fn new(global_scale: f64) -> Self {
        Self {
            generator: WorldGen::new(global_scale, None),
            chunks: HashMap::new(),
        }
    }

    pub fn update_scale(&mut self, global_scale: f64) {
        self.chunks.clear();
        self.generator.update_scale(global_scale);
    }
    /// Get a reference to a `Chunk` from its coordinates.
    ///
    /// # Panics
    ///
    /// Shouldn't panic but clippy is screaming at me.
    pub fn get_chunk_from_chunk_coord(&mut self, pos: (isize, isize)) -> &Cell {
        if let std::collections::hash_map::Entry::Vacant(e) = self.chunks.entry(pos) {
            e.insert(self.generator.generate_chunk(pos));
            self.get_chunk_from_chunk_coord(pos)
        } else {
            self.chunks.get(&pos).unwrap()
        }
    }
    pub fn get_chunk_from_world_coord(&mut self, position: (isize, isize)) -> &Cell {
        self.get_chunk_from_chunk_coord(self.chunk_coord_from_world_coord(position))
    }
    #[must_use]
    pub fn is_generated(&self, pos: (isize, isize)) -> bool {
        self.chunks.contains_key(&pos)
    }
    // #[must_use]
    // pub fn get_chunk_size(&self) -> isize {
    //     // It's always used as an isize in computations
    //     self.generator.chunk_size as isize
    // }
    // pub fn preload_radius(&mut self, position: (isize, isize), preload_chunks_radius: isize) {
    //     for y in (-preload_chunks_radius)..=(preload_chunks_radius) {
    //         for x in (-preload_chunks_radius)..=(preload_chunks_radius) {
    //             if (x - position.0) * (x - position.0) + (y - position.1) * (y - position.1)
    //                 > preload_chunks_radius * preload_chunks_radius
    //             {
    //                 continue;
    //             }
    //             let x = x + position.0 * self.get_chunk_size();
    //             let y = y + position.1 * self.get_chunk_size();
    //             if !self.is_generated((x, y)) {
    //                 self.get_chunk_from_chunk_coord((x, y));
    //             }
    //         }
    //     }
    // }
    #[must_use]
    pub fn generated_chunk_count(&self) -> usize {
        self.chunks.len()
    }
    // pub fn get_tile(&mut self, position: (isize, isize)) -> Tile {
    //     // Chunk coordinates
    //     let chunk_size = self.get_chunk_size();
    //     let (chunk_x, chunk_y) = self.chunk_coord_from_world_coord(position);

    //     let cell_x = {
    //         let cx = position.0 % chunk_size;
    //         (cx + chunk_size) % chunk_size
    //     }
    //     .unsigned_abs();
    //     let cell_y = {
    //         let cy = position.1 % chunk_size;
    //         (cy + chunk_size) % chunk_size
    //     }
    //     .unsigned_abs();

    //     self.get_chunk_from_chunk_coord((chunk_x, chunk_y)).tiles[cell_y][cell_x].clone()
    // }
    #[must_use]
    pub fn chunk_coord_from_world_coord(&self, position: (isize, isize)) -> (isize, isize) {
        let chunk_size = self.generator.chunk_size;
        let x = if position.0 >= 0 {
            position.0 / chunk_size
        } else {
            (position.0 + 1) / chunk_size - 1
        };
        let y = if position.1 >= 0 {
            position.1 / chunk_size
        } else {
            (position.1 + 1) / chunk_size - 1
        };
        (x, y)
    }

    #[must_use]
    pub fn get_chunk_size(&self) -> isize {
        self.generator.chunk_size
    }
}

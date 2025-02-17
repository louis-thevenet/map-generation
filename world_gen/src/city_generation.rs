use std::{
    collections::{HashMap, HashSet},
    f32::consts::SQRT_2,
    ops::Range,
};

use pathfinding::prelude::astar;
use progressing::{mapping, Baring};
use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]

pub struct Building {
    pub door: (i32, i32),
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl Building {
    fn overlaps(&self, other: &Building, offset: i32) -> bool {
        self.x - offset < other.x + other.width
            && self.x + self.width + offset > other.x
            && self.y - offset < other.y + other.height
            && self.y + self.height + offset > other.y
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

pub struct City {
    pub buildings: HashMap<(i32, i32), Building>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,

    pub roads: Vec<Vec<(i32, i32)>>,
    is_road: HashSet<(i32, i32)>,

    width: i32,
    height: i32,

    width_bound: Range<i32>,
    height_bound: Range<i32>,
    distance_bound: Range<i32>,
}

impl City {
    pub fn new(
        width_bound: Range<i32>,
        height_bound: Range<i32>,
        distance_bound: Range<i32>,
        width: i32,
        height: i32,
    ) -> Self {
        Self {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,

            buildings: HashMap::new(),
            is_road: HashSet::new(),
            roads: vec![],

            width_bound,
            height_bound,
            distance_bound,
            width,
            height,
        }
    }

    pub fn generate_buildings(&mut self, n: usize) {
        for _ in 0..n {
            if self.buildings.is_empty() {
                let width = thread_rng().gen_range(self.width_bound.clone());
                let height = thread_rng().gen_range(self.height_bound.clone());

                let x = self.width / 2 - width / 2;
                let y = self.height / 2 - height / 2;

                let (door_x, door_y) = if thread_rng().gen_bool(0.5) {
                    // on northern or southern side
                    if thread_rng().gen_bool(0.5) {
                        // northern side
                        (thread_rng().gen_range(x..x + width), y)
                    } else {
                        // southern side
                        (thread_rng().gen_range(x..x + width), y + height)
                    }
                } else {
                    // on eastern or western side
                    if thread_rng().gen_bool(0.5) {
                        // eastern side
                        (x + width, thread_rng().gen_range(y..y + height))
                    } else {
                        // western side
                        (x, thread_rng().gen_range(y..y + height))
                    }
                };
                self.buildings.insert(
                    (x, y),
                    Building {
                        door: (door_x, door_y),

                        x,
                        y,
                        width,
                        height,
                    },
                );
            } else {
                let index = thread_rng().gen_range(0..self.buildings.len());
                let Building {
                    door: _,

                    x,
                    y,
                    width,
                    height,
                } = self.buildings.values().nth(index).unwrap();

                let x_center = x + width / 2;
                let y_center = y + height / 2;

                let distance_x = thread_rng().gen_range(self.distance_bound.clone());
                let distance_y = thread_rng().gen_range(self.distance_bound.clone());

                let spawn_x = if thread_rng().gen_bool(0.5) {
                    x_center + distance_x
                } else {
                    x_center - distance_x
                };

                let spawn_y = if thread_rng().gen_bool(0.5) {
                    y_center + distance_y
                } else {
                    y_center - distance_y
                };

                let width = thread_rng().gen_range(self.width_bound.clone());
                let height = thread_rng().gen_range(self.height_bound.clone());

                let (door_x, door_y) = if thread_rng().gen_bool(0.5) {
                    // on northern or southern side
                    if thread_rng().gen_bool(0.5) {
                        // northern side
                        (thread_rng().gen_range(spawn_x..spawn_x + width), spawn_y)
                    } else {
                        // southern side
                        (
                            thread_rng().gen_range(spawn_x..spawn_x + width),
                            spawn_y + height,
                        )
                    }
                } else {
                    // on eastern or western side
                    if thread_rng().gen_bool(0.5) {
                        // eastern side
                        (
                            spawn_x + width,
                            thread_rng().gen_range(spawn_y..spawn_y + height),
                        )
                    } else {
                        // western side
                        (spawn_x, thread_rng().gen_range(spawn_y..spawn_y + height))
                    }
                };

                let v = Building {
                    door: (door_x, door_y),
                    x: spawn_x,
                    y: spawn_y,
                    width,
                    height,
                };
                // check overlap
                if self
                    .buildings
                    .values()
                    .any(|building| building.overlaps(&v, 3))
                {
                    continue;
                };
                self.buildings.insert((spawn_x, spawn_y), v);
            }
        }

        self.min_x = self.buildings.values().map(|b| b.x).min().unwrap() - 20;
        self.min_y = self.buildings.values().map(|b| b.y).min().unwrap() - 20;

        self.max_x = self
            .buildings
            .values()
            .map(|b| b.x + b.width)
            .max()
            .unwrap()
            + 20;
        self.max_y = self
            .buildings
            .values()
            .map(|b| b.y + b.height)
            .max()
            .unwrap()
            + 20;
    }
    fn successors(&self, p: (i32, i32)) -> Vec<((i32, i32), i32)> {
        let (x, y) = p;

        let mut successors = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0
                    || (x + i < self.min_x
                        || x + i >= self.max_x
                        || y + j < self.min_y
                        || y + j >= self.max_y)
                {
                    continue;
                }

                if self
                    .buildings
                    .values()
                    .any(|b| b.door != (x + i, y + j) && b.contains(x + i, y + j))
                {
                    continue;
                }

                let score = if self.is_road.contains(&(x + i, y + j)) {
                    1
                } else {
                    5
                } * (if i != 0 && j != 0 { SQRT_2 * 10. } else { 10.0 }) as i32
                    / 10;

                successors.push(((x + i, y + j), score));
            }
        }

        successors
    }

    pub fn generate_roads(&mut self) {
        let buildings = self.buildings.values().collect::<Vec<&Building>>().clone();
        let mut progress_bar =
            mapping::Bar::with_range(0, buildings.len() * (buildings.len() - 1) / 2).timed();
        progress_bar.set_len(20);
        let mut total = 0;
        for i in 0..buildings.len() {
            for j in (i + 1)..buildings.len() {
                total += 1;
                progress_bar.set(total as usize);

                let b1 = &buildings[i];
                let b2 = &buildings[j];

                let (x1, y1) = b1.door;
                let (x2, y2) = b2.door;

                let distance = (x1 - x2).abs() + (y1 - y2).abs();
                if distance > 200 {
                    continue;
                }

                let res = astar(
                    &(x2, y2),
                    |&p| self.successors(p),
                    |&p| {
                        let (x, y) = p;
                        (x - x1).abs() + (y - y1).abs()
                    },
                    |&p| p == (x1, y1),
                );
                match res {
                    Some((path, _)) => {
                        for (x, y) in path.clone() {
                            self.is_road.insert((x, y));
                        }
                        self.roads.push(path);
                    }
                    None => {
                        println!("No path {b1:?} and {b2:?}");
                    }
                }

                print!("\r{progress_bar}");
            }
        }
    }
}

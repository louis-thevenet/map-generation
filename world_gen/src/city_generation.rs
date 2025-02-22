use std::{collections::HashMap, ops::Range};

use pathfinding::prelude::astar;
use rand::{seq::IteratorRandom, thread_rng, Rng};
enum CellType {
    Road,
    Building,
}

/// Represents a building in the city
/// x, y are the coordinates of the top left corner of the building
/// width, height are the dimensions of the building
/// door is the coordinates of the door of the building
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Building {
    pub door: (i32, i32),
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_important: bool,
    pub id: usize,
}
impl Building {
    /// Check if two buildings overlap
    fn overlaps(&self, other: &Building, offset: i32) -> bool {
        self.x - offset < other.x + other.width
            && self.x + self.width + offset > other.x
            && self.y - offset < other.y + other.height
            && self.y + self.height + offset > other.y
    }
    /// Check if a point is inside the building
    fn contains(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;

        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Create a building from a rectangle and randomizes the door
    fn with_random_door(x: i32, y: i32, width: i32, height: i32, id: usize) -> Self {
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
        Self {
            is_important: false,
            door: (door_x, door_y),
            x,
            y,
            width,
            height,
            id,
        }
    }
    fn make_important(self) -> Self {
        Self {
            is_important: true,
            ..self
        }
    }

    // fn with_door_direction(
    //     x: i32,
    //     y: i32,
    //     width: i32,
    //     height: i32,
    //     door_direction: (i32, i32),
    //     id: usize,
    // ) -> Building {
    //     let angle = (door_direction.1 as f32).atan2(door_direction.0 as f32);
    //     // door placement is on intersection of the building and the line

    //     let (door_x, door_y) = if angle < PI / 4.0 && angle > -PI / 4.0 {
    //         // door is on the right
    //         (x + width, y + height / 2)
    //     } else if angle > PI / 4.0 && angle < 3.0 * PI / 4.0 {
    //         // door is on the bottom
    //         (x + width / 2, y + height)
    //     } else if !(-3.0 * PI / 4.0..=3.0 * PI / 4.0).contains(&angle) {
    //         // door is on the left
    //         (x, y + height / 2)
    //     } else {
    //         // door is on the top
    //         (x + width / 2, y)
    //     };

    //     Self {
    //         is_important: false,
    //         door: (door_x, door_y),
    //         x,
    //         y,
    //         width,
    //         height,
    //         id,
    //     }
    // }
}

/// Random city generator
pub struct CityGenerator {
    /// Buildings of the city
    pub buildings: HashMap<(i32, i32), Building>,
    /// Buildings of the city
    pub important_buildings: Vec<(i32, i32)>,

    /// Roads of the city
    pub roads: Vec<Vec<(i32, i32)>>,
    /// x coordinate of the leftmost building
    pub min_x: i32,
    /// y coordinate of the topmost building
    pub min_y: i32,
    /// x coordinate of the rightmost building
    pub max_x: i32,
    /// y coordinate of the bottommost building
    pub max_y: i32,
    /// Lets us know if a point is not free
    is_something: HashMap<(i32, i32), CellType>,
    /// Min and max width of the buildings
    width_bound: Range<i32>,
    /// Min and max height of the buildings
    height_bound: Range<i32>,
    /// Min and max distance between buildings
    distance_bound: Range<i32>,
    /// Max distance between important buildings
    important_buildings_max_distance: i32,
}

impl CityGenerator {
    #[must_use]
    pub fn new(
        width_bound: Range<i32>,
        height_bound: Range<i32>,
        distance_bound: Range<i32>,
        important_buildings_max_distance: i32,
    ) -> Self {
        Self {
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,

            buildings: HashMap::new(),
            important_buildings: vec![],

            is_something: HashMap::new(),
            roads: vec![],

            width_bound,
            height_bound,
            distance_bound,
            important_buildings_max_distance,
        }
    }
    pub fn generate(&mut self, normal_buildings: usize, important_buildings: usize) {
        println!("Generating important buildings");
        self.generate_important_buildings(important_buildings);
        println!("Generating normal buildings");
        self.generate_buildings(normal_buildings);
    }
    fn generate_important_buildings(&mut self, n: usize) {
        // generate the important buildings with a smaller scale
        let scale_factor = 10;

        for _ in 0..n {
            // New building
            let b1 = self.generate_random_important_building(scale_factor);
            // Register the building in the map
            for x in b1.x..=b1.x + b1.width {
                for y in b1.y..=b1.y + b1.height {
                    self.is_something.insert((x, y), CellType::Building);
                }
            }
            // Keep door free to go through
            self.is_something.remove(&b1.door);

            // Keep track of the important building
            self.important_buildings.push((b1.x, b1.y));
            self.update_borders_from_new_building(&b1);
            self.buildings.insert((b1.x, b1.y), b1);
        }
        let buildings = self.buildings.values().collect::<Vec<&Building>>(); // We'll iterate over the buildings

        for &b1 in &buildings {
            // let b2 = buildings
            //     .iter()
            //     .filter(|&&b2| b1 != b2)
            //     .choose(&mut thread_rng())
            //     .unwrap(); // retry instead of filter is better

            for b2 in self.buildings.values() {
                if b1 == b2 {
                    continue;
                }

                let road = if let Some((road, _)) = self.generate_road(b1, b2) {
                    road
                } else {
                    vec![]
                };
                for (x, y) in &road {
                    self.is_something.insert((*x, *y), CellType::Road);
                }
                self.roads.push(road);
            }
        }
        // Now, we will update everywthing to scale, so multiply everything by the scale factor
        if scale_factor > 1 {
            self.is_something.clear();
            self.min_x *= scale_factor;
            self.min_y *= scale_factor;
            self.max_x *= scale_factor;
            self.max_y *= scale_factor;

            for building in self.buildings.values_mut() {
                building.x *= scale_factor;
                building.y *= scale_factor;
                building.width *= scale_factor;
                building.height *= scale_factor;

                building.door.0 *= scale_factor;
                building.door.1 *= scale_factor;

                for x in building.x..=building.x + building.width {
                    for y in building.y..=building.y + building.height {
                        self.is_something.insert((x, y), CellType::Building);
                    }
                }
                self.is_something.remove(&building.door);
            }
            for road in &mut self.roads {
                let mut scaled_road = vec![];
                for i in 0..road.len() - 1 {
                    let mut direction = (road[i + 1].0 - road[i].0, road[i + 1].1 - road[i].1);
                    direction = (
                        if direction.0 == 0 {
                            0
                        } else {
                            direction.0 / direction.0.abs()
                        },
                        if direction.1 == 0 {
                            0
                        } else {
                            direction.1 / direction.1.abs()
                        },
                    );

                    let mut position = (road[i].0 * scale_factor, road[i].1 * scale_factor);
                    for _i in 0..scale_factor {
                        scaled_road.push(position);
                        self.is_something.insert(position, CellType::Road);
                        position = (position.0 + direction.0, position.1 + direction.1);
                    }
                }
                *road = scaled_road;
            }
        }
    }
    /// Generate a random important building
    fn generate_random_important_building(&mut self, scale_factor: i32) -> Building {
        let (x, y) = (
            thread_rng().gen_range(
                -(self.important_buildings_max_distance / (scale_factor * 2))
                    ..(self.important_buildings_max_distance / (scale_factor * 2)),
            ),
            thread_rng().gen_range(
                (-self.important_buildings_max_distance / (scale_factor * 2))
                    ..(self.important_buildings_max_distance / (scale_factor * 2)),
            ),
        );
        let width =
            (thread_rng().gen_range(self.width_bound.clone()) + scale_factor) / scale_factor;
        let height =
            (thread_rng().gen_range(self.height_bound.clone()) + scale_factor) / scale_factor;

        let building = Building::with_random_door(x, y, width, height, 0).make_important();
        if self.buildings.values().any(|b| b.overlaps(&building, 3)) {
            self.generate_random_important_building(scale_factor)
        } else {
            building
        }
    }
    fn generate_buildings(&mut self, mut n: usize) {
        let init_n = n as f32;
        while n > 0 {
            let Building {
                door: _,
                is_important: _,
                x,
                y,
                width,
                height,
                id: _,
            } = { self.buildings.values().choose(&mut thread_rng()).unwrap() };
            let x_center = x + width / 2;
            let y_center = y + height / 2;

            // let distance_x = thread_rng().gen_range(self.distance_bound.clone());
            // let distance_y = thread_rng().gen_range(self.distance_bound.clone());

            let distance_x = ((self.distance_bound.end - self.distance_bound.start) as f32
                * n as f32
                / init_n) as i32
                + self.distance_bound.start;

            let distance_y = ((self.distance_bound.end - self.distance_bound.start) as f32
                * (n as f32 / init_n)) as i32
                + self.distance_bound.start;

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

            let offset = 8; // minimum distance between buildings
            let new_building = Building::with_random_door(spawn_x, spawn_y, width, height, n);
            let overlaps =
                        // seems inefficient but it's A* that's the bottleneck
                            self
                            .buildings
                            .iter()
                            .any(|(_, b)| b.overlaps(&new_building, offset) && b != &new_building)
                            // it's okay to only check on building walls and not inside

                            || (spawn_x..=spawn_x + width)
                                .any(|x| self.is_something.contains_key(&(x, spawn_y)))

                            || (spawn_x..=spawn_x + width
            )                    .any(|x| self.is_something.contains_key(&(x, spawn_y + height)))

                            || (spawn_y..=spawn_y + height)
                                .any(|y| self.is_something.contains_key(&(spawn_x, y)))

                            ||( spawn_y..=spawn_y + height
            )                    .any(|y| self.is_something.contains_key(&(spawn_x + width, y)))

                            ;

            if !overlaps {
                let closest_important_building = self
                    .buildings
                    .get(
                        self.important_buildings
                            .iter()
                            .min_by_key(|(x, y)| (x - spawn_x).abs() + (y - spawn_y).abs())
                            .unwrap(),
                    )
                    .unwrap();

                for x in spawn_x..=spawn_x + width {
                    for y in spawn_y..=spawn_y + height {
                        self.is_something.insert((x, y), CellType::Building);
                    }
                }
                self.is_something.remove(&(*x, *y));

                let road = if let Some((road, _)) =
                    self.generate_road(&new_building, closest_important_building)
                {
                    road
                } else {
                    println!(
                        "No road found between {closest_important_building:?} and {spawn_x},{spawn_y}"
                    );

                    vec![]
                };
                for (x, y) in &road {
                    self.is_something.insert((*x, *y), CellType::Road);
                }
                self.update_borders_from_new_building(&new_building);
                self.buildings.insert((spawn_x, spawn_y), new_building);
                self.roads.push(road);

                n -= 1;
            }
        }
    }

    // fn update_borders(&mut self) {
    //     self.min_x = self.buildings.values().map(|b| b.x).min().unwrap() - 20;
    //     self.min_y = self.buildings.values().map(|b| b.y).min().unwrap() - 20;

    //     self.max_x = self
    //         .buildings
    //         .values()
    //         .map(|b| b.x + b.width)
    //         .max()
    //         .unwrap()
    //         + 20;
    //     self.max_y = self
    //         .buildings
    //         .values()
    //         .map(|b| b.y + b.height)
    //         .max()
    //         .unwrap()
    //         + 20;
    // }
    fn update_borders_from_new_building(&mut self, building: &Building) {
        self.min_x = self.min_x.min(building.x - 20);
        self.min_y = self.min_y.min(building.y - 20);

        self.max_x = self.max_x.max(building.x + building.width + 20);
        self.max_y = self.max_y.max(building.y + building.height + 20);
    }

    fn successors(&self, p: (i32, i32)) -> Vec<((i32, i32), i32)> {
        let (x, y) = p;

        let mut successors = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                // forbid diagonal
                if i != 0 && j != 0 {
                    continue;
                }

                if i == 0 && j == 0
                    || (x + i < self.min_x
                        || x + i >= self.max_x
                        || y + j < self.min_y
                        || y + j >= self.max_y)
                {
                    continue;
                }

                let base_score = if i != 0 && j != 0 { 14 } else { 10 }; // if we go diagonally, the cost is sqrt(2)

                match self.is_something.get(&(x + i, y + j)) {
                    Some(CellType::Building) => match self.buildings.get(&(x + i, y + j)) {
                        Some(building) => {
                            // if we are in the door of the building, we can go through
                            if building.door == (x + i, y + j) {
                                successors.push(((x + i, y + j), base_score));
                            }
                        }
                        None => continue,
                    },
                    Some(CellType::Road) => successors.push(((x + i, y + j), base_score)),
                    None => successors.push(((x + i, y + j), base_score * 5)), // penalize going through nothing
                }
            }
        }

        successors
    }

    fn generate_road(&self, start: &Building, end: &Building) -> Option<(Vec<(i32, i32)>, i32)> {
        // let (x1, y1) = end.door;
        let (x2, y2) = start.door;
        astar(
            &(x2, y2),
            |&p| self.successors(p),
            |&p| {
                let (x, y) = p;
                ((((x - end.x + end.width).abs() + (y - end.y + end.height).abs()) * 10) as f64)
                    .sqrt() as i32
            },
            |&p| matches!(self.is_something.get(&p), Some(CellType::Road)) || end.contains(p),
        )
    }
}

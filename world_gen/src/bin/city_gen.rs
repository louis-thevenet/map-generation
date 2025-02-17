use std::fs;

use clap::Parser;
use image::{ImageBuffer, ImageResult, Rgb};
use world_gen::{city_generation::CityGenerator, image_utils::draw_rect};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    buildings: usize,
    #[arg(short, long)]
    min_distance_road: i32,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn city_generator(cli: &Cli) -> ImageResult<()> {
    let buildings = cli.buildings;
    let min_distance_road = cli.min_distance_road;

    let width = 512;
    let height = width;

    let mut city_gen = CityGenerator::new(8..30, 8..30, 20..30, width, height, min_distance_road);

    city_gen.generate_buildings(buildings);
    city_gen.generate_roads_astar();

    // city_gen.generate_roads_astar();
    let mut img = ImageBuffer::new(
        10 + (city_gen.max_x - city_gen.min_x) as u32,
        10 + (city_gen.max_y - city_gen.min_y) as u32,
    );
    println!(
        "size : {}x{}",
        city_gen.max_x - city_gen.min_x,
        city_gen.max_y - city_gen.min_y
    );

    // roads
    for road in &city_gen.roads {
        for (x, y) in road {
            img.put_pixel(
                *x as u32 - city_gen.min_x as u32,
                *y as u32 - city_gen.min_y as u32,
                Rgb([139, 69, 19]),
            );
        }
    }
    for building in city_gen.buildings.values() {
        draw_rect(
            &mut img,
            (
                building.x as u32 - city_gen.min_x as u32,
                building.y as u32 - city_gen.min_y as u32,
            ),
            building.width as u32,
            building.height as u32,
            Rgb([255, 255, 255]),
        );
        draw_rect(
            &mut img,
            (
                building.door.0 as u32 - city_gen.min_x as u32,
                building.door.1 as u32 - city_gen.min_y as u32,
            ),
            0,
            0,
            Rgb([255, 0, 0]),
        );
    }

    img.save("output/city.png")
}
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn main() -> ImageResult<()> {
    let cli = Cli::parse();
    fs::create_dir("output").unwrap_or_default();
    city_generator(&cli)
}

use std::env;

use image::{ImageBuffer, Rgb};
use progressing::{clamping, mapping, Baring};
use world_gen::{chunk::Chunk, WorldGen};

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn save_maps(size: (u32, u32), chunks: &[Vec<Chunk>]) {
    let mut temp_img = ImageBuffer::new(size.0, size.1);
    let mut moisture_img = ImageBuffer::new(size.0, size.1);
    let mut continentalness_img = ImageBuffer::new(size.0, size.1);
    let mut erosion_img = ImageBuffer::new(size.0, size.1);

    for (x, chunk_row) in chunks.iter().enumerate() {
        for (y, chunk) in chunk_row.iter().rev().enumerate() {
            let temp_color = (255. * (chunk.temp + 1.) / 2.) as u8;
            // warm = red, cold = blue
            let color = Rgb([temp_color, 0, 255 - temp_color]);
            temp_img.put_pixel(x as u32, y as u32, color);

            let moisture_color = (255. * (chunk.moisture + 1.) / 2.) as u8;
            // wet = blue, dry = red
            let color = Rgb([moisture_color, 0, 255 - moisture_color]);
            moisture_img.put_pixel(x as u32, y as u32, color);

            let continentalness_color = (255. * (chunk.continentalness + 1.) / 2.) as u8;
            // black = terrain, white = ocean
            let color = Rgb([
                continentalness_color,
                continentalness_color,
                continentalness_color,
            ]);
            continentalness_img.put_pixel(x as u32, y as u32, color);

            let erosion_color = (255. * (chunk.erosion + 1.) / 2.) as u8;
            // white = high erosion, black = low erosion

            let color = Rgb([erosion_color, erosion_color, erosion_color]);

            erosion_img.put_pixel(x as u32, y as u32, color);
        }
    }

    temp_img.save("temperature_map.png").unwrap();
    moisture_img.save("moisture_map.png").unwrap();
    continentalness_img.save("continentalness_map.png").unwrap();
    erosion_img.save("erosion_map.png").unwrap();
}

fn save_biome_map(size: (u32, u32), chunks: &[Vec<Chunk>]) {
    let mut biome_img = ImageBuffer::new(size.0, size.1);

    for (x, chunk_row) in chunks.iter().enumerate() {
        for (y, chunk) in chunk_row.iter().rev().enumerate() {
            let color = Rgb(chunk.biome.color());
            biome_img.put_pixel(x as u32, y as u32, color);
        }
    }

    biome_img.save("biome_map.png").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let width = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        1024
    };

    let height = width;
    let mut progress_bar = mapping::Bar::with_range(0, height * width).timed();
    progress_bar.set_len(20);

    let world_gen = WorldGen::default();
    let chunks = (-width / 2..width / 2)
        .map(|x| {
            progress_bar.set((x + width / 2) * height);

            print!("\r{}", progress_bar);
            (-height / 2..height / 2)
                .map(|y| world_gen.generate_chunk((x.try_into().unwrap(), y.try_into().unwrap())))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    save_maps((width as u32, height as u32), &chunks);
    save_biome_map((width as u32, height as u32), &chunks);
}

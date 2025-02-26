use clap::Parser;
use image::{ImageBuffer, ImageResult, Rgb};
use progressing::{mapping, Baring};
use world_gen::{image_utils::draw_rect, intermediate_cell::IntermediateCell, WorldGen};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    width: i64,
    #[arg(short, long)]
    scale: f64,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn save_intermediate_maps(size: (u32, u32), cells: &[Vec<IntermediateCell>]) {
    let mut temp_img = ImageBuffer::new(size.0, size.1);
    let mut moisture_img = ImageBuffer::new(size.0, size.1);
    let mut continentalness_img = ImageBuffer::new(size.0, size.1);
    let mut erosion_img = ImageBuffer::new(size.0, size.1);

    for (x, cell_row) in cells.iter().enumerate() {
        for (y, cell) in cell_row.iter().rev().enumerate() {
            let temp_color = (255. * (cell.temp + 1.) / 2.) as u8;
            // warm = red, cold = blue
            let color = Rgb([temp_color, 0, 255 - temp_color]);
            temp_img.put_pixel(x as u32, y as u32, color);

            let moisture_color = (255. * (cell.moisture + 1.) / 2.) as u8;
            // wet = blue, dry = red
            let color = Rgb([moisture_color, 0, 255 - moisture_color]);
            moisture_img.put_pixel(x as u32, y as u32, color);

            let continentalness_color = (255. * (cell.continentalness + 1.) / 2.) as u8;
            // black = terrain, white = ocean
            let color = Rgb([
                continentalness_color,
                continentalness_color,
                continentalness_color,
            ]);
            continentalness_img.put_pixel(x as u32, y as u32, color);

            let erosion_color = (255. * (cell.erosion + 1.) / 2.) as u8;
            // white = high erosion, black = low erosion

            let color = Rgb([erosion_color, erosion_color, erosion_color]);

            erosion_img.put_pixel(x as u32, y as u32, color);
        }
    }

    temp_img.save("output/temperature_map.png").unwrap();
    moisture_img.save("output/moisture_map.png").unwrap();
    continentalness_img
        .save("output/continentalness_map.png")
        .unwrap();
    erosion_img.save("output/erosion_map.png").unwrap();
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn save_biome_map(biome_img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, cells: &[Vec<IntermediateCell>]) {
    for (x, cell_row) in cells.iter().enumerate() {
        for (y, cell) in cell_row.iter().rev().enumerate() {
            let color = Rgb(cell.biome.color());
            biome_img.put_pixel(x as u32, y as u32, color);
        }
    }
}
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn biome_generator(cli: &Cli) -> ImageResult<()> {
    let width = cli.width;
    let scale = cli.scale;
    let height = width;
    let mut progress_bar = mapping::Bar::with_range(0, height * width).timed();
    progress_bar.set_len(20);

    let world_gen = WorldGen::new(scale, None);
    let cells = (-width / 2..width / 2)
        .map(|x| {
            progress_bar.set((x + width / 2) * height);

            print!("\r{progress_bar}");
            (-height / 2..height / 2)
                .map(|y| {
                    world_gen.generate_intermediate_cell(
                        (x.try_into().unwrap(), y.try_into().unwrap()),
                        scale,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Noise maps
    save_intermediate_maps((width as u32, height as u32), &cells);

    // Biome map
    let mut biome_img = ImageBuffer::new(width as u32, height as u32);
    save_biome_map(&mut biome_img, &cells);
    draw_rect(
        &mut biome_img,
        (width as u32 / 2 - 1, height as u32 / 2 - 1),
        1,
        1,
        Rgb([255, 0, 0]),
    );
    biome_img.save("output/biome_map.png")
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn main() -> ImageResult<()> {
    let cli = Cli::parse();
    std::fs::create_dir("output").unwrap_or_default();
    biome_generator(&cli)
}

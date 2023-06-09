use image::imageops::{blur, crop_imm};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::env;
use std::fs::remove_file;
use std::path::Path;

fn save_updated_image(
    converted_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    img_file: &mut DynamicImage,
) {
    converted_image
        .save("./drafted_image.jpeg")
        .expect("Cannot save image");

    *img_file = image::open("./drafted_image.jpeg").expect("Failed to open image");

    remove_file("./drafted_image.jpeg").expect("Cannot delete image");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let image_path: &Path = Path::new(&args[1]);

    let mut img_file = image::open(&image_path).expect("Failed to open image");

    let commands = &args[2..args.len()];

    for i in 0..commands.len() {
        match commands[i].as_str() {
            "cut" => {
                let s = &commands[i + 1].parse::<u32>().is_ok();
                let s1 = &commands[i + 2].parse::<u32>().is_ok();

                if s & s1 {
                    let changed_image = crop_imm(
                        &img_file,
                        0,
                        0,
                        commands[i + 1].parse::<u32>().unwrap(),
                        commands[i + 2].parse::<u32>().unwrap(),
                    );

                    let converted_image = changed_image.to_image();

                    save_updated_image(converted_image, &mut img_file);
                }
            }
            "blur" => {
                let updated_image = blur(&img_file, 10.0);
                save_updated_image(updated_image, &mut img_file);
            }
            _ => (),
        }
    }

    let image_name = image_path.file_name().unwrap().to_str().unwrap();
    let new_image_name = String::from("./new_") + image_name;

    img_file.save(new_image_name).expect("Cannot save image");
}

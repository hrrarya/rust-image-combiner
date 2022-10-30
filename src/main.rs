mod args;
// mod human;
use args::Args;
use image::{ io::Reader ,DynamicImage, ImageFormat };
use std::{io::BufReader, fs::File};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormat,
}
fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_1, image_format_1) = get_image_from_path(args.image_1);
    let (image_2, image_format_2) = get_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormat);
    }
    Ok(())
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2:(u32,u32)) -> (u32,u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn get_image_from_path(path: String)-> (DynamicImage, ImageFormat) {
    let image_reader:Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image:DynamicImage = image_reader.decode().unwrap();
    return (image, image_format);
}
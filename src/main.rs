mod args;
// mod human;
use args::Args;
use image::{ io::Reader ,DynamicImage, ImageFormat, GenericImageView,imageops::FilterType::Triangle };
use std::{io::BufReader, fs::File};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormat,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer =Vec::with_capacity(buffer_capacity.try_into().unwrap());

        FloatingImage {
            width,
            height,
            data:buffer,
            name,
        }
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image_1, image_format_1) = get_image_from_path(args.image_1);
    let (image_2, image_format_2) = get_image_from_path(args.image_2);

    if image_format_1 != image_format_2 {
        return Err(ImageDataErrors::DifferentImageFormat);
    }
    let (_image1, _image2) = standardize_image(image_1, image_2);
    let _output = FloatingImage::new(_image1.width(), _image1.height(), args.output);
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

fn standardize_image(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let ( width, height ) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}", width, height);

    if image_1.dimensions() == (width, height) {
        return (image_1, image_2.resize_exact(width, height, Triangle));
    }else {
        return (image_1.resize_exact(width, height, Triangle), image_2);
    }
}
fn _combine_images(image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1:Vec<u8> = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
    // return vec_1;
}

fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_1.len()];
    
    let mut i = 0;
    while i < vec_1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i+3, set_rgba(&vec_1, i, i + 3));
        }else {
            combined_data.splice(i..=i+3, set_rgba(&vec_2, i, i + 3));
        }
        i+=4;
    }
    return combined_data;
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba:Vec<u8> = Vec::new();

    for i in start..=end {
        let val: u8 = match vec.get(i) {
            Some(d) => *d,
            None    => panic!("Index out of range")
        };
        rgba.push(val);
    }

    return rgba;
}
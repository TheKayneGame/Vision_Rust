use crate::vision8b::*;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

#[test]
fn test_auto_1(){
    detect_license_plate("auto1.jpg");
}

#[test]
fn test_auto_2(){
    detect_license_plate("auto2.jpg");
}

#[test]
fn test_auto_3(){
    detect_license_plate("auto3.jpg");
}

#[test]
fn test_auto_4(){
    detect_license_plate("auto4.jpg");
}

pub fn detect_license_plate(path : &str){
	let target_height = 50.0 as f64;
	let image_load = image::open(path).unwrap();

	let mut image = ImgMat::new();

	image.load_image(image_load);

    let mut license_mask = get_license_mask(&image);

    clean_bw_image(&mut license_mask, 10);

    let coordinates = find_license_bounderies(&license_mask);

    image.crop_image(coordinates.0, coordinates.1, coordinates.2, coordinates.3);

    image.grayscale();

    image.invert();

    let mut image_bw = image.treshold(128);

    image_bw.resize(target_height / image_bw.height as f64);

    clean_bw_image(&mut image_bw, 2);

    image_bw.clear_border();

    image_bw.save_image("license.bmp");

    find_license_string(&image_bw);
}

fn find_license_string(license_plate : &ImgBWMat){
    let paths = fs::read_dir("./LetterMasks").unwrap();

    let mut label_vec = ImgLabelMat::new();
    let mut characters = Vec::new();

    label_vec.hoskop_coco(license_plate.clone());

    for boundary in label_vec.boundaries{   
        if boundary.Area() > 200 {
            let mut character = license_plate.clone();
            character.crop_image(boundary.min.0, 0, boundary.max.0, license_plate.image_matrix.len() as u32);
            characters.push(character);
        }
    }
}

fn clean_bw_image(license_mask : &mut ImgBWMat, size: u32){
    let mask : Vec2d<bool> = create_disk(size);

    license_mask.morph_dilate(&mask);
    license_mask.morph_erode(&mask);
    license_mask.morph_erode(&mask);
    license_mask.morph_dilate(&mask);
}

fn get_license_mask(image : &ImgMat) -> ImgBWMat{
    let hsv = image.rgb_to_hsv();
    let mut license_mask : ImgBWMat = ImgBWMat::new();

    license_mask.width = hsv.width;
    license_mask.height = hsv.height;
    license_mask.image_matrix = vec![vec![false; hsv.width as usize]; hsv.height as usize];

    for y in 0..(hsv.height as usize) {
        for x in 0..(hsv.width as usize){
            let hue = hsv.image_matrix[y][x].hue;
            let saturation = hsv.image_matrix[y][x].saturation;
            let value = hsv.image_matrix[y][x].value;

            if hue > 15 && hue < 50 && saturation > 170 && value > 150{
                license_mask.image_matrix[y][x] = true;
            }else{
                license_mask.image_matrix[y][x] = false;
            }
        }
    }

    return license_mask;
}

fn find_license_bounderies(license_mask : &ImgBWMat) -> (u32, u32, u32, u32){
    let y_bounderies = find_y_bounderies(license_mask);
    let x_bounderies = find_x_bounderies(license_mask);

    return (x_bounderies.0, y_bounderies.0, x_bounderies.1, y_bounderies.1);
}

fn find_y_bounderies(image : &ImgBWMat) -> (u32, u32){
    let mut y_low = (image.height + 1) as i32;
    let mut y_high = -1 as i32;

    for x in 0..(image.width as usize) {
        for y in 0..(image.height as usize) {
            if image.image_matrix[y][x] {
                find_y_bounderies_lower_higer(&mut y_low, &mut y_high, y);
            }
        }
    }

    return (y_low as u32, y_high as u32);
}

fn find_y_bounderies_lower_higer(y_low : &mut i32, mut y_high : &mut i32, y : usize){
    if (y as i32) < *y_low {
        *y_low = y as i32;
    }

    if (y as i32) > *y_high {
        *y_high = y as i32;
    }
}

fn find_x_bounderies(image : &ImgBWMat) -> (u32, u32){
    let mut x_low = (image.width + 1) as i32;
    let mut x_high = -1 as i32;

    for y in 0..(image.height as usize) {
        for x in 0..(image.width as usize) {
            if image.image_matrix[y][x] {
                find_x_bounderies_lower_higer(&mut x_low, &mut x_high, x);
            }
        }
    }

    return (x_low as u32, x_high as u32);
}

fn find_x_bounderies_lower_higer(x_low : &mut i32, x_high : &mut i32, x : usize){
    if (x as i32) < *x_low {
        *x_low = x as i32;
    }

    if (x as i32) > *x_high {
        *x_high = x as i32;
    }
}

pub fn create_disk(diameter : u32) -> Vec2d<bool>{
    let mut disk : Vec2d<bool> = vec![vec![false; diameter as usize]; diameter as usize];
    let radius : i32 = (diameter as i32 / 2) as i32;

    for x in 0..(diameter as i32) {
        for y in 0..(diameter as i32) {
            if (x - radius).pow(2) + (y - radius).pow(2) <= radius.pow(2) {
                disk[y as usize][x as usize] = true;
            }
        }
    }

    return disk;
}
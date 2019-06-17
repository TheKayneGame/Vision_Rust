use crate::vision8b::*;

use std::fs;

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

#[test]
fn test_auto5(){
    detect_license_plate("nummerbord.jpg"); 
}

pub fn detect_license_plate(path : &str) -> String{

	let target_height = 50.0 as f64;
	let image_load = image::open(path).unwrap();
	let mut image = ImgMat::new();
	image.load_image(image_load);

    let mut license_mask = get_license_mask(&image);
    clean_bw_image(&mut license_mask, 10);
    license_mask.save_image("license_mask.bmp");

    let coordinates = find_license_bounderies(&license_mask);
    image.crop_image(coordinates.0, coordinates.1, coordinates.2, coordinates.3);

    image.save_image("cropped.bmp");
    image.grayscale();
    image.save_image("grayscale.bmp");
    image.invert();

    let mean = image.pixel_mean();

    let mut image_bw = if mean > 210 {
        image.treshold(180)
    }else if mean < 190  {
        image.treshold(180)
    }else{
        image.treshold(128)
    };

    image_bw.save_image("original_bw_license.bmp");
    image_bw.resize(target_height / image_bw.height as f64);
    clean_bw_image(&mut image_bw, 2);
    image_bw.clear_border();
    image_bw.save_image("license.bmp");

    return find_license_string(&image_bw);
}

fn find_license_string(license_plate : &ImgBWMat) -> String{
    let mut label_vec = ImgLabelMat::new();
    let mut characters_images = Vec::new();
    let mut stripes : Vec<u32> = Vec::new();

    label_vec.hoskop_coco(license_plate.clone());
    label_vec.boundaries.sort_by(|a, b| a.min.0.cmp(&b.min.0));

    let license_width = (label_vec.boundaries.last().unwrap().min.0 - label_vec.boundaries[0].min.0) as u32;

    for boundary in label_vec.boundaries{   
        if boundary.area() > 200 {
            let mut character = license_plate.clone();
            character.crop_image(boundary.min.0, 0, boundary.max.0, license_plate.image_matrix.len() as u32);
            characters_images.push(character);
        }else {
            stripes.push(boundary.min.0);
        }
    }

    let mut character_vector = detect_characters(&mut characters_images);

    insert_stripes(&mut character_vector, &stripes, license_width);

    let mut license_string : String = String::new();

    for character in character_vector{
        license_string.push(character);
    }
    
    return license_string;
}

fn insert_stripes(characters : &mut Vec<char>, stripes : &Vec<u32>, image_width : u32){
    let image_half = stripes[0] + (image_width / 2);

    let left = (image_half - stripes[0]) as f64;
    let right = (stripes[1] - image_half) as f64;

    let left_ratio = left / (image_half as f64);
    let right_ratio = right / (image_half as f64);

    characters.insert(2, '-');

    if (right_ratio - left_ratio) < -0.3 {
        characters.insert(6, '-');
    }else{
        characters.insert(5, '-');
    }
}

fn detect_characters(characters : &mut Vec<ImgBWMat>) -> Vec<char>{
    let mut character_vector : Vec<char> = vec![];
    let masks = load_masks();
    
    for mut character in characters {
        character_vector.push(detect_single_character(&mut character, &masks));
    }

    return character_vector;
}

fn detect_single_character(character : &mut ImgBWMat, masks : &Vec<Vec2d<bool>>) -> char{
    let mut counter : u8 = 0;
    let mut guess : char = '0';
    let mut likeness = 0;

    for mask in masks {
        let mut temp = character.clone();
        temp.morph_erode(mask);
        temp.clear_border();

        if likeness < temp.count_white_pixels() {
            likeness = temp.count_white_pixels();
            
            if counter <= 9 {
                guess = (counter + 48) as char;
            }else{
                guess = (counter + 87) as char;
            }
        }
        counter += 1;
    }
    return guess;
}

fn load_masks() -> Vec<Vec2d<bool>>{
    let paths = fs::read_dir("./LetterMasks").unwrap();
    let mut masks : Vec<Vec2d<bool>> = vec![]; 

    for path in paths {
        let image = image::open(path.unwrap().path()).unwrap();
        let mut image_rgb = ImgMat::new();
        image_rgb.load_image(image);
        image_rgb.grayscale();
        let mask_bw = image_rgb.treshold(128);
        masks.push(mask_bw.image_matrix);
    } 

    return masks;
}

fn clean_bw_image(license_mask : &mut ImgBWMat, size: u32){
    let mask : Vec2d<bool> = create_square(size);

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

fn find_y_bounderies_lower_higer(y_low : &mut i32, y_high : &mut i32, y : usize){
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

pub fn create_square(side : u32) -> Vec2d<bool>{
    return vec![vec![true; side as usize]; side as usize];
}
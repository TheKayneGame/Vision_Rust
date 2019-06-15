use crate::vision8b::*;

pub fn detect_license_plate(path : &str){
	let target_height = 50.0 as f64;
	let image_load = image::open(path).unwrap();

	let mut image = ImgMat::new();

	image.load_image(image_load);

    //create mask from license plate
    let license_mask = get_license_mask(&image);

    let coordinates = find_license_bounderies(&license_mask);

    image.crop_image(coordinates.0, coordinates.1, coordinates.2, coordinates.3);
    image.grayscale();

    let mean = image.pixel_mean();

    if mean < 128 {
        image.adjust_brightness(128 - (mean as i16));
    }

    image.invert();
    let mut image_bw = image.treshold(128);

    image_bw.resize(target_height / image_bw.height);

    //clear border
}

fn get_license_mask(image : &ImgMat) -> ImgBWMat{
    unimplemented!();
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

fn find_x_bounderies_lower_higer(x_low : &mut i32, mut x_high : &mut i32, x : usize){
    if (x as i32) < *x_low {
        *x_low = x as i32;
    }

    if (x as i32) > *x_high {
        *x_high = x as i32;
    }
}
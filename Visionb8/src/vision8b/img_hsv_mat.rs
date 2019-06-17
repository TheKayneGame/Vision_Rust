use image::GenericImage;
use image::Pixel;

pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

pub struct HSVPixel {
	pub hue: u8,
	pub saturation: u8,
	pub value: u8
} 

pub struct ImgHSVMat {
	pub image_matrix: Vec2d<HSVPixel>,
	pub width: u32,
	pub height: u32
}

impl ImgHSVMat {
	pub fn new() -> ImgHSVMat {
		let no_size = 0;

		ImgHSVMat {
			image_matrix: Vec2d::new(),
			width: no_size,
			height: no_size,
		}
	}

	#[allow(dead_code)]
	pub fn save_image(&self, path: &str) {
		let max_alpha = 255;
		let mut img = image::DynamicImage::new_rgb8(self.width, self.height);

		for (index_y, line) in self.image_matrix.iter().enumerate() {
			for (index_x, pixel) in line.iter().enumerate() {
				let rgb_pixel = image::Rgba::from_channels(
					pixel.hue,
					pixel.saturation,
					pixel.value,
					max_alpha,
				);

 				img.put_pixel(index_x as u32, index_y as u32, rgb_pixel);
			}
		}

		let _res = img.save(path);
	}
}

fn color_max (red : f64, green : f64, blue : f64) -> f64{
	let mut max = red;

	if max < green {
		max = green
	}
	
	if max < blue {
		max = blue;
	}

	return max;
}

fn color_min (red : f64, green : f64, blue : f64) -> f64{
	let mut min = red;

	if min > green {
		min = green
	}
	
	if min > blue {
		min = blue;
	}

	return min;
}

fn calculate_hue (red_accent : f64, green_accent : f64, blue_accent : f64, delta : f64, max : f64) -> u8{
	let max_value = 255.0;
	let invalid_delta = 0.0;

	let degree_multiplier = 60.0;
	let full_degree_circle = 360.0;

	let constant1 = 6.0;
	let constant2 = 2.0;
	let constant3 = 4.0;

	let no_hue = 0;

	if delta == invalid_delta {return no_hue};

	if max == red_accent {

		let intermediate = (green_accent - blue_accent) / delta;
		let result = (intermediate % constant1) * degree_multiplier;
		return ((result / full_degree_circle) * max_value) as u8;
		
	}else if max == green_accent {

		let intermediate = (blue_accent - red_accent) / delta;
		let result = (intermediate + constant2) * degree_multiplier;
		return ((result / full_degree_circle) * max_value) as u8;

	}else if max == blue_accent {

		let intermediate = (red_accent - green_accent) / delta;
		let result = (intermediate + constant3) * degree_multiplier;
		return ((result / full_degree_circle) * max_value) as u8;

	}

	return no_hue;
}

fn calculate_saturation(delta : f64, max : f64) -> u8{
	let max_value = 255.0;
	let invalid_max = 0.0;
	let no_saturation = 0;

	if max == invalid_max {
		return no_saturation;
	}else {
		return ((delta / max) * max_value) as u8;
	}
}

pub fn convert_rgb_pixel_to_hsv(rgb : &image::Rgba<u8>) -> HSVPixel{
	let max_value : f64 = 255.0;
	let no_value = 0;

	let red_index = 0;
	let green_index = 1;
	let blue_index = 2;

	let mut hsv_pixel = HSVPixel {
		hue: no_value as u8, 
		saturation: no_value as u8, 
		value: no_value as u8
	};

	let ra : f64 = rgb[red_index] as f64 / max_value;
	let ga : f64 = rgb[green_index] as f64 / max_value;
	let ba : f64 = rgb[blue_index] as f64 / max_value;

	let max = color_max(ra, ga, ba);
	let min = color_min(ra, ga, ba);
	let delta = max - min;

	hsv_pixel.hue = calculate_hue(ra, ga, ba, delta, max);
	hsv_pixel.saturation = calculate_saturation(delta, max);
	hsv_pixel.value = (max * max_value) as u8;

	return hsv_pixel;
}
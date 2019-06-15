use image::Rgba;
use image::GenericImage;
use image::GenericImageView;
use image::Pixel;

pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

pub struct HSVPixel {
	pub hue: u8,
	pub saturation: u8,
	pub value: u8
} 

impl HSVPixel {
	pub fn clone(&self) -> HSVPixel{
		HSVPixel {
			hue: self.hue,
			saturation: self.saturation,
			value: self.value
		}
	}
}

pub struct ImgHSVMat {
	pub image_matrix: Vec2d<HSVPixel>,
	pub width: u32,
	pub height: u32
}

impl ImgHSVMat {
	pub fn new() -> ImgHSVMat {
		ImgHSVMat {
			image_matrix: Vec2d::new(),
			width: 0,
			height: 0,
		}
	}

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

fn calculate_hue (redAccent : f64, greenAccent : f64, blueAccent : f64, delta : f64, max : f64) -> u8{
	let max_value = 255.0;

	if delta == 0.0 {return 0};

	if max == redAccent {
		let intermediate = (greenAccent - blueAccent) / delta;
		let result = (intermediate % 6.0) * 60.0;
		return ((result / 360.0) * max_value) as u8;
	}else if max == greenAccent {
		let intermediate = (blueAccent - redAccent) / delta;
		let result = (intermediate + 2.0) * 60.0;
		return ((result / 360.0) * max_value) as u8;
	}else if max == blueAccent {
		let intermediate = (redAccent - greenAccent) / delta;
		let result = (intermediate + 4.0) * 60.0;
		return ((result / 360.0) * max_value) as u8;
	}

	return 0;
}

fn calculate_saturation(delta : f64, max : f64) -> u8{
	let max_value = 255.0;

	if max == 0.0 {
		return 0;
	}else {
		return ((delta / max) * max_value) as u8;
	}
}

pub fn convert_rgb_pixel_to_hsv(rgb : &image::Rgba<u8>) -> HSVPixel{
	let max_value : f64 = 255.0;

	let mut hsv_pixel = HSVPixel {
		hue: 0 as u8, 
		saturation: 0 as u8, 
		value: 0 as u8
	};

	let ra : f64 = rgb[0] as f64 / max_value;
	let ga : f64 = rgb[1] as f64 / max_value;
	let ba : f64 = rgb[2] as f64 / max_value;

	let max = color_max(ra, ga, ba);
	let min = color_min(ra, ga, ba);
	let delta = max - min;

	hsv_pixel.hue = calculate_hue(ra, ga, ba, delta, max);
	hsv_pixel.saturation = calculate_saturation(delta, max);
	hsv_pixel.value = (max * max_value) as u8;

	return hsv_pixel;
}
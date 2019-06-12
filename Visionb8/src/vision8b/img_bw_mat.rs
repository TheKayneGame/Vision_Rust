
use image::GenericImage;

use crate::image::Pixel;

pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

pub struct ImgBWMat {
	pub image_matrix: Vec2d<bool>,
	pub width: u32,
	pub height: u32,
}

impl ImgBWMat {
	pub fn new() -> ImgBWMat {
		ImgBWMat {
			image_matrix: Vec2d::new(),
			width: 0,
			height: 0,
		}
	}
	
	pub fn clone(&self) -> ImgBWMat{
		ImgBWMat {
			image_matrix: self.image_matrix.clone(),
			width: self.width,
			height: self.height,
		}
	}

	pub fn print_matrix(&self) {
		for line in self.image_matrix.iter() {
			for pixel in line {
				let temp: u8 = if *pixel { 1 } else { 0 };
				print!("{},", temp);
			}
			println!();
		}
	}

	pub fn save_image(&self, path: &str) {
		let mut img = image::DynamicImage::new_rgb8(self.width, self.height);

		for (index_y, line) in self.image_matrix.iter().enumerate() {
			for (index_x, pixel) in line.iter().enumerate() {
				let bin_value = if *pixel { 255 } else { 0 };
				let new_pixel = image::Rgba::from_channels(bin_value, bin_value, bin_value, 255);
				img.put_pixel(index_x as u32, index_y as u32, new_pixel);
			}
		}

		let _res = img.save(path);
	}

	pub fn morph_erode(&mut self, window: Vec2d<bool>, centre_x: u32, centre_y: u32) {
		let window_height = window.len() as u32;
		let window_width = window[0].len() as u32;
		let mut temp_img_mat = Vec2d::new();
		for image_y in 0..(self.height) {
			let mut temp_vector: Vec<bool> = Vec::new();
			for image_x in 0..(self.width) {
				let mut pix_out = true;
				for window_y in 0..(window_height) {
					for window_x in 0..(window_width) {
						let x_check = image_x as i32 - centre_x as i32 + window_x as i32 - 1;
						let y_check = image_y as i32 - centre_y as i32 + window_y as i32 - 1;

						if x_check >= 0
							&& x_check <= self.width as i32
							&& y_check >= 0 && y_check <= self.height as i32
						{
							if window[window_y as usize][window_x as usize]
								&& !self.image_matrix[y_check as usize][x_check as usize]
							{
								pix_out = false;
							}

							if !pix_out {}
						}
					}
				}
				temp_vector.push(pix_out);
			}
			temp_img_mat.push(temp_vector);
		}
		self.image_matrix = temp_img_mat;
	}

	pub fn morph_dilate(&mut self, window: Vec2d<bool>, centre_x: u32, centre_y: u32) {
		let window_height = window.len() as u32;
		let window_width = window[0].len() as u32;
		let mut temp_img_mat = Vec2d::new();
		for image_y in 0..(self.height) {
			let mut temp_vector: Vec<bool> = Vec::new();
			for image_x in 0..(self.width) {
				let mut pix_out = false;
				for window_y in 0..(window_height) {
					for window_x in 0..(window_width) {
						let x_check = image_x as i32 - centre_x as i32 + window_x as i32 - 1;
						let y_check = image_y as i32 - centre_y as i32 + window_y as i32 - 1;

						if x_check >= 0
							&& x_check <= self.width as i32
							&& y_check >= 0 && y_check <= self.height as i32
						{
							if window[window_y as usize][window_x as usize]
								&& self.image_matrix[y_check as usize][x_check as usize]
							{
								pix_out = true;
							}

							if pix_out {}
						}
					}
				}

				temp_vector.push(pix_out);
			}

			temp_img_mat.push(temp_vector);
		}
		self.image_matrix = temp_img_mat;
	}

	pub fn resize(&mut self, ratio : f64){
		let new_height : u32 = ((self.height as f64) * (ratio)) as u32;
		let new_width : u32 = ((self.width as f64) * (ratio)) as u32;

		let mut new_image : Vec2d<bool> = vec![vec![false; new_width as usize]; new_height as usize];

		for y in 0..(self.height - 1) {
			for x in 0..(self.width - 1) {
				let new_y = (y as f64 * ratio) as usize;
				let new_x = (x as f64 * ratio) as usize;

				new_image[new_y][new_x] = self.image_matrix[y as usize][x as usize];
			}
		}

		self.image_matrix = new_image;
		self.height = self.image_matrix.len() as u32;
		self.width = self.image_matrix[0].len() as u32;

		if ratio > 1.0 {
			self.clean_image();
		}
	}

	fn clean_image(&mut self){
		let window: Vec2d<bool> = vec![
			vec![false	, true	, true	, true	, false	],
			vec![true	, true	, true	, true	, true	],
			vec![true	, true	, true	, true	, true	],
			vec![true	, true	, true	, true	, true	],
			vec![false	, true	, true	, true	, false	],
		];

		self.morph_dilate(window.clone(), 3, 3);
		self.morph_erode(window.clone(), 3, 3);
	}
}


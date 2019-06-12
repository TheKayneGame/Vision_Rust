
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
		for image_y in 0..(self.height - 1) {
			let mut temp_vector: Vec<bool> = Vec::new();
			for image_x in 0..(self.width - 1) {
				let mut pix_out = true;
				for window_y in 0..(window_height) {
					for window_x in 0..(window_width) {
						let x_check = image_x as i32 - centre_x as i32 + window_x as i32;
						let y_check = image_y as i32 - centre_y as i32 + window_y as i32;

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
		for image_y in 0..(self.height - 1) {
			let mut temp_vector: Vec<bool> = Vec::new();
			for image_x in 0..(self.width - 1) {
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

	

	
}

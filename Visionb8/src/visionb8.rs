extern crate image;

use image::GenericImage;
use image::GenericImageView;
use image::Pixel;
use image::Rgba;
use std::path::Path;
use std::vec::Vec;

pub type Vec2d<T> = Vec<Vec<T>>;
pub struct ImgVec {
	image_matrix: Vec2d<Rgba<u8>>,
	width: u32,
	height: u32,
}

pub struct ImgBWVec {
	image_matrix: Vec2d<bool>,
	width: u32,
	height: u32,
}

impl ImgBWVec {
	pub fn new() -> ImgBWVec {
		ImgBWVec {
			image_matrix: Vec2d::new(),
			width: 0,
			height: 0,
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
		for image_y in 0..(self.height-1) {
			let mut temp_vector: Vec<bool> = Vec::new();
			for image_x in 0..(self.width-1) {
				
				let mut pix_out = true;
				for window_y in 0..(window_height) {
					for window_x in 0..(window_width) {
						//println!("{} {}", window_x, window_y);
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

							if !pix_out {
//								println!(
//									"x{} y{} wx{} wy{} i{} w{} res{}",
//									x_check,
//									y_check,
//									window_x,
//									window_y,
//									self.image_matrix[y_check as usize][x_check as usize],
//									window[window_y as usize][window_x as usize],
//									pix_out
//								);
							}
						}
					}
				}
				//println!("{} {}", image_x, image_y);
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
		for image_y in 0..(self.height-1) {
			let mut temp_vector: Vec<bool> = Vec::new();
			for image_x in 0..(self.width-1) {
				let mut pix_out = false;
				for window_y in 0..(window_height) {
					for window_x in 0..(window_width) {
						let x_check = image_x as i32 - centre_x as i32 + window_x as i32 -1;
						let y_check = image_y as i32 - centre_y as i32 + window_y as i32 -1;

						if x_check >= 0
							&& x_check <= self.width as i32
							&& y_check >= 0 && y_check <= self.height as i32
						{
							if window[window_y as usize][window_x as usize]
								&& self.image_matrix[y_check as usize][x_check as usize]
							{
								pix_out = true;
							}

							if !pix_out {
//								println!(
//									"x{} y{} wx{} wy{} i{} w{} res{}",
//									x_check,
//									y_check,
//									window_x,
//									window_y,
//									self.image_matrix[y_check as usize][x_check as usize],
//									window[window_y as usize][window_x as usize],
//									pix_out
//								);
							}
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

impl ImgVec {
	pub fn new() -> ImgVec {
		ImgVec {
			image_matrix: Vec2d::new(),
			width: 0,
			height: 0,
		}
	}

	pub fn load_image(&mut self, path: &str) {
		let img = image::open(Path::new(path)).unwrap();

		let mut vec_2d = Vec::new();

		self.height = img.height();
		self.width = img.width();

		for y in 0..img.height() {
			let mut temp_vector = Vec::new();

			for x in 0..img.width() {
				temp_vector.push(img.get_pixel(x, y));
			}

			vec_2d.push(temp_vector);
		}

		self.image_matrix = vec_2d;
	}

	pub fn treshold(&self, treshold_val: u8) -> ImgBWVec {
		let mut bw_vec_out = ImgBWVec::new();
		let mut vec_2d: Vec2d<bool> = Vec::new();

		for line in self.image_matrix.iter() {
			let mut temp_vector: Vec<bool> = Vec::new();
			for pixel in line.iter() {
				let old_pixel = pixel;
				let pixel_values = old_pixel.channels4();
				let pixel_sum: u16 = u16::from(pixel_values.0)
					+ u16::from(pixel_values.1)
					+ u16::from(pixel_values.2);
				let gray_value = (pixel_sum / 3) as u8;

				let bin_value: bool = gray_value > treshold_val;

				temp_vector.push(bin_value);
			}
			vec_2d.push(temp_vector);
		}
		bw_vec_out.image_matrix = vec_2d;
		bw_vec_out.width = self.width;
		bw_vec_out.height = self.height;
		bw_vec_out
	}

	pub fn grayscale(&mut self) {
		let mut vec_2d: Vec2d<Rgba<u8>> = Vec::new();

		for line in self.image_matrix.iter() {
			let mut temp_vector: Vec<Rgba<u8>> = Vec::new();
			for pixel in line.iter() {
				let old_pixel = pixel;
				let pixel_values = old_pixel.channels4();
				let pixel_sum: u16 =
					pixel_values.0 as u16 + pixel_values.1 as u16 + pixel_values.2 as u16;
				let gray_value = (pixel_sum / 3) as u8;

				let new_pixel = image::Rgba::from_channels(gray_value, gray_value, gray_value, 255);
				temp_vector.push(new_pixel);
			}
			vec_2d.push(temp_vector);
		}
		self.image_matrix = vec_2d;
	}

	pub fn invert(&mut self) {
		let mut vec_2d: Vec2d<Rgba<u8>> = Vec::new();
		for line in self.image_matrix.iter() {
			let mut temp_vector: Vec<Rgba<u8>> = Vec::new();
			for pixel in line.iter() {
				let old_pixel = pixel;
				let pixel_values = old_pixel.channels4();

				let new_pixel = image::Rgba::from_channels(
					255 - pixel_values.0,
					255 - pixel_values.1,
					255 - pixel_values.2,
					255,
				);
				temp_vector.push(new_pixel);
			}
			vec_2d.push(temp_vector);
		}
		self.image_matrix = vec_2d;
	}

	pub fn save_image(&self, path: &str) {
		let mut img = image::DynamicImage::new_rgb8(self.width, self.height);

		for (index_y, line) in self.image_matrix.iter().enumerate() {
			for (index_x, pixel) in line.iter().enumerate() {
				img.put_pixel(index_x as u32, index_y as u32, *pixel);
			}
		}

		let _res = img.save(path);
	}

	pub fn print_matrix(&self) {
		for line in self.image_matrix.iter() {
			for pixel in line {
				print!("{:?},", pixel.to_rgb().channels4());
			}
			println!();
		}
	}
}

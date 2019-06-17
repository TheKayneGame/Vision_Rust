use crate::vision8b::img_bw_mat::ImgBWMat;
use crate::vision8b::img_hsv_mat::ImgHSVMat;
use crate::vision8b::img_hsv_mat::HSVPixel;
use crate::vision8b::img_hsv_mat::convert_rgb_pixel_to_hsv;

use image::GenericImage;
use image::GenericImageView;
use image::Pixel;
use image::Rgba;
use std::vec::Vec;

pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

pub struct ImgMat {
	pub image_matrix: Vec2d<Rgba<u8>>, 
	pub width: u32,
	pub height: u32,
}


impl ImgMat {
	pub fn new() -> ImgMat {
		let no_size = 0;

		ImgMat {
			image_matrix: Vec2d::new(),
			width: no_size,
			height: no_size,
		}
	}

	pub fn load_image(&mut self, img : image::DynamicImage) {
		let lowest_bound = 0;

		let mut vec_2d = Vec::new();

		self.height = img.height();
		self.width = img.width();

		for y in lowest_bound..img.height() {
			let mut temp_vector = Vec::new();

			for x in lowest_bound..img.width() {
				temp_vector.push(img.get_pixel(x, y));
			}

			vec_2d.push(temp_vector);
		}

		self.image_matrix = vec_2d;
	}

	pub fn treshold(&self, treshold_val: u8) -> ImgBWMat {
		let average_divisor = 3;

		let mut bw_vec_out = ImgBWMat::new();
		let mut vec_2d: Vec2d<bool> = Vec::new();

		for line in self.image_matrix.iter() {
			let mut temp_vector: Vec<bool> = Vec::new();
			for pixel in line.iter() {
				let old_pixel = pixel;
				let pixel_values = old_pixel.channels4();
				let pixel_sum: u16 = u16::from(pixel_values.0)
					+ u16::from(pixel_values.1)
					+ u16::from(pixel_values.2);
				let gray_value = (pixel_sum / average_divisor) as u8;

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
		let average_divisor = 3;
		let max_alpha = 255;

		let mut vec_2d: Vec2d<Rgba<u8>> = Vec::new();

		for line in self.image_matrix.iter() {
			let mut temp_vector: Vec<Rgba<u8>> = Vec::new();
			for pixel in line.iter() {
				let old_pixel = pixel;
				let pixel_values = old_pixel.channels4();
				let pixel_sum: u16 =
					pixel_values.0 as u16 + pixel_values.1 as u16 + pixel_values.2 as u16;
				let gray_value = (pixel_sum / average_divisor) as u8;

				let new_pixel = image::Rgba::from_channels(gray_value, gray_value, gray_value, max_alpha);
				temp_vector.push(new_pixel);
			}
			vec_2d.push(temp_vector);
		}
		self.image_matrix = vec_2d;
	}

	pub fn invert(&mut self) {
		let max_color = 255;
		let max_alpha = 255;

		let mut vec_2d: Vec2d<Rgba<u8>> = Vec::new();
		for line in self.image_matrix.iter() {
			let mut temp_vector: Vec<Rgba<u8>> = Vec::new();
			for pixel in line.iter() {
				let old_pixel = pixel;
				let pixel_values = old_pixel.channels4();

				let new_pixel = image::Rgba::from_channels(
					max_color - pixel_values.0,
					max_color - pixel_values.1,
					max_color - pixel_values.2,
					max_alpha,
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
	
	#[allow(dead_code)]
	pub fn print_matrix(&self) {
		for line in self.image_matrix.iter() {
			for pixel in line {
				print!("{:?},", pixel.to_rgb().channels4());
			}
			println!();
		}
	}

	pub fn crop_image(&mut self, upper_left_x : u32, upper_left_y : u32, lower_right_x : u32, lower_right_y : u32){
		let mut new_image: Vec2d<Rgba<u8>> = Vec::new();
		for y in upper_left_y..lower_right_y {
			let mut new_x_line: Vec<Rgba<u8>> = Vec::new();
			for x in upper_left_x..lower_right_x {
				new_x_line.push(self.image_matrix[y as usize][x as usize]);
			}
			new_image.push(new_x_line);
		}

		self.image_matrix = new_image;
		self.width = self.image_matrix[0].len() as u32;
		self.height = self.image_matrix.len() as u32;
	}

	pub fn pixel_mean(&self) -> u8{
		let average_divisor = 2;
		let average_color_divisor = 3;

		let first_pixel_x = 0;
		let first_pixel_y = 0;

		let red_index = 0;
		let green_index = 0;
		let blue_index = 0;

		let mut average = 
			((self.image_matrix[first_pixel_y][first_pixel_x][red_index] as u16) +
			(self.image_matrix[first_pixel_y][first_pixel_x][green_index] as u16) +
			(self.image_matrix[first_pixel_y][first_pixel_x][blue_index] as u16)) / average_color_divisor;

		for y in 0..(self.height as usize){
			for x in 1..(self.width as usize){ 
				average += ((self.image_matrix[y][x][red_index] as u16) +
							(self.image_matrix[y][x][green_index] as u16) +
							(self.image_matrix[y][x][blue_index] as u16)) / average_color_divisor;
				average /= average_divisor;
			}
		}

		return average as u8; 
	}

	pub fn rgb_to_hsv(&self) -> ImgHSVMat {
		let lowest_bound = 0;

		let mut hsv_mat : ImgHSVMat = ImgHSVMat::new();

		hsv_mat.height = self.height;
		hsv_mat.width = self.width;

		for y in lowest_bound..(self.height as usize) {
			let mut x_vector : Vec<HSVPixel> = Vec::new();
			for x in lowest_bound..(self.width as usize){
				x_vector.push(convert_rgb_pixel_to_hsv(&self.image_matrix[y][x]));
			}

			hsv_mat.image_matrix.push(x_vector);
		}

		return hsv_mat;
	}
}

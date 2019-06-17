
use image::GenericImage;

use image::Pixel;

use crate::vision8b::license_plate::create_disk;

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

	#[allow(dead_code)]
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

	fn erode_slice(&self, window: &Vec2d<bool>, current_x : usize, current_y : usize) -> bool {
		let window_height = window.len();
		let window_width = window[0].len();

		let centre_x = window_width / 2;
		let centre_y = window_height / 2;

		for y in 0..window_height {
			for x in 0..window_width {
				let check_x = (current_x as i32) - (centre_x as i32) + (x as i32);
				let check_y = (current_y as i32) - (centre_y as i32) + (y as i32);

				if (check_x >= 0) && (check_x < self.width as i32) && (check_y >= 0) && (check_y < self.height as i32) {
					if window[y][x] && !self.image_matrix[check_y as usize][check_x as usize] {
						
						return false;
					}
				}
			}
		}		
		return true;
	}

	pub fn morph_erode(&mut self, window: &Vec2d<bool>){
		let mut new_bw_image : Vec2d<bool> = vec![vec![false; self.width as usize]; self.height as usize];

		for y in 0..(self.height as usize) {
			for x in 0..(self.width as usize) {
				new_bw_image[y][x] = self.erode_slice(&window, x, y);
			}
		}

		self.image_matrix = new_bw_image;
	}

	fn dilate_slice(&self, window: &Vec2d<bool>, current_x : usize, current_y : usize) -> bool {
		let window_height = window.len();
		let window_width = window[0].len();

		let centre_x = window_width / 2;
		let centre_y = window_height / 2;

		for y in 0..window_height {
			for x in 0..window_width {
				let check_x = (current_x as i32) - (centre_x as i32) + (x as i32);
				let check_y = (current_y as i32) - (centre_y as i32) + (y as i32);

				if (check_x >= 0) && (check_x < self.width as i32) && (check_y >= 0) && (check_y < self.height as i32) {
					if window[y][x] && self.image_matrix[check_y as usize][check_x as usize] {
						
						return true;
					}
				}
			}
		}		
		return false;
	}

	pub fn morph_dilate(&mut self, window: &Vec2d<bool>){
		let mut new_bw_image : Vec2d<bool> = vec![vec![false; self.width as usize]; self.height as usize];

		for y in 0..(self.height as usize) {
			for x in 0..(self.width as usize) {
				new_bw_image[y][x] = self.dilate_slice(&window, x, y);
			}
		}

		self.image_matrix = new_bw_image;
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
		let window = create_disk(5);

		self.morph_dilate(&window);
		self.morph_erode(&window);
	}

	pub fn count_white_pixels(&self) -> u32{
		let mut whites : u32 = 0;

		for y in 0..(self.height as usize) {
			for x in 0..(self.width as usize) {
				if self.image_matrix[y][x] {
					whites = whites + 1;
				}
			}
		}

		return whites;
	}

	pub fn crop_image(&mut self, upper_left_x : u32, upper_left_y : u32, lower_right_x : u32, lower_right_y : u32){
		let mut new_image: Vec2d<bool> = Vec::new();
		for y in upper_left_y..lower_right_y {
			let mut new_x_line: Vec<bool> = Vec::new();
			for x in upper_left_x..lower_right_x {
				new_x_line.push(self.image_matrix[y as usize][x as usize]);
			}
			new_image.push(new_x_line);
		}

		self.image_matrix = new_image;
		self.width = self.image_matrix[0].len() as u32;
		self.height = self.image_matrix.len() as u32;
	}

	fn clear_top_border(&mut self){
		for x in 0..(self.image_matrix[0].len()){
			if self.image_matrix[0][x] {
				let mut y = 0;

				loop{
					self.image_matrix[y][x] = false;
					y += 1;

					if y == self.image_matrix.len() {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	fn clear_bottom_border(&mut self){
		for x in 0..(self.image_matrix[0].len()){
			let y = self.image_matrix.len() - 1;
			self.image_matrix[y][x] = true;
		}

		for x in 0..(self.image_matrix[0].len()){
			if self.image_matrix[self.image_matrix.len() - 1][x] {

				let mut y = self.image_matrix.len() -1;
				loop{
					self.image_matrix[y][x] = false;

					y -= 1;

					if y == 0 {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	fn clear_left_border(&mut self){
		for y in 0..(self.image_matrix.len()){
			if self.image_matrix[y][0] {
				let mut x = 0;
				
				loop{
					self.image_matrix[y][x] = false;

					x += 1;

					if x == self.image_matrix[0].len() {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	fn clear_right_border(&mut self){
		for y in 0..(self.image_matrix.len()){
			if self.image_matrix[y][self.image_matrix[0].len() - 1] {
				let mut x = self.image_matrix[0].len() - 1;

				loop{
					self.image_matrix[y][x] = false;

					x -= 1;

					if x == 0 {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	pub fn clear_border(&mut self){
		self.clear_top_border();
		self.clear_bottom_border();
		self.clear_left_border();
		self.clear_right_border();
	}
}


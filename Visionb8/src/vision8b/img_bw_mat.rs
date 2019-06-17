
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
		let no_size = 0;

		ImgBWMat {
			image_matrix: Vec2d::new(),
			width: no_size,
			height: no_size,
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
		let pixel_true = 1;
		let pixel_false = 0;

		for line in self.image_matrix.iter() {
			for pixel in line {
				let temp: u8 = if *pixel { pixel_true } else { pixel_false };
				print!("{},", temp);
			}
			println!();
		}
	}

	pub fn save_image(&self, path: &str) {
		let pixel_highest = 255;
		let pixel_lowest = 0;

		let mut img = image::DynamicImage::new_rgb8(self.width, self.height);

		for (index_y, line) in self.image_matrix.iter().enumerate() {
			for (index_x, pixel) in line.iter().enumerate() {
				let bin_value = if *pixel { pixel_highest } else { pixel_lowest };
				let new_pixel = image::Rgba::from_channels(bin_value, bin_value, bin_value, pixel_highest);
				img.put_pixel(index_x as u32, index_y as u32, new_pixel);
			}
		}

		let _res = img.save(path);
	}

	/// Takes a part from the image vector and erodes it using the window vector
	/// Returns if the current pixel should be true or false
	/// 
	/// # Arguments
	/// * `window` - The 2d vector containing the sliding window
	/// * `current_x` - The x value of the pixel to calculate
	/// * `current_y` - The y value of the pixel to calculate
	fn erode_slice(&self, window: &Vec2d<bool>, current_x : usize, current_y : usize) -> bool {
		let first_x_line = 0;
		let half_divisor = 2;
		let lowest_bound = 0;

		let window_height = window.len();
		let window_width = window[first_x_line].len();

		let centre_x = window_width / half_divisor;
		let centre_y = window_height / half_divisor;

		//check every pixel off the window against the surrounding pixels of the current pixel
		for y in 0..window_height {
			for x in 0..window_width {
				let check_x = (current_x as i32) - (centre_x as i32) + (x as i32);
				let check_y = (current_y as i32) - (centre_y as i32) + (y as i32);

				if (check_x >= lowest_bound) && (check_x < self.width as i32) && (check_y >= lowest_bound) && (check_y < self.height as i32) {
					if window[y][x] && !self.image_matrix[check_y as usize][check_x as usize] {
						return false;
					}
				}
			}
		}		
		return true;
	}

	/// Takes the image vector and erodes it using the window vector
	/// 
	/// # Arguments
	/// * `window` - The 2d vector containing the sliding window
	pub fn morph_erode(&mut self, window: &Vec2d<bool>){
		let mut new_bw_image : Vec2d<bool> = vec![vec![false; self.width as usize]; self.height as usize];

		//erode for every pixel in the image
		for y in 0..(self.height as usize) {
			for x in 0..(self.width as usize) {
				new_bw_image[y][x] = self.erode_slice(&window, x, y);
			}
		}

		self.image_matrix = new_bw_image;
	}

	/// Takes a part from the image vector and dilates it using the window vector
	/// Returns if the current pixel should be true or false
	/// 
	/// # Arguments
	/// * `window` - The 2d vector containing the sliding window
	/// * `current_x` - The x value of the pixel to calculate
	/// * `current_y` - The y value of the pixel to calculate
	fn dilate_slice(&self, window: &Vec2d<bool>, current_x : usize, current_y : usize) -> bool {
		let first_x_line = 0;
		let half_divisor = 2;
		let lowest_bound = 0;

		let window_height = window.len();
		let window_width = window[first_x_line].len();

		let centre_x = window_width / half_divisor;
		let centre_y = window_height / half_divisor;

		//check every pixel off the window against the surrounding pixels of the current pixel
		for y in 0..window_height {
			for x in 0..window_width {
				let check_x = (current_x as i32) - (centre_x as i32) + (x as i32);
				let check_y = (current_y as i32) - (centre_y as i32) + (y as i32);

				if (check_x >= lowest_bound) && (check_x < self.width as i32) && (check_y >= lowest_bound) && (check_y < self.height as i32) {
					if window[y][x] && self.image_matrix[check_y as usize][check_x as usize] {
						return true;
					}
				}
			}
		}		
		return false;
	}

	/// Takes the image vector and dilates it using the window vector
	/// 
	/// # Arguments
	/// * `window` - The 2d vector containing the sliding window
	pub fn morph_dilate(&mut self, window: &Vec2d<bool>){
		let lowest_bound = 0;

		let mut new_bw_image : Vec2d<bool> = vec![vec![false; self.width as usize]; self.height as usize];
		
		//dilate for every pixel in the image
		for y in lowest_bound..(self.height as usize) {
			for x in lowest_bound..(self.width as usize) {
				new_bw_image[y][x] = self.dilate_slice(&window, x, y);
			}
		}

		self.image_matrix = new_bw_image;
	}

	/// Takes the ratio and resizes the image according to the ratio
	/// 
	/// # Arguments
	/// * `ratio` - the ratio to resize the image to
	pub fn resize(&mut self, ratio : f64){
		let lowest_bound = 0;
		let ratio_clean_limit = 1.0;

		let new_height : u32 = ((self.height as f64) * (ratio)) as u32;
		let new_width : u32 = ((self.width as f64) * (ratio)) as u32;

		let mut new_image : Vec2d<bool> = vec![vec![false; new_width as usize]; new_height as usize];

		//map all the old pixels to the new matrix
		for y in lowest_bound..(self.height - 1) {
			for x in lowest_bound..(self.width - 1) {
				let new_y = (y as f64 * ratio) as usize;
				let new_x = (x as f64 * ratio) as usize;

				new_image[new_y][new_x] = self.image_matrix[y as usize][x as usize];
			}
		}

		self.image_matrix = new_image;
		self.height = self.image_matrix.len() as u32;
		self.width = self.image_matrix[0].len() as u32;

		//clean to image if the size is increased
		if ratio > ratio_clean_limit {
			self.clean_image();
		}
	}

	/// Cleanes the image by using a 5 by 5 sliding window with a 
	/// erode and than a dilate
	fn clean_image(&mut self){
		let disk_size = 5;

		let window = create_disk(disk_size);

		self.morph_dilate(&window);
		self.morph_erode(&window);
	}

	/// Returns the amount of white pixels within an image
	pub fn count_white_pixels(&self) -> u32{
		let mut whites : u32 = 0;
		let lowest_bound = 0;

		for y in lowest_bound..(self.height as usize) {
			for x in lowest_bound..(self.width as usize) {
				if self.image_matrix[y][x] {
					whites = whites + 1;
				}
			}
		}

		return whites;
	}

	///	Crops the image according to the given coördinates.
	/// 
	/// # Arguments
	/// * `upper_left_x` - The x of the upper left corner to crop to
	/// * `upper_left_y` - The y of the upper left corner to crop to
	/// * `lower_right_x` - The x of the lower right corner to crop to
	/// * `lower_right_y` - The y of the lower right corner to crop to
	pub fn crop_image(&mut self, upper_left_x : u32, upper_left_y : u32, lower_right_x : u32, lower_right_y : u32){
		let mut new_image: Vec2d<bool> = Vec::new();
		let first_x_line = 0;

		for y in upper_left_y..lower_right_y {

			let mut new_x_line: Vec<bool> = Vec::new();

			for x in upper_left_x..lower_right_x {
				new_x_line.push(self.image_matrix[y as usize][x as usize]);
			}

			new_image.push(new_x_line);
		}

		self.image_matrix = new_image;
		self.width = self.image_matrix[first_x_line].len() as u32;
		self.height = self.image_matrix.len() as u32;
	}

	///	Removes the whites border(s) from the image at the top
	fn clear_top_border(&mut self){
		let first_x_line = 0;
		let lowest_bound = 0;
		let pixel_increment = 1;

		//start on the leftmost pixel and go right until the last x is reached
		for x in lowest_bound..(self.image_matrix[first_x_line].len()){
			if self.image_matrix[first_x_line][x] {

				let mut y = lowest_bound;

				//keep going inward in the image until a black pixel is found
				//and set all white pixels to black on the way.
				loop{
					self.image_matrix[y][x] = false;
					y += pixel_increment;

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

	///	Removes the white border(s) from the image at the bottom
	fn clear_bottom_border(&mut self){
		let first_x_line = 0;
		let lowest_bound = 0;
		let pixel_increment = 1;
		let offset = 1;
		
		//set all bottom pixels to white
		for x in lowest_bound..(self.image_matrix[first_x_line].len()){
			let y = self.image_matrix.len() - offset;
			self.image_matrix[y][x] = true;
		}

		//start on the leftmost pixel and go right until the last x is reached
		for x in lowest_bound..(self.image_matrix[first_x_line].len()){
			if self.image_matrix[self.image_matrix.len() - offset][x] {

				//keep going inward in the image until a black pixel is found
				//and set all white pixels to black on the way.
				let mut y = self.image_matrix.len() - offset;
				loop{
					self.image_matrix[y][x] = false;

					y -= pixel_increment;

					if y == lowest_bound {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	///	Removes the white border(s) from the image on the left
	fn clear_left_border(&mut self){
		let first_x_line = 0;
		let lowest_bound = 0;
		let pixel_increment = 1;

		//starts on the least highest pixel and go until the last y is reached
		for y in lowest_bound..(self.image_matrix.len()){
			if self.image_matrix[y][0] {
				let mut x = lowest_bound;
				
				//keep going inward in the image until a black pixel is found
				//and set all white pixels to black on the way.
				loop{
					self.image_matrix[y][x] = false;

					x += pixel_increment;

					if x == self.image_matrix[first_x_line].len() {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	///	Removes the white border(s) from the image on the right
	fn clear_right_border(&mut self){
		let first_x_line = 0;
		let lowest_bound = 0;
		let pixel_increment = 1;
		let offset = 1;

		//starts on the least highest pixel and go until the last y is reached
		for y in lowest_bound..(self.image_matrix.len()){
			if self.image_matrix[y][self.image_matrix[first_x_line].len() - 1] {

				let mut x = self.image_matrix[first_x_line].len() - offset;

				//keep going inward in the image until a black pixel is found
				//and set all white pixels to black on the way.
				loop{
					self.image_matrix[y][x] = false;

					x -= pixel_increment;

					if x == lowest_bound {
						break;
					}

					if !self.image_matrix[y][x] {
						break;
					}
				}
			}
		}
	}

	///Removes all border(s) from the image
	pub fn clear_border(&mut self){
		self.clear_top_border();
		self.clear_bottom_border();
		self.clear_left_border();
		self.clear_right_border();
	}
}


extern crate image;

use image::GenericImage;
use image::GenericImageView;
use image::Pixel;
use image::Rgba;
use std::path::Path;
use std::vec::Vec;

pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

//Imgage matrix
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

	fn find(index: u32, mut equivalency_list: Vec<u32>) -> u32 {
		let mut y: u32 = index;
		let mut x: u32 = index;
		while equivalency_list[(y - 1) as usize] != y {
			y = equivalency_list[(y - 1) as usize];
		}

		while equivalency_list[(x - 1) as usize] != x {
			let z = equivalency_list[(y - 1) as usize];
			equivalency_list[(x - 1) as usize] = y;
			x = z;
		}

		y
	}

	fn union(mut equivalency_list: Vec<u32>, x: u32, y: u32) -> Vec<u32> {
		let temp_eqlist = equivalency_list.clone();
		equivalency_list[(ImgBWVec::find(x, temp_eqlist) - 1) as usize] =
			ImgBWVec::find(y, equivalency_list.clone());
		equivalency_list
	}

	pub fn hoskop_coco(&self) {
		let mut counter: u32 = 0;
		let mut out_vec: Vec2d<u32> = vec![vec![0; self.width as usize]; self.height as usize];
		let mut equivalency_list: Vec<u32> = Vec::new();
		for image_y in 0..(self.height - 1) {
			for image_x in 0..(self.width - 1) {

				let back_neighbor;
				let top_neighbor;
				if self.image_matrix[image_y as usize][image_x as usize] {
					if image_x > 0 {
						back_neighbor = out_vec[image_y as usize][(image_x - 1) as usize];
					} else {
						back_neighbor = 0;
					}
					if image_y > 0 {
						top_neighbor = out_vec[(image_y - 1) as usize][image_x as usize];
					} else {
						top_neighbor = 0;
					}
					if back_neighbor == 0 && top_neighbor == 0 {
						counter += 1;
						out_vec[image_y as usize][image_x as usize] = counter;
						equivalency_list.push(counter);
					} else if back_neighbor != 0 && top_neighbor == 0 {
						out_vec[image_y as usize][image_x as usize] =
							ImgBWVec::find(back_neighbor, equivalency_list.clone());
					} else if back_neighbor == 0 && top_neighbor != 0 {
						out_vec[image_y as usize][image_x as usize] =
							ImgBWVec::find(top_neighbor, equivalency_list.clone());
					} else {
						let neigbor = if back_neighbor > top_neighbor  {back_neighbor} else {top_neighbor};
						out_vec[image_y as usize][image_x as usize] =
							ImgBWVec::find(neigbor , equivalency_list.clone());
						equivalency_list =
							ImgBWVec::union(equivalency_list.clone(), back_neighbor, top_neighbor);
					}
				}
			}
		}
		
		let mut labels : Vec<u32> = Vec::new();
		let mut obj_bount = 1;
		for y in 0..(self.height - 1) {
			for x in 0..(self.width - 1) {
				if out_vec[y as usize][x as usize] != 0 {
					out_vec[y as usize][x as usize] =
						ImgBWVec::find(out_vec[y as usize][x as usize], equivalency_list.clone());
				}
				
				let  work_label = out_vec[y as usize][x as usize];
				if work_label != 0{
					let temp_label : u32;
					let  index = labels.iter().position(|&s| s == work_label);
					
					if index.is_none() {
						labels.push(work_label);
						temp_label = obj_bount;
						obj_bount += 1;
					}
					else{
						temp_label = (index.unwrap() as u32) + 1 ;	
					}
					out_vec[y as usize][x as usize] = temp_label;
					
				}
				
				if out_vec[y as usize][x as usize] == 0 {
					print!("  |");
				} else {
					print!("{:2}|",out_vec[y as usize][x as usize]);
				}
				

			}
			println!();
		}

		println!("{:?}", labels);
		println!("{:?}", equivalency_list);
		println!("obj_count:{}", labels.len());
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

	pub fn load_image(&mut self, img : image::DynamicImage) {

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

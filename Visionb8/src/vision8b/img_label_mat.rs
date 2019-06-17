use crate::vision8b::img_bw_mat::ImgBWMat;



pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

pub struct ImgLabelMat {
	pub image_matrix: Vec2d<u32>,
	pub width: u32,
	pub height: u32,
	pub obj_count: u32,
	pub boundaries: Vec<Boundary>
}

pub struct Boundary {
	pub label: u32,
	pub min: (u32,u32),
	pub max: (u32,u32)
}

impl ImgLabelMat {
	pub fn new() -> ImgLabelMat {
		ImgLabelMat {
			image_matrix: Vec2d::new(),
			width: 0,
			height: 0,
			obj_count: 0,
			boundaries: Vec::new()
		}
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
		equivalency_list[(ImgLabelMat::find(x, temp_eqlist) - 1) as usize] =
			ImgLabelMat::find(y, equivalency_list.clone());
		equivalency_list
	}

	pub fn hoskop_coco(&mut self, bw_img : ImgBWMat) {
		let mut counter: u32 = 0;
		let mut out_vec: Vec2d<u32> = vec![vec![0; bw_img.width as usize]; bw_img.height as usize];
		let mut equivalency_list: Vec<u32> = Vec::new();
		for image_y in 0..(bw_img.height - 1) {
			for image_x in 0..(bw_img.width - 1) {

				let back_neighbor;
				let top_neighbor;
				if bw_img.image_matrix[image_y as usize][image_x as usize] {
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
							ImgLabelMat::find(back_neighbor, equivalency_list.clone());
					} else if back_neighbor == 0 && top_neighbor != 0 {
						out_vec[image_y as usize][image_x as usize] =
							ImgLabelMat::find(top_neighbor, equivalency_list.clone());
					} else {
						let neigbor = if back_neighbor > top_neighbor  {back_neighbor} else {top_neighbor};
						out_vec[image_y as usize][image_x as usize] =
							ImgLabelMat::find(neigbor , equivalency_list.clone());
						equivalency_list =
							ImgLabelMat::union(equivalency_list.clone(), back_neighbor, top_neighbor);
					}
				}
			}
		}
		let mut labels : Vec<u32> = Vec::new();
		let mut obj_bount = 1;
		for y in 0..(bw_img.height - 1) {
			for x in 0..(bw_img.width - 1) {
				if out_vec[y as usize][x as usize] != 0 {
					out_vec[y as usize][x as usize] =
						ImgLabelMat::find(out_vec[y as usize][x as usize], equivalency_list.clone());
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
			}
		}
		
		self.height = bw_img.height;
		self.width = bw_img.width;
		self.image_matrix = out_vec;
		self.obj_count = obj_bount - 1;
		self.get_boundaries();
	}
	
fn get_boundaries(&mut self){
		let objcount = self.obj_count;
		for target_label in 1..=objcount {
			//print!("l:{:3} ",target_label);
			let mut found_min_x = false;
			let mut found_min_y = false;
			
			let mut found_max_x = false;
			let mut found_max_y = false;
			
			let mut min_x = 0;
			let mut min_y = 0;
			
			let mut max_x = (self.width - 1) as usize;
			let mut max_y = (self.height - 1) as usize;
			
			
			while !found_min_x || !found_min_y || !found_max_x || !found_max_y{
				
				if !found_min_x {
					for j in 0..self.height {
						if self.image_matrix[j as usize][min_x] == target_label  {
							found_min_x = true;
						}
					}
					min_x += 1;
				}
				
				if !found_min_y {
					for j in 0..self.width {
						if self.image_matrix[min_y][j as usize] == target_label {
							found_min_y = true;
						}
					}
					min_y += 1;
				}
				
				if !found_max_x {
					for j in 0..self.height {
						if self.image_matrix[j as usize][max_x] == target_label  {
							found_max_x = true;
						}
					}
					max_x -= 1;
				}
				
				if !found_max_y {
					for j in 0..self.width {
						if self.image_matrix[max_y][j as usize] == target_label {
							found_max_y = true;
						}
					}
					max_y -= 1;
				}
				
				//println!("{} {} {} {}",found_min_x,found_min_y,min_x,min_y);
			}
			
			
			
			
			self.boundaries.push(Boundary
				{
					label: target_label,
					min: ((min_x + 1) as u32,(min_y + 1) as u32 ),
					max: ((max_x +1) as u32,(max_y + 1)as u32)
				}
			);
		}
		
	}
	
	pub fn print_matrix(self){
		for line in self.image_matrix.clone() {
			for pix in line {
				if pix == 0 {
					print!("  |");
				} else {
					print!("{:2}|",pix);
				}
			}
			println!();
			
		}
		println!("Objects: {}", self.obj_count);
		
		
		for boundary in self.boundaries{
			println!("label: {:3},\t min: \t x: {:3} y: {:3},\t max: \t x: {:3} y: {:3}", boundary.label, boundary.min.0,boundary.min.1,boundary.max.0,boundary.max.1);
		}
		
		
		
	}
	
	
}

impl Boundary {
	pub fn Area(&self) -> u32{
		let dx = (self.max.0 as i32) - (self.min.0 as i32);
		let dy = (self.max.1 as i32) - (self.min.1 as i32);

		return (dx.abs() * dy.abs()) as u32;
	}
}
use crate::vision8b::*;

use crate::vision8b::license_plate::create_disk;

pub fn count_eyes(path : &str) -> u32{
	let desired_height : f64 = 300.0;
	let disk_size_3 : u32 = 3;
	let disk_size_5 : u32 = 5; 
	
	let img = image::open(path).unwrap();
	
	let mut img_vec: ImgMat = ImgMat::new();
	
	img_vec.load_image(img);
	
	img_vec.save_image("dice_step_1.bmp");
	
	img_vec.grayscale();
	
	img_vec.invert();
	let mut bw_vec = img_vec.treshold(210);
	
	let ratio :f64 = desired_height / f64::from(bw_vec.height);
	bw_vec.resize(ratio);
	
	let mut label_vec = ImgLabelMat::new();
	img_vec.save_image("dice_step_2.bmp");
	
	let window = create_disk(disk_size_3);
	let window2 = create_disk(disk_size_5);
	
	bw_vec.save_image("dice_step_3.bmp");
	
	bw_vec.morph_erode(&window);
	bw_vec.save_image("dice_step_4.bmp");
	
	bw_vec.morph_dilate(&window2);
	bw_vec.save_image("dice_end_result.bmp");
	
	label_vec.hoskop_coco(bw_vec.clone());

	return label_vec.obj_count;
}


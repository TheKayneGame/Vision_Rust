extern crate image;
mod visionb8;
use image::GenericImageView;



fn main() {
	
	let path = "13ogen.bmp"; 
	println!("Hello, world!");

	let img = image::open(path).unwrap();
	println!("dimensions {:?}", img.dimensions());
	
	let mut img_vec : visionb8::ImgVec = visionb8::ImgVec::new();
	
	img_vec.load_image(path);
	//
	img_vec.grayscale();
	//img_vec.print_matrix();
	img_vec.invert();
	let mut bw_vec = img_vec.treshold(210);
	//bw_vec.print_matrix();
	let window : visionb8::Vec2d<bool> = vec![
		vec![false,true,false],
		vec![true,true,true],
		vec![false,true,false]
	];
	
	let window2 : visionb8::Vec2d<bool> = vec![
		vec![false,true,true,true,false],
		vec![true,true,true,true,true],
		vec![true,true,true,true,true],
		vec![true,true,true,true,true],
		vec![false,true,true,true,false]
	];
	
	
	
	bw_vec.save_image("test1.bmp");
	//bw_vec.print_matrix();
	bw_vec.morph_erode(window.clone(), 1, 1);
	for _i in 0..6 {
		bw_vec.morph_dilate(window2.clone(), 3, 3);
	}
	
	
	img_vec.save_image("test2.bmp");
	bw_vec.save_image("test3.bmp");
}



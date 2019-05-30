extern crate image;
mod visionb8;
use image::GenericImageView;

fn main() {
	let path = "13ogen.bmp";
	println!("Hello, world!");

	let imgpre = image::open(path).unwrap();
	let img = imgpre.resize(1000, 300, image::imageops::Nearest);
	println!("dimensions {:?}", img.dimensions());

	let mut img_vec: visionb8::ImgVec = visionb8::ImgVec::new();

	img_vec.load_image(img);
	//
	img_vec.grayscale();
	//img_vec.print_matrix();
	img_vec.invert();
	let mut bw_vec = img_vec.treshold(210);
	img_vec.save_image("testPre.bmp");
	//bw_vec.print_matrix();
	let window: visionb8::Vec2d<bool> = vec![
		vec![false, true, false],
		vec![true, true, true],
		vec![false, true, false],
	];

	let window2: visionb8::Vec2d<bool> = vec![
		vec![false, true, true, true, false],
		vec![true, true, true, true, true],
		vec![true, true, true, true, true],
		vec![true, true, true, true, true],
		vec![false, true, true, true, false],
	];

	bw_vec.save_image("test1.bmp");
	//bw_vec.print_matrix();
	bw_vec.morph_erode(window.clone(), 1, 1);
	bw_vec.save_image("test2.bmp");
	bw_vec.morph_dilate(window2.clone(), 3, 3);
	bw_vec.save_image("test3.bmp");

	//bw_vec.label_coco();
	bw_vec.hoskop_coco();
}

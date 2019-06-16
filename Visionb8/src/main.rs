extern crate image;
mod vision8b;
use image::GenericImageView;
use vision8b::license_plate::create_disk;

fn main() {
	let path = "13ogen.bmp";
	println!("Hello, world!");

	let imgpre = image::open(path).unwrap();
	let img = imgpre.resize(1000, 300, image::imageops::Nearest);
	println!("dimensions {:?}", img.dimensions());

	let mut img_vec: vision8b::ImgMat = vision8b::ImgMat::new();

	img_vec.load_image(img);

	img_vec.save_image("cropped.bmp");

	img_vec.grayscale();
	
	img_vec.invert();
	let mut bw_vec = img_vec.treshold(210);
	let mut label_vec = vision8b::ImgLabelMat::new();
	img_vec.save_image("testPre.bmp");

	let window = create_disk(3);
	let window2 = create_disk(5);

	bw_vec.save_image("test1.bmp");

	bw_vec.morph_erode(&window);
	bw_vec.save_image("test2.bmp");

	bw_vec.morph_dilate(&window2);
	bw_vec.save_image("test3.bmp");
	
	label_vec.hoskop_coco(bw_vec.clone());

	vision8b::license_plate::detect_license_plate("auto1.jpg");
}
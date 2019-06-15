/*use image::Rgba;

pub type Vec2d<T> = Vec<Vec<T>>; //2D vector

pub struct HSVPixel {
	h: f32,
	s: f32,
	v: f32
} 

pub struct ImgHSVMat {
	pub image_matrix: Vec2d<HSVPixel>,
	pub width: u32,
	pub height: u32
}

impl ImgHSVMat {
	pub fn rgb_to_hsv(input : Rgba<u8>) -> HSVPixel{
		let mut hsv: HSVPixel = HSVPixel {h:0.0,s:0.0,v:0.0};
		let mut min: f32;
		let rgbin = input.channels4();
		let mut max: f32;
		let mut delta: f32 = max - min;
		hsv.v = max;
		
		hsv
	}
}*/
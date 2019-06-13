use image::Rgba;
use image::Pixel;

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
	pub fn rgb_to_hsv(input : Rgba<u8>) -> HSVPixel {
		let mut hsv: HSVPixel = HSVPixel {h:0.0,s:0.0,v:0.0};

		let rgbin = input.channels4();
		let rgbinf : (f32,f32,f32) = ((rgbin.1 /255).into(), (rgbin.2 / 255).into(),(rgbin.3/255).into());
	
		let mut min: f32 = if rgbinf.0 < rgbinf.1  {rgbinf.0 } else {rgbinf.1};
		min = if min < rgbinf.2  {min} else {rgbinf.2};
		
		let mut max: f32 = if rgbinf.0 > rgbinf.1  {rgbinf.0 } else {rgbinf.1};
		max = if min > rgbinf.2  {min} else {rgbinf.2};
		
		let delta: f32 = max - min;
		hsv.v = max;
		
		if delta < 0.00001
		{
        	hsv.s = 0.0;
        	hsv.h = 0.0; // undefined, maybe nan?
        	return hsv;
   		}

		if max > 0.0 { // NOTE: if Max is == 0, this divide would cause a crash
			hsv.s = delta / max;                  // s
		} else {
		// if max is 0, then r = g = b = 0              
		// s = 0, h is undefined
		hsv.s = 0.0;
		hsv.h = 0.0;
		return hsv;
		}
		
		if rgbinf.0 >= max {
			hsv.h = ( rgbinf.1 - rgbinf.2 ) as f32 / delta;	
		}
		else if rgbinf.1 >= max {
			hsv.h = 2.0 + ( rgbinf.2 - rgbinf.0 ) as f32;  // between cyan & yellow			
		} else {
			hsv.h = 4.0 + ( rgbinf.0 - rgbinf.1 ) as f32 / delta;  // between magenta & cyan
		}
				

		
		hsv.h *= 60.0;                              // degrees
		
		if hsv.h < 0.0 { 
			hsv.h += 360.0;			
		}


		
		hsv
	}
}
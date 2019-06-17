
use std::env;
mod vision8b;

fn main() {
	let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
	println!("Hello, world!");
	vision8b::dice_count::count_eyes("13ogen.jpg");
	

	vision8b::license_plate::detect_license_plate("auto1.jpg");
}
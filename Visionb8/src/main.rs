use std::env;
mod vision8b;
use std::path::Path ;

fn main() {
	let args: Vec<String> = env::args().collect();

	let photo_path = Path::new(&args[2]);

	if args[1] == "--dice" && args.len() == 3 && photo_path.exists() {

		println!("{}", vision8b::dice_count::count_eyes(&args[2][..]));

	}else if args[1] == "--license" && args.len() == 3 && photo_path.exists() {

		println!("{}", vision8b::license_plate::detect_license_plate(&args[2][..]));
		
	}else{
		println!("Wrong arguments, try using \"--license <image location>\" or \"--dice <image location>\"");
	}	
}
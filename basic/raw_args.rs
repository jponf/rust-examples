
use std::env;

fn main() {
	println!("**** using args() ****");
	println!("# Arguments: {}", env::args().len());
	println!("----------------------");
	// the returned iterator will panic if any argument
	// is not valid unicode
	for argument in env::args() {
		println!("{}", argument);
	}

	println!("\n**** using args_os() ****");
	println!("# Arguments: {}", env::args_os().len());
	println!("-------------------------");
	// args_os has no problem with non unicode arguments
	for argument in env::args_os() {
		println!("{:?}", argument);
	}

}
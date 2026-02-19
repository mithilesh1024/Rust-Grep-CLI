use std::env;
use std::fs;
//use std::process;

fn main() -> Result<(), std::io::Error> {
	let mut args = env::args();

	args.next();

	let pattern = match args.next() {
		Some(p) => p,
		None => {
			eprintln!("Usage: rgrep <pattern> <file>");
			std::process::exit(1);
		}
	};

	let file_name = match args.next() {
		Some(p) => p,
		None => {
			eprintln!("Usage: rgrep <pattern> <file>");
			std::process::exit(1);
		}
	};

	let contents = fs::read_to_string(&file_name)?;

	for (line_number, line) in contents.lines().enumerate() {
		if line.contains(&pattern) {
			println!("{} {} {}",file_name,line_number + 1, line);
		}
	}

	Ok(())
}

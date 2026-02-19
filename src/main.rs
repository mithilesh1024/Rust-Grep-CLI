use std::env;
use std::fs;
//use std::process;

fn main() -> Result<(), std::io::Error> {
	let mut args = env::args().skip(1);

	let pattern = match args.next() {
		Some(p) => p,
		None => {
			eprintln!("Usage: rgrep <pattern> <file>");
			std::process::exit(1);
		}
	};

	for file_name in args {
		let contents = fs::read_to_string(&file_name)?;

		for (line_number, line) in contents.lines().enumerate() {
			if line.contains(&pattern) {
				let highlighted = line.replace(
                    &pattern,
                    &format!("\x1b[1;31m{}\x1b[0m", &pattern) // 31 = red, 0 = reset
                );

                let fname = format!("\x1b[35m{}\x1b[0m", file_name);
                let collon = format!("\x1b[36m{}\x1b[0m", ":");
		
				println!("{}{}{}{}{}",fname,collon,line_number + 1,collon, highlighted);
			}
		}
	}

	Ok(())
}

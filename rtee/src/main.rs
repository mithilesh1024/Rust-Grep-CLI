use std::fs;
use std::env;
use std::io::{self, Read, Write};

fn main() -> Result<(), std::io::Error>{
    let file_name = match env::args().nth(1) {
        Some(name) => name,
        None => {
            eprintln!("Please enter a valid file name");
            return Ok(());
        }
    };

    let mut file = fs::File::create(file_name)?;
    loop {
        let mut buffer = vec![0u8; 4096];
        let size = io::stdin().read(&mut buffer)?; 

        if size == 0 {
            break; // EOF
        }
        
        io::stdout().write_all(&buffer[..size])?;
        file.write_all(&buffer[..size])?;
    }

    Ok(())
}

use std::fs;
use std::io;

pub fn list_files() -> io::Result<()>{

    let entries = fs::read_dir(".")?;
    for entry in entries {
        match entry {
            Ok(e) => {
                let path = e.path();
                match path.file_name() {
                    Some(name_osstr) => {
                        let name = name_osstr.to_string_lossy();

                        match name.chars().next() {
                            Some('.') => continue,
                            _ => print!("{} ",name),
                        }
                    }
                    None => continue,
                }
                
            },
            Err(_) => continue,
        };
    }
    println!("");
    Ok(())
}
use std::io::{self, Write};
use std::env;

mod listFiles;

fn main() -> Result<(), std::io::Error>{
    let shell_name = String::from("\x1b[1;31mrshell>\x1b[0m");
    loop {
        let path = env::current_dir()?;
        print!("{}\x1b[1;34m{}>\x1b[0m$",shell_name,path.display());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        let _ = match input {
            "ls" => listFiles::list_files(),
            "exit" => break,
            _ => continue,
        };
    }

    Ok(())
}

use std::{fs::OpenOptions, io::Read};

fn main() -> std::io::Result<()> {
    let mut buf = String::new();
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.md")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(10);
        }
    };

    file.read_to_string(&mut buf)?;

    println!("{buf}");

    Ok(())
}

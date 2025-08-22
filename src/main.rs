use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rcat <filename>");
        process::exit(1);
    }

    let mut had_error = false;

    for arg in &args[1..] {
        if let Err(e) = check_file_exists(arg) {
            eprintln!("Error: {}", e);
            had_error = true;
            continue;
        }
        let filename = arg.trim();
        if let Err(e) = run(filename) {
            eprintln!("Error: {}", e);
            had_error = true;
            continue;
        }
    }

    if had_error {
        process::exit(1);
    }
}

fn check_file_exists(filename: &str) -> io::Result<()> {
    if !fs::metadata(filename).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File '{}' not found", filename),
        ));
    }
    Ok(())
}

fn run(filename: &str) -> io::Result<()> {
    let content = fs::read_to_string(filename)?;
    println!("{}", content);
    Ok(())
}

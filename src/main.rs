use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::process;

fn main() {
    println!("Hello from windsurf");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rcat <filename>");
        process::exit(1);
    }

    let mut had_error = false;

    for arg in &args[1..] {
        if arg == "-" {
            if let Err(e) = read_in_stdin() {
                eprintln!("Error reading from stdin: {}", e);
                had_error = true;
            }

            continue;
        }
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
    let mut file = fs::File::open(filename)?;
    let mut buffer = [0u8; 4096];
    let mut stdout = io::stdout();
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        stdout.write_all(&buffer[..n])?;
    }
    stdout.flush()?;
    Ok(())
}

fn read_in_stdin() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        writeln!(stdout, "{}", line)?;
        stdout.flush()?;
    }
    Ok(())
}

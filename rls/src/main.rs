use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let mut show_all = false;
    let mut path = String::from(".");

    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    _ => {
                        eprintln!("rls: unknown option -{}", c);
                        std::process::exit(1);
                    }
                }
            }
        } else {
            path = arg;
        }
    }

    let mut names: Vec<String> = Vec::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if !show_all && name.starts_with('.') {
            continue;
        }

        names.push(name);
    }

    names.sort();

    for name in &names {
        println!("{}", name);
    }

    Ok(())
}
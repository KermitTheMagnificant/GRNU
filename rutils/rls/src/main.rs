use std::env;
use std::fs;

fn main() {
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

    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(e) => {
            // io::Error's Display appends " (os error N)"; GNU ls doesn't, so strip it.
            let msg = e.to_string();
            let msg = msg.split(" (os error").next().unwrap_or(&msg);
            eprintln!("rls: cannot access '{}': {}", path, msg);
            std::process::exit(2);
        }
    };

    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| show_all || !name.starts_with('.'))
        .collect();

    names.sort();

    for name in &names {
        println!("{}", name);
    }
}

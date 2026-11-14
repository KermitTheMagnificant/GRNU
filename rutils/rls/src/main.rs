use std::env;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

fn main() {
    let mut long = false;
    let mut show_all = false;
    let mut path = String::from(".");

    for arg in env::args().skip(1) {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long = true,
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

    // read_dir never yields "." and "..", but GNU ls -a lists them.
    if show_all {
        names.push(String::from("."));
        names.push(String::from(".."));
    }

    names.sort();

    if long {
        print_long(&path, &names);
    } else {
        for name in &names {
            println!("{}", name);
        }
    }

    fn print_long(dir: &str, names: &[String]) {
        let mut rows = Vec::new();
        let mut total_blocks = 0;

        for name in names {
            let full = Path::new(dir).join(name);
            let meta = match fs::symlink_metadata(&full) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("rls: cannot access '{}': {}", full.display(), e);
                    continue;
                }
            };
            total_blocks += meta.blocks();
            rows.push((
                mode_string(&meta),
                meta.nlink().to_string(),
                meta.uid().to_string(),
                meta.gid().to_string(),
                meta.size().to_string(),
                name.clone(),
            ));
        }
        let links_w = rows.iter().map(|r| r.1.len()).max().unwrap_or(0);
        let uid_w   = rows.iter().map(|r| r.2.len()).max().unwrap_or(0);
        let gid_w   = rows.iter().map(|r| r.3.len()).max().unwrap_or(0);
        let size_w  = rows.iter().map(|r| r.4.len()).max().unwrap_or(0);

        println!("total {}", total_blocks / 2);
        for (mode, links, uid, gid, size, name) in &rows {
            println!(
                "{} {:>links_w$} {:<uid_w$} {:<gid_w$} {:>size_w$} {}",
                mode, links, uid, gid, size, name
            );
        }

        fn mode_string(meta: &fs::Metadata) -> String {
            let ft = meta.file_type();
            let kind = if ft.is_dir() {
                'd'
            } else if ft.is_symlink() {
                'l'
            } else {
                '-'
            };

            let mode = meta.permissions().mode();
            let mut s = String::with_capacity(10);
            s.push(kind);
            for shift in [6, 3, 0] {
                let bits = (mode >> shift) & 0o7;
                s.push(if bits & 0o4 != 0 { 'r' } else { '-' });
                s.push(if bits & 0o2 != 0 { 'w' } else { '-' });
                s.push(if bits & 0o1 != 0 { 'x' } else { '-' });
            }
            s
        }
    }
}

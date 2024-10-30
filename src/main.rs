#[cfg(not(target_os = "linux"))]
compile_error!("This program only supports Linux.");

use std::env;
use std::fs;
use std::path::PathBuf;
use chrono::Local;

fn main() {
    // Get command line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: rr <file or dir>");
        std::process::exit(1);
    }

    // Get the current date in the format YYYY-MM-DD
    let date = Local::now().format("%Y-%m-%d").to_string();

    // Construct the target directory path /var/tmp/rr_removed/YYYY-MM-DD/
    let base_dir = PathBuf::from("/var/tmp/rr_removed").join(date);

    // Create the target directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&base_dir) {
        eprintln!("Error creating directory {}: {}", base_dir.display(), e);
        std::process::exit(1);
    }

    // Process each input file or directory
    for arg in args {
        let src = PathBuf::from(&arg);
        if !src.exists() {
            eprintln!("File or directory does not exist: {}", src.display());
            continue;
        }

        // Get the file or directory name
        let file_name = match src.file_name() {
            Some(name) => name,
            None => {
                eprintln!("Invalid file or directory: {}", src.display());
                continue;
            }
        };

        // Construct the target path
        let dest = base_dir.join(file_name);

        // Move the file or directory
        if let Err(e) = fs::rename(&src, &dest) {
            eprintln!("Error moving {} to {}: {}", src.display(), dest.display(), e);
            continue;
        }
    }
}

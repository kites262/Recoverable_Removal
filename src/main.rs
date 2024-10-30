use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

fn main() {
    // Get command line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: rr <file or dir> or rr --restore");
        std::process::exit(1);
    }

    if args[0] == "--restore" {
        // Handle restore functionality
        println!("Starting restore process...");

        // Read last.tag file
        let last_tag_path = PathBuf::from("/var/tmp/rr_removed/last.tag");
        let last_tag_contents = fs::read_to_string(&last_tag_path)
            .expect("Unable to read last.tag file");

        let mut lines: Vec<&str> = last_tag_contents.lines().collect();

        if lines.is_empty() {
            eprintln!("No entries in last.tag to restore.");
            std::process::exit(1);
        }

        // Get the last timestamp
        let timestamp = lines.pop().unwrap().to_string();
        println!("Using timestamp {}", timestamp);

        // Write back the rest of the lines to last.tag
        fs::write(&last_tag_path, lines.join("\n")).expect("Unable to write to last.tag file");

        // Construct base_dir as /var/tmp/rr_removed/{timestamp}
        let base_dir = PathBuf::from("/var/tmp/rr_removed").join(&timestamp);

        // Read rr_removed.restore_path.txt to get restore path
        let restore_path_file_path = base_dir.join("rr_removed.restore_path.txt");
        let restore_path_contents = fs::read_to_string(&restore_path_file_path)
            .expect("Unable to read rr_removed.restore_path.txt file");

        let restore_path = PathBuf::from(restore_path_contents.trim());

        let restore_dir = base_dir.join("restore");

        // Collect entries and check for conflicts
        let mut entries = Vec::new();
        let mut conflicts_found = false;

        for entry_result in fs::read_dir(&restore_dir).expect("Unable to read restore directory") {
            let entry = entry_result.expect("Unable to get directory entry");
            let src_path = entry.path();
            let file_name = src_path.file_name().unwrap().to_owned();
            let dest_path = restore_path.join(&file_name);

            // Check if destination path exists
            if dest_path.exists() {
                conflicts_found = true;
            }

            entries.push((src_path, dest_path));
        }

        if conflicts_found {
            // Conflict detected
            // Create restore_conflicted_{timestamp} under restore_path
            let conflicted_dir_name = format!("restore_conflicted_{}", timestamp);
            let conflicted_dir = restore_path.join(&conflicted_dir_name);

            // Create the conflicted directory
            if let Err(e) = fs::create_dir_all(&conflicted_dir) {
                eprintln!("Error creating conflicted directory {}: {}", conflicted_dir.display(), e);
                std::process::exit(1);
            }

            // Move all files from restore_dir to conflicted_dir
            for (src_path, _) in entries {
                let file_name = src_path.file_name().unwrap();
                let dest_path = conflicted_dir.join(file_name);

                if let Err(e) = fs::rename(&src_path, &dest_path) {
                    eprintln!("Error moving {} to {}: {}", src_path.display(), dest_path.display(), e);
                    continue;
                }
            }

            println!("Conflicts detected. \nFiles have been moved to: \n{}", conflicted_dir.display());

        } else {
            // No conflicts, proceed to restore
            for (src_path, dest_path) in entries {
                if let Err(e) = fs::rename(&src_path, &dest_path) {
                    eprintln!("Error moving {} to {}: {}", src_path.display(), dest_path.display(), e);
                    continue;
                }
            }

            println!("Files have been successfully restored to: \n{}", restore_path.display());
        }

        // Optionally, remove the base_dir
        // fs::remove_dir_all(&base_dir).expect("Unable to remove base directory");

    } else {
        // Handle delete (move to trash) functionality

        // Get the current date in the format YYYY-MM-DD
        let date = Local::now().format("%Y-%m-%d-%H-%M-%S-%f").to_string();

        // Construct the base directory path /var/tmp/rr_removed/{timestamp}
        let base_dir = PathBuf::from("/var/tmp/rr_removed").join(&date);

        // Create the restore directory
        let restore_dir = base_dir.join("restore");

        // Create the restore directory
        if let Err(e) = fs::create_dir_all(&restore_dir) {
            eprintln!("Error creating directory {}: {}", restore_dir.display(), e);
            std::process::exit(1);
        }

        // Append the timestamp to last.tag
        let last_tag_path = PathBuf::from("/var/tmp/rr_removed/last.tag");
        let mut last_tag_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&last_tag_path)
            .expect("Unable to open last.tag file");
        writeln!(last_tag_file, "{}", date).expect("Unable to write to last.tag file");

        // Create rr_removed.restore_path.txt in base_dir, recording the current directory
        let restore_path_file_path = base_dir.join("rr_removed.restore_path.txt");
        let mut restore_path_file = fs::File::create(&restore_path_file_path)
            .expect("Unable to create rr_removed.restore_path.txt file");

        // Record the current working directory
        let current_dir = env::current_dir().expect("Unable to get current directory");
        writeln!(restore_path_file, "{}", current_dir.display()).expect("Unable to write to rr_removed.restore_path.txt");

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

            // Construct the target path in the restore directory
            let dest = restore_dir.join(file_name);

            // Move the file or directory
            if let Err(e) = fs::rename(&src, &dest) {
                eprintln!("Error moving {} to {}: {}", src.display(), dest.display(), e);
                continue;
            } else {
                // println!("Delete '{}', files restored to '{}'", src.display(), dest.display());
            }
        }
    }
}

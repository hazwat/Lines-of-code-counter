use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Get the current working directory
    let current_dir = std::env::current_dir()?;
    
    // Process the directory recursively and print total lines
    match process_directory_total_lines(&current_dir) {
        Ok(total_lines) => println!("Total lines in directory: {}", total_lines),
        Err(e) => eprintln!("Error processing directory: {:?}", e),
    }

    Ok(())
}

fn process_directory_total_lines<P: AsRef<Path>>(dir_path: P) -> io::Result<usize> {
    let mut total_lines = 0;

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // If the entry is a directory, process it recursively
            total_lines += process_directory_total_lines(&path)?;
        } else if path.is_file() {
            // If the entry is a file, process it
            if let Ok(line_count) = process_file_total_lines(&path) {
                total_lines += line_count;
            } else {
                //eprintln!("Failed to read file: {:?}", &path);
            }
        }
    }

    Ok(total_lines)
}

fn process_file_total_lines<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
    // Read the contents of the file into a string
    let contents = fs::read_to_string(&file_path)?;

    // Count non-blank lines
    let line_count = contents.lines().filter(|line| !line.trim().is_empty()).count();

    Ok(line_count)
}

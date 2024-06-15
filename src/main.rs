use std::{
    io::{self, BufRead},
    fs::File,
};
use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "source code counter", version,
about = "Check source code vitals summary", long_about = None)]
struct Cli {
    #[arg(short = 'f', long = "folder", required = true, help = "Folder path to search for files")]
    folder_path: String,
    #[arg(short, long, required = true, help = "File extension to search for (ie. 'rs' for Rust files)")]
    extension: String,
    #[arg(short, long = "ignore", value_delimiter = ',', help = "Folders to exclude from search")]
    ignore_folders: Option<Vec<String>>,
    #[arg(long = "hc", default_value = "false", help = "Use hardcoded exclude folders")]
    use_hardcoded_exclude: bool,
}

fn main() {
    let exclude_folders_embedded = vec![
        ".idea", ".git",
        "node_modules",
        "obj", "bin",
        "build", "out", "dist",
    ];

    let cli = Cli::parse();

    let folder_path = &cli.folder_path;
    let file_extension = &cli.extension;
    let passed_exclude_folders = match &cli.ignore_folders {
        Some(folders) => folders.iter().map(String::as_str).collect(),
        None => Vec::new(),
    };

    let exclude_folders = if cli.use_hardcoded_exclude && passed_exclude_folders.is_empty() {
        exclude_folders_embedded
    } else {
        passed_exclude_folders
    };

    let mut total_files = 0;
    let mut total_lines_in_all_files = 0;
    let mut file_paths = Vec::new();

    println!("Searching for files in folder {}", folder_path);
    println!();

    for entry in WalkDir::new(folder_path)
        .into_iter().filter_entry(|e| should_include_entry(e, &exclude_folders)) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext.to_string_lossy() == file_extension.to_string()) {
            file_paths.push(path.to_path_buf());
        }
    }

    file_paths.sort();

    for path in file_paths {
        total_files += 1;

        let lines = count_lines_in_file(&path);
        match lines {
            Ok(count) => {
                total_lines_in_all_files += count;

                let canonical_path = path.display().to_string().replace(folder_path, "");
                println!("File {}, lines: {}", remove_first_char(&canonical_path), count);
            },
            Err(ex) => eprintln!("Error reading a file {:?}: {}", path, ex),
        };
    }

    println!();
    println!("Total files found: {}", total_files);
    println!("Total lines in all files: {}", total_lines_in_all_files);
}

fn count_lines_in_file<F>(filename: F) -> io::Result<usize>
    where F: AsRef<std::path::Path> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let non_empty_lines = reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .count();

    Ok(non_empty_lines)
}

fn remove_first_char(s: &str) -> &str {
    &s[1..]
}

fn should_include_entry(entry: &DirEntry, exclude_folders: &[&str]) -> bool {
    !exclude_folders.iter().any(|&exclude| entry.path().to_string_lossy().contains(exclude))
}
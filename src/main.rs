mod models;
mod hardcoded;

use std::{
    io::{self, BufRead},
    fs::File,
};
use std::io::Write;
use clap::Parser;
use colored::Colorize;
use walkdir::{DirEntry, WalkDir};
use crate::hardcoded::EXCLUDE_FOLDERS_EMBEDDED;
use crate::models::{Cli, FileResult};

const DEFAULT_MAX_DEPTH: usize = 10;

fn main() {
    let cli = Cli::parse();

    let folder_path = &cli.folder_path;
    let passed_exclude_folders = match &cli.ignore_folders {
        Some(folders) => folders.iter().map(String::as_str).collect(),
        None => Vec::new(),
    };

    let exclude_folders = if cli.use_hardcoded_exclude && passed_exclude_folders.is_empty() {
        EXCLUDE_FOLDERS_EMBEDDED.iter().map(|&s| s).collect()
    } else {
        passed_exclude_folders
    };

    let mut file_paths = Vec::new();

    println!("Searching for files in folder {}", folder_path.bright_blue());
    println!();

    for entry in WalkDir::new(folder_path)
        .max_depth(cli.max_depth.unwrap_or(DEFAULT_MAX_DEPTH))
        .into_iter().filter_entry(|e| should_include_entry(e, &exclude_folders)) {
        let entry = entry.unwrap();
        let path = entry.path();

        // if path.is_file() && path.extension().map_or(false, |ext| ext.to_string_lossy() == file_extension.to_string()) {
        //     file_paths.push(path.to_path_buf());
        // }

        if path.is_file() {
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            if cli.extensions.iter().any(|ext| ext == extension) {
                file_paths.push(path.to_path_buf());
            }
        }
    }

    file_paths.sort();

    let mut files_data : Vec<FileResult> = Vec::new();

    for path in file_paths {
        let lines = count_lines_in_file(&path);
        match lines {
            Ok(count) => {
                let canonical_path = path.display().to_string().replace(folder_path, "");

                files_data.push(FileResult {
                    path: canonical_path.clone(),
                    lines_count: count,
                });

                println!("File {}, lines: {}",
                         remove_first_char(&canonical_path).to_string().bright_blue(),
                         count.to_string().bright_green());
            },
            Err(ex) => eprintln!("Error reading a file {:?}: {}", path, ex),
        };
    }

    println!();
    println!("Total files found: {}", files_data.len().to_string().bright_green());
    println!("Total lines in all files: {}", files_data.iter()
        .fold(0, |sum, val| sum + val.lines_count)
        .to_string().bright_green());

    if cli.statistics {
        let (average, median, std_deviation) = extended_statistics(&files_data);
        println!();
        println!("Average lines per file: {}", average.to_string().bright_green());
        println!("Median lines per file: {}", median.to_string().bright_green());
        println!("Standard deviation: {}", std_deviation.to_string().bright_green());
    }

    if cli.generate_report {
        generate_markdown_report(&files_data, folder_path.clone());
    }
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

fn generate_markdown_report(data: &Vec<FileResult>, folder_path: String) {
    let mut report = String::new();

    report.push_str("# Report\n\n");
    report.push_str(&format!("Folder: *{}*\n\n", folder_path));
    report.push_str("| File | # Lines |\n");
    report.push_str("| --- | --- |\n");

    for file in data {
        report.push_str(&format!("| {} | _{}_ |\n", file.path, file.lines_count));
    }

    report.push_str("\n");
    report.push_str(&format!("Total files: *{}*\n\n", data.len()));
    report.push_str(&format!("Total lines: *{}*\n", data.iter()
        .fold(0, |sum, val| sum + val.lines_count)));
    report.push_str("\n---\n");
    report.push_str("_Generated by [source-code-scanner](https://github.com/RustamIrzaev/source-code-scanner)_");

    let report_path = format!("{}/report_scs.md", folder_path.clone());
    let mut file = File::create(&report_path).expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    println!("\nReport generated in {}", report_path.bright_blue());
}

fn extended_statistics(data: &Vec<FileResult>) -> (usize, usize, f64) {
    let lines_count: Vec<_> = data.iter().map(|file| file.lines_count).collect();

    if lines_count.is_empty() {
        return (0, 0, 0.0);
    }

    let num_files = lines_count.len();
    let total_lines: usize = lines_count.iter().sum();
    let average = total_lines / num_files;

    let mut sorted = lines_count.to_vec();
    sorted.sort();

    let median = if num_files % 2 == 0 {
        let mid = num_files / 2;
        (sorted[mid - 1] + sorted[mid]) / 2
    } else {
        sorted[num_files / 2]
    };

    let variance = lines_count.iter()
        .map(|&count| (count as f64 - average as f64).powf(2.0))
        .sum::<f64>() / num_files as f64;
    let std_deviation = variance.sqrt();

    (average, median, std_deviation)
}
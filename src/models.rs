use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "source code counter", version,
about = "Check source code vitals summary", long_about = None)]
pub struct Cli {
    #[arg(short = 'f', long = "folder", required = true, help = "Folder path to search for files")]
    pub folder_path: String,

    #[arg(short, long, required = true, help = "File extension to search for (ie. 'rs' for Rust files)")]
    pub extension: String,

    #[arg(short, long = "ignore", value_delimiter = ',', help = "Folders to exclude from search")]
    pub ignore_folders: Option<Vec<String>>,

    #[arg(long = "hc", default_value = "false", help = "Use hardcoded exclude folders")]
    pub use_hardcoded_exclude: bool,
}
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "source code counter", version,
about = "Check source code vitals summary", long_about = None)]
pub struct Cli {
    #[arg(short = 'f', long = "folder", required = true, help = "Folder path to search for files")]
    pub folder_path: String,

    #[arg(short, long = "extensions", value_delimiter = ',', required = true, help = "File extension to search for (ie. 'rs' for Rust files)")]
    pub extensions: Vec<String>,

    #[arg(short, long = "ignore", value_delimiter = ',', help = "Folders to exclude from search")]
    pub ignore_folders: Option<Vec<String>>,

    #[arg(long = "hce", default_value = "false", help = "Use hardcoded exclude folders")]
    pub use_hardcoded_exclude: bool,

    #[arg(short = 'r', long = "report", help = "Generate a markdown report")]
    pub generate_report: bool,

    #[arg(long = "max-depth", help = "Maximum depth for recursive scanning")]
    pub max_depth: Option<usize>,

    #[arg(short, long = "stats", help = "Show extended summary statistics (average, median, std deviation)")]
    pub statistics: bool,
}

pub struct FileResult {
    pub path: String,
    pub lines_count: usize,
}
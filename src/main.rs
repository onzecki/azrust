use walkdir::{DirEntry, WalkDir};
use regex::Regex;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Pattern to search for
    pattern: String,

    /// Path to start the search from
    path: String,

    /// Results return path, filename, size, date modified, and file type
    #[clap(short, long)]
    detail: bool,

    /// Output results in JSON format
    #[clap(short, long)]
    json: bool,

    /// Search hidden files and directories
    #[clap(long)]
    hidden: bool
}



fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn matches_requirements(entry: &DirEntry, pattern: &Regex, hidden_flag: bool) -> bool {
    // Check if the entry is hidden and whether there is a hidden flag
    if is_hidden(entry) && hidden_flag {
        return false;
    }
    entry.file_name()
         .to_str()
         .map(|s| pattern.is_match(s))
         .unwrap_or(false)
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(&args.pattern).unwrap();
    for entry_result in WalkDir::new(&args.path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || args.hidden) // Include hidden entries if args.hidden is true
    {
        let entry = match entry_result{
            Ok(entry) => entry,
            Err(_) => {
                println!("Error reading.");
                continue;
            },
        }; // Unwrap the Result

        if matches_requirements(&entry, &re, args.hidden){
            println!("{}", entry.path().display());
        }
    }
}

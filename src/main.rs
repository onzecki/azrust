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

fn matches_requirements(entry: &DirEntry, pattern: &Regex) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| pattern.is_match(s))
         .unwrap_or(false)
}

fn main() {
    let args = Args::parse();
    // Parse the cli arguments
    let re = Regex::new(&args.pattern).unwrap();
    // Set "re" as the Regex pattern
    for entry_result in WalkDir::new(&args.path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || args.hidden) // Include hidden entries if args.hidden is true
    {

        let entry = match entry_result{

            Ok(entry) => entry,
            Err(_) => {
                println!("Error reading.");
                continue;
                // For each file in a directory/directories check whether an entry is valid and unwrap the result
            },
        };

        if matches_requirements(&entry, &re){
            println!("{}", entry.path().display());
            // Check whether entry matches the Regex
        }
    }
}

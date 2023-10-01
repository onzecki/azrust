use regex::RegexBuilder;
use clap::{CommandFactory, Parser};
use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
struct Args {
    /// Pattern to search for
    pattern: Option<String>,

    /// Path to start the search from
    path: Option<String>,

    /// Results return path, filename, size, date modified, and file type
    #[clap(short, long)]
    detail: bool,

    /// Output results in JSON format
    #[clap(short, long)]
    json: bool,

    /// Search hidden files and directories
    #[clap(long)]
    hidden: bool,
}

// Check if the &DirEntry is hidden
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// Print details about a given &DirEntry
fn print_detailed(entry: &DirEntry) {
    if let Ok(metadata) = fs::metadata(entry.path()) {
        println!("Name: {}", entry.file_name().to_str().expect("Failed to get file name"));
        println!("Size: {} bytes", metadata.len());
        println!("Modified: {:?}", format_system_time(metadata.modified().expect("Failed to get modified time")));
        println!("Accessed: {:?}", format_system_time(metadata.accessed().expect("Failed to get accessed time")));
        println!("Created: {:?}", format_system_time(metadata.created().expect("Failed to get created time")));
    }
}

// Return System time as a RFC2822 string
fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc2822()
}

fn main() {
    let args = Args::parse();

    let pattern = match args.pattern {
        Some(ref arg) => {
            // Check if the provided pattern is valid RegEx
            match RegexBuilder::new(&arg).build() {
                Ok(re) => re,
                Err(_) => {
                    println!("Invalid RegEx!\n");
                    let mut cmd = Args::command();
                    cmd.print_help().expect("Failed to print help");
                    return;
                }
            }
        }
        None => RegexBuilder::new(".*").build().unwrap(),
    };

    // If path is not provided, set it to the working directory
    let path = match args.path {
        Some(arg) => arg,
        None =>
            std::env::current_dir()
                .expect("Failed to get working directory").to_string_lossy().to_string(),
    };

    let mut json_paths = Vec::new();

    for entry_result in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || args.hidden) // Include hidden entries if args.hidden is true
    {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Check if the pattern has been provided
        if args.pattern.is_some() {
            // Check whether entry matches the Regex
            if pattern.is_match(entry.file_name().to_str().unwrap()) {
                println!("{}", entry.path().display()); // Print full path

                if args.json {
                    json_paths.push(entry.path().to_string_lossy().to_string());
                } else {
                    if args.detail {
                        print_detailed(&entry);
                    }
                }
            }
        } else {
            if args.json {
                json_paths.push(entry.path().to_string_lossy().to_string());
            } else {
                if args.detail {
                    print_detailed(&entry);
                } else {
                    // Print full path
                    println!("{}", entry.path().display());
                }
            }
        }

        if args.json {
            // Print results in JSON format
            println!("{}", serde_json::to_string(&json_paths).expect("Failed to serialize to JSON"))
        }
    }
}

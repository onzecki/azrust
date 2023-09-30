use walkdir::{DirEntry, WalkDir};
use regex::{Regex, RegexBuilder};
use clap::Parser;
use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::Serialize;

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

#[derive(Serialize)]
struct PathInfo {
    path: String,
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

fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc2822()
}


fn main() {
    let args = Args::parse();

    // Parse the cli arguments
    let pattern = match args.pattern {
        Some(arg) => {
            match RegexBuilder::new(&arg).build() {
                Ok(re) => re,
                Err(_) => {
                    println!("Invalid RegEx!\n");
                    use clap::CommandFactory;
                    let mut cmd = Args::command();
                    match cmd.print_help() {
                        Ok(help) => help,
                        Err(e) => println!("Clap error: {}", e)
                    };
                    return;
                }
            }
        }
        None => RegexBuilder::new(".*").build().unwrap(),
    };

    let path = match args.path {
        Some(arg) => arg,
        // Gotta love safety
        None => std::env::current_dir().expect("Failed to get working directory").to_string_lossy().into_owned(),
    };

    // Set "re" as the Regex pattern
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
                // For each file in a directory/directories check whether an entry is valid and unwrap the result
            }
        };

        if matches_requirements(&entry, &pattern) { // Check whether entry matches the Regex
            if !args.json { // Might hinder the performance a tiny bit, but I don't care
                println!("{}", entry.path().display()); // Show the entries full path
                if args.detail { // If we want to show details, then show the details ^^
                    if let Ok(metadata) = fs::metadata(entry.path()) {
                        println!("Name: {}", entry.file_name().to_str().unwrap());
                        println!("Size: {} bytes", metadata.len());
                        println!("Modified: {:?}", format_system_time(metadata.modified().unwrap()));
                        println!("Accessed: {:?}", format_system_time(metadata.accessed().unwrap()));
                        println!("Created: {:?}", format_system_time(metadata.created().unwrap()));
                    }
                }
            } else {
                json_paths.push(entry.path().to_string_lossy().to_string());
                // Push the path of every single result to the json thingy
            }
        }
    }
    if args.json {
        println!("{}", serde_json::to_string(&json_paths).unwrap())
        // Output the json string
    }
}

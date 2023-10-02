use chrono::{DateTime, Utc};
use clap::{crate_description, crate_name, crate_version, CommandFactory, Parser};
use humansize::{format_size, DECIMAL};
use regex::RegexBuilder;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[clap(
name = crate_name!(),
version = crate_version!(),
about = crate_description!()
)]
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
struct JsonDetail {
    name: String,
    path: String,
    size: isize,
    modified: u64,
    accessed: u64,
    created: u64,
}

// Check if the &DirEntry is hidden
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// Print details about a given &DirEntry
fn print_detailed(entry: &DirEntry) {
    if let Ok(metadata) = fs::metadata(entry.path()) {
        println!("\n{}", entry.path().display());
        println!(
            "\tName: {}",
            entry.file_name().to_str().expect("Failed to get file name")
        );
        println!("\tSize: {}", format_size(metadata.len(), DECIMAL));
        println!(
            "\tModified: {:?}",
            format_system_time(metadata.modified().expect("Failed to get modified time"))
        );
        println!(
            "\tAccessed: {:?}",
            format_system_time(metadata.accessed().expect("Failed to get accessed time"))
        );
        println!(
            "\tCreated: {:?}",
            format_system_time(metadata.created().expect("Failed to get created time"))
        );
    }
}

// Return System time as a RFC2822 string
fn format_system_time(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc2822()
}

fn print_help() {
    let mut cmd = Args::command();
    cmd.print_help().expect("Failed to print help");
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
                    print_help();
                    return;
                }
            }
        }
        None => RegexBuilder::new(".*").build().unwrap(),
    };

    // If path is not provided, set it to the working directory
    let path = match args.path {
        Some(arg) => {
            if !Path::exists(arg.as_ref()) {
                println!("Invalid directory path!");
                print_help();
                return;
            }

            fs::canonicalize(arg).unwrap().to_string_lossy().to_string()
        }
        None => std::env::current_dir()
            .expect("Failed to get working directory")
            .to_string_lossy()
            .to_string(),
    };

    let mut json_paths = Vec::new();
    let mut json_paths_detailed = Vec::new();

    for entry_result in WalkDir::new(path.clone())
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || args.hidden)
    // Include hidden entries if args.hidden is true
    {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                if !args.json {
                    // Wouldn't want error messages to pop up with a json output.
                    println!("{}", e);
                }
                continue;
            }
        };

        // Check if the pattern has been provided
        if args.pattern.is_some() {
            // Check whether entry matches the Regex
            if pattern.is_match(entry.file_name().to_str().unwrap()) {
                if args.json {
                    if args.detail {
                        if let Ok(metadata) = fs::metadata(entry.path()) {
                            let json_detailed = JsonDetail {
                                name: entry
                                    .file_name()
                                    .to_str()
                                    .expect("Failed to get file name")
                                    .to_string(),
                                path: path.clone(),
                                size: metadata.len() as isize,
                                modified: metadata
                                    .modified()
                                    .expect("Failed to get modified time")
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Failed to calculate duration")
                                    .as_secs(),
                                accessed: metadata
                                    .accessed()
                                    .expect("Failed to get accessed time")
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Failed to calculate duration")
                                    .as_secs(),
                                created: metadata
                                    .created()
                                    .expect("Failed to get created time")
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Failed to calculate duration")
                                    .as_secs(),
                            };

                            json_paths_detailed.push(json_detailed);
                        }
                    } else {
                        json_paths.push(entry.path().to_string_lossy().to_string());
                    }
                } else {
                    if args.detail {
                        print_detailed(&entry);
                    } else {
                        println!("{}", entry.path().display()); // Print full path
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
    }

    // Print results in JSON format
    if args.json {
        if args.detail {
            println!("{}", serde_json::to_string(&json_paths_detailed).unwrap());
        } else {
            println!("{}", serde_json::to_string(&json_paths).unwrap());
        }
    }
}

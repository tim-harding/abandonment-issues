use clap::{CommandFactory, Parser};
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    time::SystemTime, fs,
};
use walkdir::WalkDir;

const SECS_PER_MIN: u64 = 60;
const MINS_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const SECS_PER_DAY: u64 = SECS_PER_MIN * MINS_PER_HOUR * HOURS_PER_DAY;

fn main() {
    let args = Args::parse();
    if args.directory.is_empty() {
        let _ = Args::command().print_help();
        return;
    }

    let now = SystemTime::now();
    for directory in args.directory {
        let mut walk_dir = WalkDir::new(directory);
        if let Some(depth) = args.depth {
            walk_dir = walk_dir.max_depth(depth);
        }

        for entry in walk_dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let dir_to_delete = match entry.file_name().to_str() {
                Some(name) => match name {
                    "Cargo.lock" => "target",
                    "package-lock.json" => "node_modules",
                    _ => continue,
                },
                None => continue,
            };

            let parent = match entry.path().parent() {
                Some(parent) => parent,
                None => continue,
            };


            if let Some(recent) = args.recent {
                match entry.metadata() {
                    Ok(metadata) => match metadata.accessed() {
                        Ok(accessed) => match now.duration_since(accessed) {
                            Ok(duration) => {
                                if duration.as_secs() / SECS_PER_DAY < recent {
                                    let parent = parent.display();
                                    println!("Ignoring {parent}");
                                    continue;
                                }
                            }
                            Err(_) => continue,
                        },
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                }
            }

            let _ = fs::remove_dir_all(parent.join(dir_to_delete));
        }
    }
}

#[derive(clap::Parser, Debug, Clone, PartialEq, Eq, Hash)]
#[command(author = "Tim Harding <tim@timharding.co>")]
#[command(about = "Finds and removes build and package files")]
struct Args {
    /// List directories in which to search for projects
    #[arg(short, long, value_name = "FILE")]
    pub directory: Vec<PathBuf>,

    /// How deep into a file tree to search for projects
    #[arg(long)]
    pub depth: Option<usize>,

    /// If the number of days since the last access of the lock file is fewer
    /// than this, don't clean out the project
    #[arg(long, value_name = "DAYS")]
    pub recent: Option<u64>,
}

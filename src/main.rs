use clap::{CommandFactory, Parser};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::SystemTime,
};
use walkdir::WalkDir;

const SECS_PER_MIN: u64 = 60;
const MINS_PER_HOUR: u64 = 60;
const HOURS_PER_DAY: u64 = 24;
const SECS_PER_DAY: u64 = SECS_PER_MIN * MINS_PER_HOUR * HOURS_PER_DAY;

fn main() {
    let args = Args::parse();
    if args.directory.is_empty() {
        Args::command().print_help().unwrap();
        return;
    }

    let now = SystemTime::now();
    for directory in args.directory {
        let mut project_kind = ProjectKind::None;
        let mut walk_dir = WalkDir::new(directory).contents_first(true);
        if let Some(depth) = args.depth {
            walk_dir = walk_dir.max_depth(depth);
        }
        for entry in walk_dir {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() {
                match project_kind {
                    ProjectKind::None => {}
                    ProjectKind::Cargo => run("cargo", &["clean"], entry.path()),
                    ProjectKind::Npm => run("rm", &["-r", "node_modules"], entry.path()),
                }
                project_kind = ProjectKind::None;
            } else {
                let tentative_project_kind = match entry.file_name().to_str() {
                    Some(name) => match name {
                        "Cargo.lock" => ProjectKind::Cargo,
                        "package-lock.json" => ProjectKind::Npm,
                        _ => continue,
                    },
                    None => continue,
                };

                if let Some(recent) = args.recent {
                    match entry.metadata() {
                        Ok(metadata) => match metadata.accessed() {
                            Ok(accessed) => match now.duration_since(accessed) {
                                Ok(duration) => {
                                    if duration.as_secs() / SECS_PER_DAY < recent {
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

                project_kind = tentative_project_kind;
            }
        }
    }
}

fn run(program: &str, args: &[&str], dir: &Path) {
    let child = Command::new(program)
        .current_dir(dir)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
    match child {
        Ok(mut child) => {
            print!("Cleaning {dir:?}");
            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        println!(" ✓")
                    } else {
                        eprintln!("\nExited unsuccessfully");
                    }
                }
                Err(e) => eprintln!("\nFailed to clean {dir:?}: {e}"),
            }
        }
        Err(e) => eprintln!("Failed to run {program}: {e}"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ProjectKind {
    None,
    Cargo,
    Npm,
}

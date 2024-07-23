use std::{collections::BTreeMap, env, fs, path::PathBuf, process::Command};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Checks for dirty Git repositories.
struct Cli {
    /// Optional root dir, otherwise uses current working dir.
    root: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cwd = env::current_dir().expect("Error: Failed to get current working directory");
    let root = match cli.root {
        Some(root) => PathBuf::from(root),
        None => cwd.clone(),
    };

    let entries = fs::read_dir(&root).context(format!("Couldn't read directory: {:?}", &root))?;
    for e in entries.filter_map(Result::ok).filter(|e| e.path().is_dir()) {
        let path = e.path();
        if path.join(".git").exists() {
            let output = Command::new("git")
                .arg("status")
                .arg("--porcelain=v1")
                .current_dir(e.path())
                .output()
                .context(format!("Failed git status on: {:?}", e.path()))?;
            if output.status.success() {
                let output = String::from_utf8_lossy(&output.stdout);
                let statuses = parse(&output);
                if !statuses.is_empty() {
                    let path = path.strip_prefix(&cwd).unwrap_or(path.as_path());
                    println!("{} {:?}", path.display(), statuses);
                }
                
            } else {
                let output = String::from_utf8_lossy(&output.stderr);
                eprintln!("{}", output);
            }
        };
    }
    Ok(())
}

fn parse(output: &str) -> BTreeMap<String, u32> {
    let mut statuses = BTreeMap::<String, u32>::new();
    for line in output.lines() {
        match line.split_once(' ') {
            Some((status, _path)) => *statuses.entry(status.to_string()).or_insert(0) += 1,
            None => eprintln!("Unparsable output {}", line),
        }
    }
    statuses
}

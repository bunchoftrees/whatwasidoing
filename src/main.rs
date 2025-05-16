use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use regex::Regex;

fn main() {
    println!("Last commit:\n");

    match get_last_commit() {
        Some(msg) => println!("  {}\n", msg),
        None => println!("  No commits found or not a Git repo.\n"),
    }

    println!("TODOs:\n");
    let current_dir = Path::new(".");
    scan_for_todos(current_dir);
}

fn get_last_commit() -> Option<String> {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=%B")
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

fn scan_for_todos(dir: &Path) {
    let mut comment_styles = HashMap::new();
    comment_styles.insert("rs", "//");
    comment_styles.insert("py", "#");
    comment_styles.insert("sh", "#");
    comment_styles.insert("js", "//");
    comment_styles.insert("ts", "//");
    comment_styles.insert("jsx", "//");
    comment_styles.insert("tsx", "//");
    comment_styles.insert("java", "//");
    comment_styles.insert("toml", "#");
    comment_styles.insert("yml", "#");
    comment_styles.insert("yaml", "#");
    comment_styles.insert("html", "<!--");

    let re = Regex::new(r"\b(TODO|FIXME)\b").unwrap();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name != "target" && name != ".git" {
                    scan_for_todos(&path);
                }
            } else if let Ok(file) = File::open(&path) {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if let Some(comment_prefix) = comment_styles.get(ext.as_str()) {
                    let reader = BufReader::new(file);
                    for (i, line) in reader.lines().enumerate() {
                        if let Ok(content) = line {
                            let trimmed = content.trim_start();
                            if trimmed.starts_with(comment_prefix) && re.is_match(trimmed) {
                                println!("- [{}:{}] {}", path.display(), i + 1, trimmed);
                            }
                        }
                    }
                }
            }
        }
    }
}

// TODO: make this thing work with just my TODOs
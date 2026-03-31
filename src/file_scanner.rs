use std::fs::{self};
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn scan_directories(paths: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    if paths.is_dir() {
        for entry in fs::read_dir(paths)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if matches!(name, "node_modules" | ".next" | "dist" | "build" | ".git") {
                        continue;
                    }
                }
                let mut sub_paths = scan_directories(&path)?;
                file_paths.append(&mut sub_paths);
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "ts" | "tsx" | "jsx" | "js") {
                    file_paths.push(path);
                }
            }
        }
    }
    Ok(file_paths)
}

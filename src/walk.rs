use std::io::Result;
use std::path::PathBuf;

use jwalk::WalkDir;

/// Recursively traverse through directories and look for README and FUNDING.yml files.
pub fn walk_dir() -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut readme_files: Vec<PathBuf> = Vec::new();
    let mut funding_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(".").skip_hidden(false) {
        let path = entry?.path();
        match path.file_name() {
            Some(filename) => {
                if filename.to_str().is_some() {
                    let fmt_filename = filename.to_str()
                        .unwrap()
                        .to_lowercase();
                    
                    match fmt_filename.as_str() {
                        "readme.md" => readme_files.push(path),
                        "funding.yml" => funding_files.push(path),
                        _ => ()
                    }
                }
            },
            None => ()
        }
    }

    Ok((readme_files, funding_files))
}
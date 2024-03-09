use std::env;
use std::fs;
use walkdir::WalkDir;
use std::path::Path;
use std::io::Result;

fn remove_dot_underscore_files(root_path: &Path) -> Result<()> {
    for entry in WalkDir::new(root_path) {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.starts_with("._"))
            .unwrap_or(false)
        {
            fs::remove_file(entry_path)?;
            println!("Removed: {:?}", entry_path);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        std::process::exit(1);
    }

    let root_path = Path::new(&args[1]);
    remove_dot_underscore_files(root_path)?;
    Ok(())
}

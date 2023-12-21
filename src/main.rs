use std::io::prelude::*;
use std::path::Path;
use std::{env, fs};

fn read_bytes(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = fs::File::open(filepath)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn save_bytes(bytes: &[u8], target_path: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(target_path)?;
    file.write_all(bytes)?;
    Ok(())
}

fn double_file_extension(source_path: &str) -> String {
    let path = Path::new(source_path);
    let extension = path.extension().unwrap_or_default();
    let stem = path.file_stem().unwrap_or_default();
    format!(
        "{}.{}{}",
        stem.to_str().unwrap(),
        extension.to_str().unwrap(),
        extension.to_str().unwrap()
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let source_path = &args[1];

    if args.len() != 2 {
        println!("Usage: program_name file_name");
        return;
    }

    let target_path = double_file_extension(source_path);

    if let Err(error) =
        read_bytes(source_path).and_then(|content| save_bytes(&content, &target_path))
    {
        println!("Error: {}", error);
    } else {
        println!("File successfully saved to: {}", target_path);
    }
}

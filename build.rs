use brotli::CompressorWriter;
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Directory containing JSON files
    let json_dir = Path::new("pinmame-nvram-maps");
    println!("cargo:rerun-if-changed={}", json_dir.display());

    // Output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("maps.brotli");
    fs::create_dir_all(&out_path)?;

    eprintln!(
        "Compressing JSON files in {} to {}",
        json_dir.display(),
        out_path.display()
    );

    let quality = 11;
    let lg_window_size = 22;

    // Process files recursively, keeping the directory structure
    process_directory(json_dir, &out_path, quality, lg_window_size)?;

    Ok(())
}

fn process_directory(
    in_path: &Path,
    out_path: &Path,
    quality: u32,
    lg_window_size: u32,
) -> io::Result<()> {
    for entry in fs::read_dir(in_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let out_path_deeper = out_path.join(path.file_name().unwrap());
            process_directory(&path, &out_path_deeper, quality, lg_window_size)?;
        } else if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .ends_with(".nv.json")
            || path.file_name() == Some("index.json".as_ref())
        {
            // Make sure the parent directory exists.
            // We do this here because we only want directories with JSON files.
            fs::create_dir_all(out_path)?;
            // Read the JSON file
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Minify the JSON content
            let json: Value = serde_json::from_str(&contents)?;
            let minified = serde_json::to_string(&json)?;

            let mut compressed_path = out_path.join(entry.file_name());
            compressed_path.set_extension(format!(
                "{}.brotli",
                compressed_path.extension().unwrap().to_string_lossy()
            ));

            // Compress the minified JSON using Brotli
            let mut compressed_file = CompressorWriter::new(
                File::create(&compressed_path)?,
                4096,
                quality,
                lg_window_size,
            );
            compressed_file.write_all(minified.as_bytes())?;

            eprintln!(
                "Compressed {} to {}",
                path.display(),
                compressed_path.display()
            );
        }
    }
    Ok(())
}

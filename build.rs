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

    // Iterate over all JSON files in the directory
    for entry in fs::read_dir(json_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            // Read the JSON file
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // Minify the JSON content
            let json: Value = serde_json::from_str(&contents)?;
            let minified = serde_json::to_string(&json)?;

            // Compress the minified JSON using Brotli
            // path out file with .json.brotli
            let compressed_path = out_path
                .join(path.file_stem().unwrap())
                .with_extension("json.brotli");
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

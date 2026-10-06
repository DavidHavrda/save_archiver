use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short, long, default_value = "backup")]
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let save_path = args.path;

    if !save_path.exists() {
        println!("No folder found at: {:?}", save_path);
        return Ok(());
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let archive_name = format!("{}_{}.zip", args.name, timestamp);

    let zip_file = File::create(&archive_name).context("Failed to create zip file")?;
    let mut zip_writer = ZipWriter::new(zip_file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    println!("Backing up {:?} to {}...\n", save_path, archive_name);

    for entry in WalkDir::new(&save_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let relative_path = path.strip_prefix(&save_path)?;
            let relative_path_str = relative_path.to_string_lossy().into_owned();

            zip_writer.start_file(&relative_path_str, options)?;

            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip_writer.write_all(&buffer)?;

            println!("Added: {}", relative_path_str);
        }
    }

    zip_writer.finish()?;
    println!("\nBackup complete! Saved as {}", archive_name);

    Ok(())
}

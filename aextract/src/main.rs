use std::{env::set_current_dir, error::Error, fs, io::{Read, Write}, path::PathBuf};

mod cli;
use chrono::Utc;
use clap::Parser;
pub use cli::Cli;

pub mod types;
pub mod error;
pub mod generation;
pub use error::*;
use satisfactory_data::registry::RegistryMeta;
use tempfile::TempDir;
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

fn setup_logger(level: log::LevelFilter) -> std::result::Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}/{}:: {}",
                record.target(),
                record.level(),
                message
            ))
        }).level(level).chain(std::io::stdout()).apply()?;
    Ok(())
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let options = Cli::parse();
    setup_logger(options.verbosity.clone().into())?;

    let (working_directory, _tmpdir): (PathBuf, Option<TempDir>) = if let Some(wd) = options.workdir.clone() {
        if wd.exists() {
            (wd, None)
        } else {
            std::fs::create_dir_all(wd.clone())?;
            (wd, None)
        }
    } else {
        let tmd = TempDir::new()?;
        (tmd.path().to_path_buf(), Some(tmd))
    };

    let generated = generation::raw::generate_raw_data(options.clone(), working_directory.clone())?;
    let mut cleaned = generation::cleaned::generate_clean_data(generated);
    cleaned.metadata = RegistryMeta {
        generated: Utc::now(),
        game_version: options.game_version.clone(),
        experimental: options.experimental_version,
    };

    fs::create_dir_all(working_directory.join("staging").join("icons"))?;
    fs::create_dir_all(working_directory.join("staging").join("map"))?;
    fs::write(working_directory.join("staging").join("registry.json"), serde_json::to_string_pretty(&cleaned)?)?;

    for fp in glob::glob(working_directory.join("assets").join("*.png").to_str().unwrap())? {
        if let Ok(pt) = fp {
            if let Some(fname) = pt.file_name() {
                if fname.to_str().unwrap().starts_with("MapSlice") {
                    fs::rename(pt.clone(), working_directory.join("staging").join("map").join(fname))?;
                } else {
                    fs::rename(pt.clone(), working_directory.join("staging").join("icons").join(fname))?;
                }
            }
        }
    }

    let mut output_file = fs::File::create(options.output())?;
    let mut zipfile = ZipWriter::new(&mut output_file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut buffer: Vec<u8> = Vec::new();
    set_current_dir(working_directory.join("staging"))?;
    for file_path in WalkDir::new(".") {
        if let Ok(path) = file_path.and_then(|p| Ok(p.path().to_path_buf())) {
            if path.is_file() {
                zipfile.start_file_from_path(&path, options.clone())?;

                let mut f = fs::File::open(path)?;
                f.read_to_end(&mut buffer)?;
                zipfile.write_all(&*buffer)?;
                buffer.clear();
            } else if path.as_os_str().len() != 0 {
                zipfile.add_directory_from_path(&path, options.clone())?;
            }
        }
    }

    Ok(())
}


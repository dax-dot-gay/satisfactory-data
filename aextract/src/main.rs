use std::{error::Error, path::PathBuf};

mod cli;
use clap::Parser;
pub use cli::Cli;

pub mod types;
pub mod error;
pub mod generation;
pub use error::*;
use tempfile::TempDir;

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

    let generated = generation::raw::generate_raw_data(options.clone(), working_directory)?;
    let cleaned = generation::cleaned::generate_clean_data(generated);
    std::fs::write("./test.json", serde_json::to_string_pretty(&cleaned)?)?;
    Ok(())
}


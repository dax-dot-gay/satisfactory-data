use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::InfoLevel;

/// Automated extractor for Satisfactory assets and documentation data
#[derive(Parser, Clone, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,

    /// Set path to steam library (ie ~/.steam/steam)
    #[arg(value_parser = clap::value_parser!(std::path::PathBuf), short = 's', long = "steam-library", alias = "steam")]
    pub steam_library: PathBuf,

    /// Set game version (useful for metadata)
    #[arg(long = "game-version", alias = "gv")]
    pub game_version: String,

    /// Documentation locale to generate from
    #[arg(short, long, default_value = "en-US")]
    pub locale: String,

    /// Working directory (should generally be left empty, will default to a tempdir.)
    #[arg(short, long, value_parser = clap::value_parser!(std::path::PathBuf))]
    pub workdir: Option<PathBuf>,

    /// Output file path (will output to a zip file, defaulting to "./satisfactory-<version>-<locale>.zip")
    #[arg(short, long, value_parser = clap::value_parser!(std::path::PathBuf))]
    pub output: Option<PathBuf>,

    /// Run in NixOS compatibility mode (using steam-run FHS)
    #[arg(long)]
    pub fhs: bool,
}

impl Cli {
    pub fn output(&self) -> PathBuf {
        if let Some(output) = self.output.clone() {
            output
        } else {
            PathBuf::from(format!(
                "satisfactory-{}-{}.zip",
                self.game_version, self.locale
            ))
        }
    }

    pub fn game_path(&self) -> PathBuf {
        self.steam_library
            .join("steamapps")
            .join("common")
            .join("Satisfactory")
    }

    pub fn community_resources_path(&self) -> PathBuf {
        self.game_path().join("CommunityResources")
    }

    pub fn docs_path(&self) -> PathBuf {
        self.community_resources_path()
            .join("Docs")
            .join(format!("{}.json", self.locale))
    }

    pub fn paks_path(&self) -> PathBuf {
        self.game_path().join("FactoryGame/Content/Paks")
    }
}

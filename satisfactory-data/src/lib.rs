#![warn(missing_docs)]

//! A wrapper around [Satisfactory](https://www.satisfactorygame.com/)'s community resources.
//!
//! Provides normalized abstractions of Satisfactory's [Community Resources](https://satisfactory.wiki.gg/wiki/Community_resources), specifically Docs.json and access to extracted game icons. 
//! The associated asset packs are provided on this project's GitHub [here](https://github.com/dax-dot-gay/satisfactory-data/tree/main/data).
//! This project aims to support the most recent production release of Satisfactory, though the associated extraction tool should be sufficiently version-agnostic to parse other versions.
//! It also supports basic modification of the parsed data, allowing the user to create/modify existing documentation entries.

pub mod types;
pub use types::{
    BuildableItem, DescriptionItem, RecipeItem, ResearchItem,
    id::{self, Id},
};

mod error;
pub mod registry;

pub use error::Error;
#[allow(unused)]
pub(crate) use error::Result;

pub use registry::{Registry, RegistryItem};

#[cfg(feature = "async")]
pub(crate) use tokio;

#[cfg(feature = "schemars")]
pub(crate) use schemars;

#[cfg(feature = "specta-1")]
pub(crate) use specta_01 as specta;

#[cfg(feature = "specta-2")]
pub(crate) use specta_02 as specta;

pub(crate) use serde;

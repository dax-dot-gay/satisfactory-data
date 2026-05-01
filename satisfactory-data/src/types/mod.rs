//! Contains documentation types and a strongly-typed ID type.
#![allow(missing_docs)]

pub mod research;
pub use research::ResearchItem;

pub mod description;
pub use description::DescriptionItem;

pub mod buildable;
pub use buildable::BuildableItem;

pub mod recipe;
pub use recipe::RecipeItem;

pub mod id;
pub use id::*;
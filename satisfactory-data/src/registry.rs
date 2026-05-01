use std::collections::HashMap;

use crate::types::*;
use chrono::Utc;
use satisfactory_data_macros::model;

#[model(Default)]
pub struct RegistryMeta {
    pub generated: chrono::DateTime<Utc>,
    pub game_version: String,
    pub experimental: bool
}

#[model(Default)]
pub struct RawRegistry {
    pub metadata: RegistryMeta,

    #[serde(default)]
    pub recipes: HashMap<String, RecipeItem>,

    #[serde(default)]
    pub research: HashMap<String, ResearchItem>,

    #[serde(default)]
    pub descriptions: HashMap<String, DescriptionItem>,

    #[serde(default)]
    pub buildables: HashMap<String, BuildableItem>,
}

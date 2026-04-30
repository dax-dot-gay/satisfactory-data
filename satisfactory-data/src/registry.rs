use std::collections::HashMap;

use crate::types::*;
use satisfactory_data_macros::model;

#[model(Default)]
pub struct RawRegistry {
    #[serde(default)]
    pub recipes: HashMap<String, RecipeItem>,

    #[serde(default)]
    pub research: HashMap<String, ResearchItem>,

    #[serde(default)]
    pub descriptions: HashMap<String, DescriptionItem>,

    #[serde(default)]
    pub buildables: HashMap<String, BuildableItem>,
}

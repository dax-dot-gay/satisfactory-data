pub mod building;
pub mod description;
pub mod generator;
pub mod recipe;
pub mod research;
pub mod uestring;
pub mod utility;

use std::collections::HashMap;

pub use building::{BuildingFuelType, BuildingItem};
pub use description::{
    DescriptionEquipmentSlot, DescriptionGasType, DescriptionItem, DescriptionStackSize,
    DescriptionType,
};
pub use generator::{Generated, Generator};
pub use recipe::RecipeItem;
pub use research::{ResearchItem, ResearchType, ResearchUnlock};
use serde::{Deserialize, Serialize};
use specta::Type;
pub use utility::{
    AssetReference, ClassReference, Coercion, IconPath, NormalizedString, parse_docs_json,
};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BeanCounterData {
    pub research: HashMap<String, ResearchItem>,
    pub descriptions: HashMap<String, DescriptionItem>,
    pub buildables: HashMap<String, BuildingItem>,
    pub recipes: HashMap<String, RecipeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "item_type")]
#[serde(rename_all = "snake_case")]
pub enum BeanCounterItem {
    Research(ResearchItem),
    Description(DescriptionItem),
    Buildable(BuildingItem),
    Recipe(RecipeItem),
}

impl BeanCounterData {
    pub fn get_id(&self, id: String) -> Option<BeanCounterItem> {
        if let Some(item) = self.research.get(&id) {
            return Some(BeanCounterItem::Research(item.clone()));
        }

        if let Some(item) = self.descriptions.get(&id) {
            return Some(BeanCounterItem::Description(item.clone()));
        }

        if let Some(item) = self.buildables.get(&id) {
            return Some(BeanCounterItem::Buildable(item.clone()));
        }

        if let Some(item) = self.recipes.get(&id) {
            return Some(BeanCounterItem::Recipe(item.clone()));
        }

        None
    }
}

use satisfactory_data_macros::model;

use crate::types::Id;

#[model]
pub struct RecipeResource {
    pub item: Id,
    pub amount: u64,
}

#[model]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RecipeMachine {
    BuildGun {},
    HubWorkbench {},
    EquipmentWorkshop {},
    Machine { id: Id },
}

#[model]
pub struct RecipeItem {
    pub id: Id,
    pub display_name: String,
    pub inputs: Vec<RecipeResource>,
    pub outputs: Vec<RecipeResource>,
    pub duration: f32,
    pub machines: Vec<RecipeMachine>,
}

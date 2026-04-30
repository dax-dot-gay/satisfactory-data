use satisfactory_data_macros::model;

#[model]
pub struct RecipeResource {
    pub item: String,
    pub amount: u64,
}

#[model]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RecipeMachine {
    BuildGun {},
    HubWorkbench {},
    EquipmentWorkshop {},
    Machine { id: String },
}

impl From<String> for RecipeMachine {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BpWorkBenchComponentC" | "FgBuildableAutomatedWorkBench" => Self::HubWorkbench {},
            "BpBuildGunC" | "FgBuildGun" => Self::BuildGun {},
            "BpWorkshopComponentC" => Self::EquipmentWorkshop {},
            machine => Self::Machine {
                id: machine.to_string(),
            },
        }
    }
}

#[model]
pub struct RecipeItem {
    pub id: String,
    pub display_name: String,
    pub inputs: Vec<RecipeResource>,
    pub outputs: Vec<RecipeResource>,
    pub duration: f32,
    pub machines: Vec<RecipeMachine>,
}

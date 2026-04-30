use satisfactory_data_macros::model;

#[model]
pub struct ResearchCost {
    pub item: String,
    pub amount: u64,
}

#[model]
pub struct ScannableObjectType {
    pub item: String,
    pub allowed_scanners: Vec<String>,
}

#[model]
#[serde(rename_all = "snake_case", tag = "class")]
pub enum ResearchUnlock {
    Recipe {
        #[serde(default)]
        recipes: Vec<String>,
    },
    ScannableResource {
        #[serde(default)]
        resources: Vec<String>,
    },
    Schematic {
        #[serde(default)]
        schematics: Vec<String>,
    },
    CentralStorageUpload {
        // This amount seems to be the amount of 1/25 second ticks needed per item
        // So actual items/min is: 25/<amount> * 60
        #[serde(default)]
        amount: u64,
    },
    InventorySlot {
        #[serde(default, alias = "resources")]
        slots: u64,
    },
    ToolSlot {
        #[serde(default, alias = "amount")]
        slots: u64,
    },
    Emote {
        #[serde(default)]
        emotes: Vec<String>,
    },
    ScannableObject {
        #[serde(default, alias = "resources")]
        objects: Vec<ScannableObjectType>,
    },
    BoomboxTape {
        #[serde(default)]
        tapes: Vec<String>,
    },
    Map {},
    Info {},
    BuildEfficiency {},
    Blueprints {},
    ProductionBoost {},
    Overclocking {},
    CentralStorageItems {},
    CentralStorageSlots {},
}

#[model]
#[serde(rename_all = "snake_case")]
pub enum ResearchType {
    MamResearch,
    AlternateRecipe,
    Milestone,
    ResourceSink,
}

#[model]
pub struct ResearchItem {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub research_type: ResearchType,
    pub cost: Vec<ResearchCost>,
    pub unlocks: Vec<ResearchUnlock>,
    pub tier: u64,
}

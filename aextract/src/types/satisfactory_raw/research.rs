use serde::{Deserialize, Serialize};
use specta::Type;

use super::{uestring::UE, ClassReference, Coercion, NormalizedString};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct ScannableObject {
    #[serde(alias = "ItemDescriptor")]
    pub item: ClassReference,

    #[serde(alias = "ActorsAllowedToScan")]
    pub allowed_scanners: Vec<ClassReference>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
#[serde(tag = "Class")]
#[serde(rename_all = "snake_case")]
#[serde(rename_all_fields = "snake_case")]
pub enum ResearchUnlock {
    #[serde(alias = "BP_UnlockRecipe_C")]
    Recipe {
        #[serde(alias = "mRecipes")]
        recipes: UE<Vec<ClassReference>>
    },
    #[serde(alias = "BP_UnlockBlueprints_C")]
    Blueprints {
        #[serde(alias = "mRecipes")]
        recipes: UE<Vec<ClassReference>>
    },
    #[serde(alias = "BP_UnlockSchematic_C")]
    Schematic {
        #[serde(alias = "mSchematics")]
        schematics: UE<Vec<ClassReference>>
    },
    #[serde(alias = "BP_UnlockScannableResource_C")]
    ScannableResource {
        #[serde(alias = "mResourcesToAddToScanner")]
        resources: UE<Vec<ClassReference>>
    },
    #[serde(alias = "BP_UnlockScannableObject_C")]
    ScannableObject {
        #[serde(alias = "mScannableObjects")]
        resources: UE<Vec<ScannableObject>>
    },
    #[serde(alias = "BP_UnlockInventorySlot_C")]
    InventorySlot {
        #[serde(alias = "mNumInventorySlotsToUnlock")]
        resources: Coercion
    },
    #[serde(alias = "BP_UnlockInfoOnly_C")]
    Info {},
    #[serde(alias = "FGUnlockTape")]
    BoomboxTape {
        #[serde(alias = "mTapeUnlocks")]
        tapes: UE<Vec<ClassReference>>
    },
    #[serde(alias = "BP_UnlockArmEquipmentSlot_C")]
    ToolSlot {
        #[serde(alias = "mNumArmEquipmentSlotsToUnlock")]
        amount: Coercion
    },
    #[serde(alias = "BP_UnlockEmote_C")]
    Emote {
        #[serde(alias = "mEmotes")]
        emotes: UE<Vec<ClassReference>>
    },
    #[serde(alias = "BP_UnlockBuildProductionBoost_C")]
    ProductionBoost {},
    #[serde(alias = "BP_UnlockCentralStorageUploadSpeed_C")]
    CentralStorageUpload {
        #[serde(alias = "mUploadSpeedPercentageDecrease")]
        amount: Coercion
    },
    #[serde(alias = "BP_UnlockBuildEfficiency_C")]
    BuildEfficiency {},
    #[serde(alias = "BP_UnlockCentralStorageItemLimit_C")]
    CentralStorageItems {
        #[serde(alias = "mItemStackLimitIncrease")]
        amount: Coercion
    },
    #[serde(alias = "BP_UnlockCentralStorageUploadSlots_C")]
    CentralStorageSlots {
        #[serde(alias = "mNumSlotsToUnlock")]
        amount: Coercion
    },
    #[serde(alias = "BP_UnlockBuildOverclock_C")]
    Overclocking {},
    #[serde(alias = "BP_UnlockMap_C")]
    Map {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ResearchType {
    #[serde(alias = "EST_MAM")]
    MamResearch,

    #[serde(alias = "EST_Milestone")]
    Milestone,

    #[serde(alias = "EST_Alternate")]
    #[serde(alias = "EST_Custom")]
    AlternateRecipe,

    #[serde(alias = "EST_ResourceSink")]
    ResourceSink
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct ResearchItemCost {
    #[serde(alias = "ItemClass")]
    pub item: ClassReference,

    #[serde(alias = "Amount")]
    pub amount: Coercion,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct ResearchItem {
    #[serde(alias = "ClassName")]
    pub id: NormalizedString,

    #[serde(alias = "mDisplayName")]
    pub display_name: String,

    #[serde(alias = "mDescription")]
    pub description: String,

    #[serde(alias = "mType")]
    pub research_type: ResearchType,

    #[serde(alias = "mCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<UE<Vec<ResearchItemCost>>>,

    #[serde(alias = "mUnlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlocks: Option<Vec<ResearchUnlock>>,

    #[serde(alias = "mSubCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_categories: Option<UE<Vec<ClassReference>>>,

    #[serde(alias = "mTechTier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Coercion>,
}
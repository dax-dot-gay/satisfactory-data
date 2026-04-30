use serde::{Deserialize, Serialize};
use specta::Type;

use super::{Coercion, NormalizedString, AssetReference};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionType {
    #[serde(alias = "RF_LIQUID")]
    Liquid,

    #[serde(alias = "RF_SOLID")]
    Item,

    #[serde(alias = "RF_GAS")]
    Gas,

    #[serde(alias = "RF_INVALID")]
    Building,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionStackSize {
    #[serde(alias = "SS_FLUID")] // Liquid (no stack)
    Liquid,

    #[serde(alias = "SS_ONE")] // Single item
    One,

    #[serde(alias = "SS_SMALL")] // 50 items
    Small,

    #[serde(alias = "SS_MEDIUM")] // 100 items or N/A
    Medium,

    #[serde(alias = "SS_LARGE")] // 200 items
    Large,

    #[serde(alias = "SS_HUGE")]
    #[serde(alias = "SS_BIG")] // 500 items
    Huge,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionGasType {
    #[serde(alias = "GT_NORMAL")] // Normal or N/A
    Normal,

    #[serde(alias = "GT_ENERGY")] // Energetic gas
    Energy
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionEquipmentSlot {
    #[serde(alias = "ES_ARMS")]
    Arms,

    #[serde(alias = "ES_BACK")]
    Back,

    #[serde(alias = "ES_LEGS")]
    Legs,

    #[serde(alias = "ES_HEAD")]
    Head,

    #[serde(alias = "ES_BODY")]
    Body,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct DescriptionItem {
    #[serde(alias = "ClassName")]
    pub id: NormalizedString,

    #[serde(alias = "mDisplayName")]
    pub display_name: String,

    #[serde(alias = "mDescription")]
    pub description: String,

    #[serde(alias = "mForm")]
    pub description_type: DescriptionType,

    #[serde(alias = "mStackSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_size: Option<DescriptionStackSize>,

    #[serde(alias = "mGasType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_type: Option<DescriptionGasType>,

    #[serde(alias = "mIsAlienItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_alien: Option<Coercion>,

    #[serde(alias = "mEnergyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_value: Option<Coercion>,

    #[serde(alias = "mRadioactiveDecay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radioactivity: Option<Coercion>,

    #[serde(alias = "mHealthGain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_gain: Option<Coercion>,

    #[serde(alias = "mPowerConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption: Option<Coercion>,

    #[serde(alias = "mSmallIcon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<AssetReference>,

    #[serde(alias = "mPersistentBigIcon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_icon: Option<AssetReference>,

    #[serde(alias = "mEquipmentSlot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_slot: Option<DescriptionEquipmentSlot>,

    #[serde(alias = "mAmountOfWaste")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_waste: Option<Coercion>,

    #[serde(alias = "mResourceSinkPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_sink_points: Option<Coercion>,
}
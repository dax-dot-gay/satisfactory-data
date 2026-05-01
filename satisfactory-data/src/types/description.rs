use satisfactory_data_macros::model;

use crate::types::Id;

#[model]
#[serde(rename_all = "snake_case")]
pub enum DescriptionType {
    Building,
    Item,
    Liquid,
    Gas,
}

#[model]
#[serde(rename_all = "snake_case")]
pub enum DescriptionStackSize {
    One,
    Small,
    Medium,
    Large,
    Huge,
    Liquid,
}

#[model]
#[serde(rename_all = "snake_case")]
pub enum DescriptionGasType {
    Normal,
    Energy,
}

#[model]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum DescriptionPowerConsumption {
    Static { amount: f32 },
    Variable { min: f32, max: f32 },
}

#[model]
pub struct DescriptionItem {
    pub id: Id,
    pub description_type: DescriptionType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub display_name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub stack_size: Option<DescriptionStackSize>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub gas_type: Option<DescriptionGasType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_alien: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub energy_value: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub radioactivity: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub health_gain: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_consumption: Option<DescriptionPowerConsumption>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub icon: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub big_icon: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub generated_waste: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub resource_sink_points: Option<f32>,
}

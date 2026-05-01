use satisfactory_data_macros::model;

use crate::types::Id;

#[model]
pub struct BuildableFuelType {
    pub primary_resource: Id,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub secondary_resource: Option<Id>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub byproduct_resource: Option<Id>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub byproduct_amount: Option<f32>,
}

#[model]
pub struct BuildableItem {
    pub id: Id,
    pub display_name: String,
    pub description: String,
    #[serde(default)]
    pub custom: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_adaptive_generator: Option<bool>,

    #[serde(default)]
    pub fuels: Vec<BuildableFuelType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_production: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_consumption: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_consumption_exponent: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_consumption_boost: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_consumption_minimum: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub power_consumption_maximum: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_overclockable: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_boostable: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_sinkable: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_patternable: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_colorable: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(any(feature = "specta-1", feature = "specta-2"), specta(optional))]
    pub is_interactable: Option<bool>,
}

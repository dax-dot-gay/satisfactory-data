use serde::{Deserialize, Serialize};
use specta::Type;

use super::{Coercion, NormalizedString};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct BuildingFuelType {
    #[serde(alias = "mFuelClass")]
    pub primary_resource: NormalizedString,

    #[serde(alias = "mSupplementalResourceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_resource: Option<NormalizedString>,

    #[serde(alias = "mByproduct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byproduct_resource: Option<NormalizedString>,

    #[serde(alias = "mByproductAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byproduct_amount: Option<Coercion>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct BuildingItem {
    #[serde(alias = "ClassName")]
    pub id: NormalizedString,

    #[serde(alias = "mDisplayName")]
    pub display_name: String,

    #[serde(alias = "mDescription")]
    pub description: String,

    #[serde(alias = "mIsFullBlast")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_generator: Option<Coercion>,

    #[serde(alias = "mFuel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuels: Option<Vec<BuildingFuelType>>,

    #[serde(alias = "mPowerProduction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_production: Option<Coercion>,

    #[serde(alias = "mPowerConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption: Option<Coercion>,

    #[serde(alias = "mPowerConsumptionExponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption_exponent: Option<Coercion>,

    #[serde(alias = "mProductionBoostPowerConsumptionExponent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption_boost: Option<Coercion>,

    #[serde(alias = "mEstimatedMininumPowerConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption_minimum: Option<Coercion>,

    #[serde(alias = "mEstimatedMaximumPowerConsumption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_consumption_maximum: Option<Coercion>,

    #[serde(alias = "mCanChangePotential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overclockable: Option<Coercion>,

    #[serde(alias = "mCanChangeProductionBoost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boostable: Option<Coercion>,

    #[serde(alias = "mCanBeDiscarded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sinkable: Option<Coercion>,

    #[serde(alias = "mAllowPatterning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pattern: Option<Coercion>,

    #[serde(alias = "mAllowColoring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_color: Option<Coercion>,

    #[serde(alias = "mIsUseable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactable: Option<Coercion>,
}
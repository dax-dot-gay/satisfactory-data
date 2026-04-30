use serde::{Deserialize, Serialize};
use specta::Type;

use super::{uestring::UE, ClassReference, Coercion, NormalizedString};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct ItemReference {
    #[serde(alias = "ItemClass")]
    pub item: ClassReference,

    #[serde(alias = "Amount")]
    pub amount: Coercion
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
pub struct RecipeItem {
    #[serde(alias = "ClassName")]
    pub id: NormalizedString,

    #[serde(alias = "mDisplayName")]
    pub display_name: String,

    #[serde(alias = "mIngredients")]
    pub ingredients: UE<Vec<ItemReference>>,

    #[serde(alias = "mProduct")]
    pub product: UE<Vec<ItemReference>>,

    #[serde(alias = "mManufactoringDuration")]
    #[serde(alias = "mManufacturingDuration")]
    pub duration: Coercion,

    #[serde(alias = "mProducedIn")]
    pub machine: UE<Vec<ClassReference>>
}
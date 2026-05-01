use std::fmt::Debug;

use convert_case::{Case, Casing};
use satisfactory_data::{
    registry::RawRegistry, types as clean, types::research::ResearchUnlock as CUnlock,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::types::satisfactory_raw::{self as raw, Generated, research::ResearchUnlock as RUnlock};

fn clean_id(id: impl Into<String>) -> String {
    let mut progress = id.into();
    progress = if progress.starts_with("Research") {
        progress.replacen("Research", "research/", 1)
    } else if progress.starts_with("Schematic") {
        progress.replacen("Schematic", "research:schematic/", 1)
    } else if progress.starts_with("Desc") {
        progress.replacen("Desc", "desc/", 1)
    } else if progress.starts_with("FoundationConcretePolished") {
        ("desc/".to_string() + progress.as_str()).to_string()
    } else if progress.starts_with("BpEquipmentDescriptor") {
        progress.replacen("BpEquipmentDescriptor", "desc:equipment/", 1)
    } else if progress.starts_with("BpEqDesc") {
        progress.replacen("BpEqDesc", "desc:equipment/", 1)
    } else if progress.starts_with("BpItemDescriptor") {
        progress.replacen("BpItemDescriptor", "desc:equipment/", 1)
    } else if progress.starts_with("Build") {
        progress.replacen("Build", "build/", 1)
    } else if progress.starts_with("Recipe") {
        progress.replacen("Recipe", "recipe/", 1)
    } else if progress.starts_with("Tape") {
        progress.replacen("Tape", "special:tape/", 1)
    } else if progress.starts_with("FgBuildableRadarTower") {
        String::from("special:scanner/radar_tower")
    } else if progress.starts_with("FgObjectScanner") {
        String::from("special:scanner/object_scanner")
    } else if progress.starts_with("CustomizerUnlock") {
        progress.replacen("CustomizerUnlock", "special:customizer/", 1)
    } else if progress.starts_with("Emote") {
        progress.replacen("Emote", "special:emote/", 1)
    } else {
        println!("Unhandled ID format: {}", progress);
        progress
    };

    if progress.ends_with("C'") {
        progress.truncate(progress.len() - 2);
    } else if progress.ends_with("C") {
        progress.truncate(progress.len() - 1);
    }

    progress.to_case(Case::Snake)
}

fn extract_ue<T: Clone + Debug + Serialize + DeserializeOwned + PartialEq + Default>(
    ue: raw::uestring::UE<T>,
) -> T {
    return AsRef::<Option<T>>::as_ref(&ue).clone().unwrap_or_default();
}

fn extract_crs(crs: raw::uestring::UE<Vec<raw::utility::ClassReference>>) -> Vec<String> {
    return extract_ue(crs)
        .into_iter()
        .map(|v| clean_id(Into::<String>::into(v)))
        .collect();
}

fn clean_research(item: raw::ResearchItem) -> clean::ResearchItem {
    clean::ResearchItem {
        id: clean_id(item.id.clone()),
        display_name: item.display_name.clone(),
        description: item.description.clone(),
        research_type: item.research_type.clone().into(),
        cost: match item.cost.clone() {
            Some(items) => extract_ue(items)
                .into_iter()
                .map(|v| clean::research::ResearchCost {
                    item: clean_id(Into::<String>::into(v.item)),
                    amount: v.amount.into(),
                })
                .collect(),
            None => Vec::new(),
        },
        unlocks: match item.unlocks.clone() {
            Some(unlocks) => unlocks
                .into_iter()
                .map(|unlock| match unlock {
                    RUnlock::Recipe { recipes } => CUnlock::Recipe {
                        recipes: extract_crs(recipes),
                    },
                    RUnlock::Blueprints { .. } => CUnlock::Blueprints {},
                    RUnlock::Schematic { schematics } => CUnlock::Schematic {
                        schematics: extract_crs(schematics),
                    },
                    RUnlock::ScannableResource { resources } => CUnlock::ScannableResource {
                        resources: extract_crs(resources),
                    },
                    RUnlock::ScannableObject { resources } => CUnlock::ScannableObject {
                        objects: extract_ue(resources)
                            .into_iter()
                            .map(|v| clean::research::ScannableObjectType {
                                item: clean_id(Into::<String>::into(v.item.clone())),
                                allowed_scanners: v
                                    .allowed_scanners
                                    .clone()
                                    .into_iter()
                                    .map(|v| clean_id(Into::<String>::into(v)))
                                    .collect(),
                            })
                            .collect(),
                    },
                    RUnlock::InventorySlot { resources } => CUnlock::InventorySlot {
                        slots: resources.into(),
                    },
                    RUnlock::Info {} => CUnlock::Info {},
                    RUnlock::BoomboxTape { tapes } => CUnlock::BoomboxTape {
                        tapes: extract_crs(tapes),
                    },
                    RUnlock::ToolSlot { amount } => CUnlock::ToolSlot {
                        slots: amount.into(),
                    },
                    RUnlock::Emote { emotes } => CUnlock::Emote {
                        emotes: extract_crs(emotes),
                    },
                    RUnlock::ProductionBoost {} => CUnlock::ProductionBoost {},
                    RUnlock::CentralStorageUpload { amount } => CUnlock::CentralStorageUpload {
                        amount: amount.into(),
                    },
                    RUnlock::BuildEfficiency {} => CUnlock::BuildEfficiency {},
                    RUnlock::CentralStorageItems { .. } => CUnlock::CentralStorageItems {},
                    RUnlock::CentralStorageSlots { .. } => CUnlock::CentralStorageSlots {},
                    RUnlock::Overclocking {} => CUnlock::Overclocking {},
                    RUnlock::Map {} => CUnlock::Map {},
                })
                .collect(),
            None => Vec::new(),
        },
        tier: item.tier.and_then(|v| Some(v.into())).unwrap_or(0),
    }
}

fn clean_recipe(item: raw::RecipeItem) -> clean::RecipeItem {
    clean::RecipeItem {
        id: clean_id(item.id.clone()),
        display_name: item.display_name.clone(),
        inputs: extract_ue(item.ingredients.clone())
            .into_iter()
            .map(|v| clean::recipe::RecipeResource {
                item: clean_id(v.item.clone()),
                amount: v.amount.into(),
            })
            .collect(),
        outputs: extract_ue(item.product.clone())
            .into_iter()
            .map(|v| clean::recipe::RecipeResource {
                item: clean_id(v.item.clone()),
                amount: v.amount.into(),
            })
            .collect(),
        duration: item.duration.clone().into(),
        machines: extract_ue(item.machine.clone())
            .into_iter()
            .map(|v| {
                let mid: String = v.into();
                match mid.as_str() {
                    "BpWorkBenchComponentC" | "FgBuildableAutomatedWorkBench" => {
                        clean::recipe::RecipeMachine::HubWorkbench {}
                    }
                    "BpBuildGunC" | "FgBuildGun" => clean::recipe::RecipeMachine::BuildGun {},
                    "BpWorkshopComponentC" => clean::recipe::RecipeMachine::EquipmentWorkshop {},
                    machine => clean::recipe::RecipeMachine::Machine {
                        id: clean_id(machine),
                    },
                }
            })
            .collect(),
    }
}

fn clean_description(item: raw::DescriptionItem) -> clean::DescriptionItem {
    clean::DescriptionItem {
        id: clean_id(item.id.clone()),
        description_type: item.description_type.clone().into(),
        display_name: if item.display_name.is_empty() {
            None
        } else {
            Some(item.display_name.clone())
        },
        description: if item.description.is_empty() {
            None
        } else {
            Some(item.description.clone())
        },
        stack_size: item.stack_size.clone().and_then(|v| Some(v.into())),
        gas_type: item.gas_type.clone().and_then(|v| Some(v.into())),
        is_alien: item.is_alien.and_then(|v| Some(v.into())),
        energy_value: item.energy_value.and_then(|v| Some(v.into())),
        radioactivity: item.radioactivity.and_then(|v| Some(v.into())),
        health_gain: item.health_gain.and_then(|v| Some(v.into())),
        power_consumption: item.power_consumption.clone().and_then(|v| {
            Some(match v {
                raw::Coercion::Float(v) => {
                    clean::description::DescriptionPowerConsumption::Static { amount: v }
                }
                raw::Coercion::Integer(v) => {
                    clean::description::DescriptionPowerConsumption::Static { amount: v as f32 }
                }
                raw::Coercion::String(Some(content)) => {
                    if let Some((min, max)) = content.trim_matches(['(', ')']).split_once(",") {
                        clean::description::DescriptionPowerConsumption::Variable {
                            min: min
                                .split_once("=")
                                .and_then(|(_, v)| Some(v.parse::<f32>().unwrap_or(0.0)))
                                .unwrap_or(0.0),
                            max: max
                                .split_once("=")
                                .and_then(|(_, v)| Some(v.parse::<f32>().unwrap_or(0.0)))
                                .unwrap_or(0.0),
                        }
                    } else {
                        clean::description::DescriptionPowerConsumption::Static { amount: 0.0 }
                    }
                }
                _ => clean::description::DescriptionPowerConsumption::Static { amount: 0.0 },
            })
        }),
        icon: item.icon.and_then(|v| v.asset_id),
        big_icon: item.big_icon.and_then(|v| v.asset_id),
        generated_waste: item.generated_waste.clone().and_then(|v| Some(v.into())),
        resource_sink_points: item
            .resource_sink_points
            .clone()
            .and_then(|v| Some(v.into())),
    }
}

fn clean_buildable(item: raw::BuildingItem) -> clean::BuildableItem {
    clean::BuildableItem {
        id: clean_id(item.id.clone()),
        display_name: item.display_name.clone(),
        description: item.description.clone(),
        is_adaptive_generator: item.adaptive_generator.clone().and_then(|v| Some(v.into())),
        fuels: item.fuels.clone().and_then(|fuels| Some(fuels.into_iter().map(|fuel| clean::buildable::BuildableFuelType {
            primary_resource: clean_id(fuel.primary_resource.clone()),
            secondary_resource: fuel.secondary_resource.clone().and_then(|v| {
                let vs: String = v.into();
                if vs.is_empty() {
                    None
                } else {
                    Some(clean_id(vs))
                }
            }),
            byproduct_resource: fuel.byproduct_resource.clone().and_then(|v| {
                let vs: String = v.into();
                if vs.is_empty() {
                    None
                } else {
                    Some(clean_id(vs))
                }
            }),
            byproduct_amount: fuel.byproduct_amount.and_then(|v| Some(v.into())),
        }).collect())).unwrap_or(Vec::new()),
        power_production: item.power_production.clone().and_then(|v| Some(v.into())),
        power_consumption: item.power_consumption.clone().and_then(|v| Some(v.into())),
        power_consumption_exponent: item.power_consumption_exponent.clone().and_then(|v| Some(v.into())),
        power_consumption_boost: item.power_consumption_boost.clone().and_then(|v| Some(v.into())),
        power_consumption_minimum: item.power_consumption_minimum.clone().and_then(|v| Some(v.into())),
        power_consumption_maximum: item.power_consumption_maximum.clone().and_then(|v| Some(v.into())),
        is_overclockable: item.overclockable.clone().and_then(|v| Some(v.into())),
        is_boostable: item.boostable.clone().and_then(|v| Some(v.into())),
        is_sinkable: item.sinkable.clone().and_then(|v| Some(v.into())),
        is_patternable: item.can_pattern.clone().and_then(|v| Some(v.into())),
        is_colorable: item.can_color.clone().and_then(|v| Some(v.into())),
        is_interactable: item.interactable.clone().and_then(|v| Some(v.into())),
    }
}

pub fn generate_clean_data(data: Generated) -> RawRegistry {
    let mut registry = RawRegistry::default();
    registry.research = data
        .research
        .clone()
        .into_iter()
        .map(|(id, item)| (clean_id(id), clean_research(item)))
        .collect();

    registry.recipes = data
        .recipes
        .clone()
        .into_iter()
        .map(|(id, item)| (clean_id(id), clean_recipe(item)))
        .collect();

    registry.descriptions = data
        .descriptions
        .clone()
        .into_iter()
        .map(|(id, item)| (clean_id(id), clean_description(item)))
        .collect();

    registry.buildables = data
        .buildables
        .clone()
        .into_iter()
        .map(|(id, item)| (clean_id(id), clean_buildable(item)))
        .collect();

    registry
}

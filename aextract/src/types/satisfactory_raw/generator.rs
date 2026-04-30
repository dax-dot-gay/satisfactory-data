use crate::types::satisfactory_raw::{BuildingItem, DescriptionItem, RecipeItem, ResearchItem};
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Generated {
    pub research: HashMap<String, ResearchItem>,
    pub descriptions: HashMap<String, DescriptionItem>,
    pub buildables: HashMap<String, BuildingItem>,
    pub recipes: HashMap<String, RecipeItem>,
}

#[derive(Debug, Clone)]
pub struct Generator {
    data: Generated,
    raw: Value,
}

/*
Generate data from:
- Research + Schematic
- Desc + BP
- Build
- Recipe
*/
impl Generator {
    pub fn new(data: Value) -> Self {
        Generator {
            data: Generated {
                research: HashMap::new(),
                recipes: HashMap::new(),
                descriptions: HashMap::new(),
                buildables: HashMap::new(),
            },
            raw: data.clone(),
        }
    }

    fn handle_research(&mut self, name: String, data: Value) {
        let des_result = serde_json::from_value::<ResearchItem>(data.clone());
        if let Ok(deserialized) = des_result {
            if !deserialized.display_name.starts_with("Discontinued") {
                self.data.research.insert(name, deserialized);
            }
        }
    }

    fn handle_description(&mut self, name: String, data: Value) {
        let des_result = serde_json::from_value::<DescriptionItem>(data.clone());
        if let Ok(deserialized) = des_result {
            if !deserialized.display_name.starts_with("Discontinued") {
                self.data.descriptions.insert(name, deserialized);
            }
        }
    }

    fn handle_recipe(&mut self, name: String, data: Value) {
        let des_result = serde_json::from_value::<RecipeItem>(data.clone());
        if let Ok(deserialized) = des_result {
            if !deserialized.display_name.starts_with("Discontinued") {
                self.data.recipes.insert(name, deserialized);
            }
        }
    }

    fn handle_buildable(&mut self, name: String, data: Value) {
        let des_result = serde_json::from_value::<BuildingItem>(data.clone());
        if let Ok(deserialized) = des_result {
            if !deserialized.display_name.starts_with("Discontinued") {
                self.data.buildables.insert(name, deserialized);
            }
        }
    }

    pub fn generate(&mut self) -> Generated {
        for category in self
            .clone()
            .raw
            .as_array()
            .expect("Expected array at docs root")
            .iter()
            .map(|v| {
                v.as_object()
                    .expect("Expected object in root array")
                    .clone()
            })
        {
            let classes = category
                .get("Classes")
                .expect("Expected Classes key")
                .as_array()
                .expect("Expected array of classes");

            for class in classes {
                let ctype = class
                    .get("ClassName")
                    .expect("Expected ClassName")
                    .as_str()
                    .expect("Expected string")
                    .split_once("_")
                    .expect("Expected _")
                    .0;
                let cname = class
                    .get("ClassName")
                    .expect("Expected ClassName")
                    .as_str()
                    .expect("Expected string")
                    .to_string()
                    .to_case(Case::Pascal);
                match ctype {
                    "Desc" => self.handle_description(cname, class.clone()),
                    "BP" => self.handle_description(cname, class.clone()),
                    "Research" => self.handle_research(cname, class.clone()),
                    "Schematic" => self.handle_research(cname, class.clone()),
                    "Recipe" => self.handle_recipe(cname, class.clone()),
                    "Build" => self.handle_buildable(cname, class.clone()),
                    _ => (),
                }
            }
        }
        self.data.clone()
    }
}

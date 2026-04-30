use std::{collections::HashMap, error::Error, fmt::Debug};

use crate::types::satisfactory_raw::Coercion;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use specta::Type;

#[derive(Clone, Debug)]
pub enum UEString {
    String(String),
    Atomic(String),
    Boolean(bool),
    KeyValue(String, Box<UEString>),
    Array(Vec<Box<UEString>>),
    None,
}

impl UEString {
    pub fn from_string(source: String) -> UEString {
        if source.len() == 0 {
            return UEString::None;
        }
        if source.starts_with("(") && source.ends_with(")") {
            if source.len() == 2 {
                return UEString::Array(Vec::new());
            }
            let mut context = String::new();
            let mut items = Vec::<Box<UEString>>::new();
            let mut block_counter = 0;
            for ch in &source.chars().collect::<Vec<char>>()[1..source.len()] {
                match ch {
                    '(' => {
                        block_counter += 1;
                        context += "(";
                    }
                    ')' => {
                        if block_counter > 0 {
                            block_counter -= 1;
                            context += ")";
                        } else {
                            items.push(Box::new(UEString::from_string(context.clone())));
                            context.clear();
                        }
                    }
                    ',' => {
                        if block_counter == 0 {
                            items.push(Box::new(UEString::from_string(context.clone())));
                            context.clear();
                        } else {
                            context += ",";
                        }
                    }
                    e => {
                        context += &e.to_string();
                    }
                }
            }
            return UEString::Array(items);
        } else if source.starts_with("\"") && source.ends_with("\"") {
            return UEString::String(source[1..source.len()].to_string());
        } else if source.contains("=") {
            let (key, val) = source.split_once("=").unwrap();
            return UEString::KeyValue(
                key.to_string(),
                Box::new(UEString::from_string(val.to_string())),
            );
        } else if source == "True" || source == "False" {
            return match source.as_str() {
                "True" => UEString::Boolean(true),
                "False" => UEString::Boolean(false),
                _ => UEString::Atomic(source),
            };
        } else {
            return UEString::Atomic(source);
        }
    }

    pub fn to_value(&self) -> Result<Value, Box<dyn Error>> {
        Ok(match self.clone() {
            UEString::String(v) => Value::String(v),
            UEString::Boolean(v) => Value::Bool(v),
            UEString::Atomic(v) => {
                serde_json::to_value(serde_json::from_str::<Coercion>(v.as_str())?)?
            }
            UEString::Array(v) => {
                if v.iter().all(|member| match **member {
                    UEString::KeyValue(_, _) => true,
                    _ => false,
                }) {
                    let mut result: HashMap<String, Value> = HashMap::new();
                    for (key, val) in v.iter().filter_map(|member| match *member.clone() {
                        UEString::KeyValue(key, val) => Some((key, *val)),
                        _ => None,
                    }) {
                        result.insert(key, val.to_value()?);
                    }
                    serde_json::to_value(result)?
                } else {
                    serde_json::to_value(
                        v.iter()
                            .map(|item| item.to_value())
                            .filter(|item| item.is_ok())
                            .map(|item| item.unwrap())
                            .collect::<Vec<Value>>(),
                    )?
                }
            }
            UEString::KeyValue(key, val) => json!({key: val.to_value()?}),
            UEString::None => Value::Null,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Type)]
pub struct UE<T: Clone + Debug + Serialize + DeserializeOwned + PartialEq>(Option<T>);

impl<T: Clone + Debug + Serialize + DeserializeOwned + PartialEq> Serialize for UE<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: Clone + Debug + Serialize + DeserializeOwned + PartialEq> Deserialize<'de> for UE<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized = Value::deserialize(deserializer)?;
        if let Some(packed) = deserialized.as_str() {
            Ok(UE(serde_json::from_value::<Option<T>>(
                UEString::from_string(packed.to_string())
                    .to_value()
                    .or_else(|e| Err(serde::de::Error::custom(format!("{e:?}"))))?,
            )
            .or_else(|e| {
                Err(serde::de::Error::custom(format!("{e:?}")))
            })?))
        } else {
            Ok(UE(serde_json::from_value::<Option<T>>(deserialized)
                .or_else(|e| {
                    Err(serde::de::Error::custom(format!("{e:?}")))
                })?))
        }
    }
}

impl<T: Clone + Debug + Serialize + DeserializeOwned + PartialEq> AsRef<Option<T>> for UE<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}

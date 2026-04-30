use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;
use std::{
    char::decode_utf16,
    fs::File,
    io::{self, Read},
    path::Path,
};
use strip_bom::StripBom;

use crate::DocsError;

#[derive(Serialize, Clone, Debug, PartialEq, Type)]
#[serde(untagged)]
pub enum Coercion {
    Float(f32),
    Integer(i32),
    Boolean(bool),
    String(Option<String>),
}

impl<'de> Deserialize<'de> for Coercion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized = Value::deserialize(deserializer)?;
        if let Some(raw) = deserialized.clone().as_str() {
            Ok(match raw {
                "True" => Coercion::Boolean(true),
                "False" => Coercion::Boolean(false),
                "None" => Coercion::String(None),
                v => {
                    if let Ok(valid_num) = v.parse::<f32>() {
                        if valid_num.ceil() == valid_num {
                            Coercion::Integer(valid_num as i32)
                        } else {
                            Coercion::Float(valid_num)
                        }
                    } else {
                        Coercion::String(Some(v.to_string()))
                    }
                }
            })
        } else {
            match deserialized {
                Value::Bool(v) => Ok(Coercion::Boolean(v)),
                Value::Number(v) => {
                    if v.is_f64() {
                        Ok(Coercion::Float(v.as_f64().unwrap() as f32))
                    } else if v.is_i64() {
                        Ok(Coercion::Integer(v.as_i64().unwrap() as i32))
                    } else {
                        Ok(Coercion::Integer(v.as_u64().unwrap() as i32))
                    }
                }
                Value::String(v) => Ok(Coercion::String(Some(v))),
                Value::Null => Ok(Coercion::String(None)),
                _ => Ok(Coercion::String(None)),
            }
        }
    }
}

impl Into<f32> for Coercion {
    fn into(self) -> f32 {
        if let Coercion::Float(v) = self {
            v
        } else {
            0.0
        }
    }
}

impl Into<i32> for Coercion {
    fn into(self) -> i32 {
        if let Coercion::Integer(v) = self {
            v
        } else {
            0
        }
    }
}

impl Into<bool> for Coercion {
    fn into(self) -> bool {
        if let Coercion::Boolean(v) = self {
            v
        } else {
            false
        }
    }
}

impl Into<Option<String>> for Coercion {
    fn into(self) -> Option<String> {
        if let Coercion::String(v) = self {
            v
        } else {
            None
        }
    }
}

impl Into<String> for Coercion {
    fn into(self) -> String {
        if let Coercion::String(v) = self {
            v.unwrap_or(String::new())
        } else {
            String::new()
        }
    }
}

impl Into<u64> for Coercion {
    fn into(self) -> u64 {
        match self {
            Coercion::Float(v) => v as u64,
            Coercion::Integer(v) => v as u64,
            _ => 0
        }
    }
}

impl Into<f64> for Coercion {
    fn into(self) -> f64 {
        match self {
            Coercion::Float(v) => v as f64,
            Coercion::Integer(v) => v as f64,
            _ => 0.0
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Type)]
pub struct IconPath(Option<String>);

impl<'de> Deserialize<'de> for IconPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        if !raw.contains("/") && raw.ends_with("256") {
            Ok(IconPath(Some(raw)))
        } else {
            if let Some((_, name)) = raw.trim_end_matches("_").rsplit_once(".") {
                if let Some((unsize, _)) = name.trim_end_matches("_").rsplit_once("_") {
                    Ok(IconPath(Some(format!("{unsize}_256"))))
                } else {
                    Ok(IconPath(None))
                }
            } else {
                Ok(IconPath(None))
            }
        }
    }
}

impl Into<String> for IconPath {
    fn into(self) -> String {
        self.0.or(Some(String::new())).unwrap()
    }
}

impl AsRef<str> for IconPath {
    fn as_ref(&self) -> &str {
        match &self.0 {
            Some(v) => v.as_str(),
            None => "",
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Type)]
pub struct NormalizedString(String);

impl<'de> Deserialize<'de> for NormalizedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Ok(NormalizedString(raw.to_case(Case::Pascal)))
    }
}

impl Into<String> for NormalizedString {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for NormalizedString {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Type)]
pub struct ClassReference(String);

impl<'de> Deserialize<'de> for ClassReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        if raw.contains("/") {
            if let Some((_, name)) = raw.rsplit_once(".") {
                Ok(ClassReference(
                    name.to_case(Case::Pascal)
                        .trim_end_matches("'")
                        .trim_end_matches("\"")
                        .to_string(),
                ))
            } else {
                Ok(ClassReference(raw))
            }
        } else {
            Ok(ClassReference(raw))
        }
    }
}

impl Into<String> for ClassReference {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for ClassReference {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

pub fn parse_docs_json(docs_file: impl AsRef<Path>) -> crate::Result<Value> {
    let combined_path = docs_file.as_ref().to_path_buf();
    if !combined_path.exists() {
        return Err(DocsError::failed_read(
            combined_path.clone(),
            io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Docs file <{}> not found!",
                    combined_path.clone().to_string_lossy().to_string()
                ),
            ),
        )
        .into());
    }

    let mut file_bytes: Vec<u8> = Vec::new();
    let file_bytesize = File::open(&combined_path)
        .or_else(|e| {
            Err::<File, crate::CommonError>(
                DocsError::failed_read(combined_path.as_path(), e).into(),
            )
        })?
        .read_to_end(&mut file_bytes)
        .or_else(|e| {
            Err::<usize, crate::CommonError>(
                DocsError::failed_read(combined_path.as_path(), e).into(),
            )
        })?;

    if file_bytesize % 2 != 0 {
        return Err(
            DocsError::invalid_format("File is not UTF-16 encoded/is missing bytes.").into(),
        );
    }
    let iter = (0..(file_bytesize / 2))
        .map(|i| u16::from_le_bytes([file_bytes[2 * i], file_bytes[2 * i + 1]]));

    let decoded = decode_utf16(iter)
        .collect::<Result<String, _>>()
        .or_else(|e| {
            Err::<String, crate::CommonError>(
                DocsError::invalid_format(format!("Bad UTF-16 encoding: {:?}", e)).into(),
            )
        })?;

    serde_json::from_str::<Value>(&decoded.strip_bom())
        .or_else(|e| Err(DocsError::invalid_format(format!("Bad JSON data: {:?}", e)).into()))
}

#[derive(Serialize, Clone, Debug, PartialEq, Type)]
pub struct AssetReference {
    pub asset_type: String,
    pub asset_path: String,
    pub asset_id: Option<String>,
}

impl<'de> Deserialize<'de> for AssetReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;

        let asset_id = if !raw.contains("/") && raw.ends_with("256") {
            Some(raw.clone())
        } else {
            if let Some((_, name)) = raw.trim_end_matches("_").rsplit_once(".") {
                if let Some((unsize, _)) = name.trim_end_matches("_").rsplit_once("_") {
                    Some(format!("{unsize}_256"))
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some((kind, path)) = raw.split_once(' ') {
            Ok(Self {
                asset_type: kind.to_string(),
                asset_path: path.to_string(),
                asset_id,
            })
        } else {
            Ok(Self {
                asset_type: String::from("UNKNOWN"),
                asset_path: raw,
                asset_id,
            })
        }
    }
}

use std::fmt::Display;

use getset::CloneGetters;
use satisfactory_data_macros::model;

#[model(Default, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub enum ResearchIdKind {
    #[default]
    Default,
    Schematic,
}

impl From<ResearchIdKind> for String {
    fn from(value: ResearchIdKind) -> Self {
        (match value {
            ResearchIdKind::Default => "default",
            ResearchIdKind::Schematic => "schematic",
        })
        .to_string()
    }
}

impl TryFrom<String> for ResearchIdKind {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "default" => Ok(Self::Default),
            "schematic" => Ok(Self::Schematic),
            other => Err(crate::Error::invalid_id(other)),
        }
    }
}

impl Display for ResearchIdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self.clone()))
    }
}

#[model(Default, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub enum DescriptionIdKind {
    #[default]
    Default,
    Equipment,
}

impl From<DescriptionIdKind> for String {
    fn from(value: DescriptionIdKind) -> Self {
        (match value {
            DescriptionIdKind::Default => "default",
            DescriptionIdKind::Equipment => "equipment",
        })
        .to_string()
    }
}

impl TryFrom<String> for DescriptionIdKind {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "default" => Ok(Self::Default),
            "equipment" => Ok(Self::Equipment),
            other => Err(crate::Error::invalid_id(other)),
        }
    }
}

impl Display for DescriptionIdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self.clone()))
    }
}

#[model(PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub enum SpecialIdKind {
    Tape,
    Scanner,
    Customizer,
    Emote,
}

impl From<SpecialIdKind> for String {
    fn from(value: SpecialIdKind) -> Self {
        (match value {
            SpecialIdKind::Tape => "tape",
            SpecialIdKind::Scanner => "scanner",
            SpecialIdKind::Customizer => "customizer",
            SpecialIdKind::Emote => "emote",
        })
        .to_string()
    }
}

impl TryFrom<String> for SpecialIdKind {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "tape" => Ok(Self::Tape),
            "scanner" => Ok(Self::Scanner),
            "customizer" => Ok(Self::Customizer),
            "emote" => Ok(Self::Emote),
            other => Err(crate::Error::invalid_id(other)),
        }
    }
}

impl Display for SpecialIdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self.clone()))
    }
}

#[model(PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub enum IdKind {
    Research(ResearchIdKind),
    Description(DescriptionIdKind),
    Build,
    Recipe,
    Special(SpecialIdKind),
}

impl From<IdKind> for String {
    fn from(value: IdKind) -> Self {
        match value {
            IdKind::Research(research_id_kind) => {
                format!("research:{}", String::from(research_id_kind))
            }
            IdKind::Description(description_id_kind) => {
                format!("desc:{}", String::from(description_id_kind))
            }
            IdKind::Build => "build".to_string(),
            IdKind::Recipe => "recipe".to_string(),
            IdKind::Special(special_id_kind) => {
                format!("special:{}", String::from(special_id_kind))
            }
        }
    }
}

impl TryFrom<String> for IdKind {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "build" => Ok(Self::Build),
            "recipe" => Ok(Self::Recipe),
            complex_id if complex_id.contains(":") => {
                let (head, tail) = complex_id.split_once(":").unwrap();
                match head {
                    "research" => Ok(Self::Research(ResearchIdKind::try_from(tail.to_string())?)),
                    "desc" => Ok(Self::Description(DescriptionIdKind::try_from(
                        tail.to_string(),
                    )?)),
                    "special" => Ok(Self::Special(SpecialIdKind::try_from(tail.to_string())?)),
                    unknown => Err(crate::Error::invalid_id(unknown)),
                }
            }
            unknown => Err(crate::Error::invalid_id(unknown)),
        }
    }
}

impl Display for IdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self.clone()))
    }
}

#[model(CloneGetters, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
#[cfg_attr(feature = "specta-1", specta(type = ::std::string::String))]
#[cfg_attr(feature = "specta-2", specta(type = ::std::string::String))]
#[getset(get_clone = "pub")]
pub struct Id {
    kind: IdKind,
    name: String,
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        format!("{}/{}", value.kind(), value.name())
    }
}

impl TryFrom<String> for Id {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some((head, tail)) = value.split_once("/") {
            match IdKind::try_from(head.to_string()) {
                Ok(kind) => Ok(Self {
                    kind,
                    name: tail.to_string(),
                }),
                Err(crate::Error::InvalidId(segment)) => Err(crate::Error::invalid_id(format!(
                    "{value} (Invalid segment: {segment})"
                ))),
                Err(other) => Err(other),
            }
        } else {
            Err(crate::Error::invalid_id(format!(
                "{value} (Missing / delimiter)"
            )))
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self.clone()))
    }
}

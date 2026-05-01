use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::types::*;
use chrono::Utc;
use log::{debug, info, warn};
use parking_lot::RwLock;
use satisfactory_data_macros::model;
use tempfile::{TempDir, tempfile};

#[model(Default)]
pub struct RegistryMeta {
    pub generated: chrono::DateTime<Utc>,
    pub game_version: String,
    pub experimental: bool,
}

#[model(Default)]
pub struct RawRegistry {
    pub metadata: RegistryMeta,

    #[serde(default)]
    pub recipes: HashMap<String, RecipeItem>,

    #[serde(default)]
    pub research: HashMap<String, ResearchItem>,

    #[serde(default)]
    pub descriptions: HashMap<String, DescriptionItem>,

    #[serde(default)]
    pub buildables: HashMap<String, BuildableItem>,
}

#[derive(Debug)]
enum RegistryHandle {
    Archive { source: PathBuf, path: TempDir },
    Remote { url: String, path: TempDir },
    Persisted { path: PathBuf },
}

impl RegistryHandle {
    pub(self) fn reference(&self) -> RegistryReference {
        match self {
            RegistryHandle::Archive { source, .. } => RegistryReference::Archive {
                source: source.clone(),
            },
            RegistryHandle::Remote { url, .. } => RegistryReference::Remote { url: url.clone() },
            RegistryHandle::Persisted { path } => {
                RegistryReference::Persisted { path: path.clone() }
            }
        }
    }

    pub(self) fn path(&self) -> PathBuf {
        match self {
            RegistryHandle::Archive { path, .. } => path.path().to_path_buf(),
            RegistryHandle::Remote { path, .. } => path.path().to_path_buf(),
            RegistryHandle::Persisted { path } => path.clone(),
        }
    }
}

#[model]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RegistryReference {
    Archive { source: PathBuf },
    Remote { url: String },
    Persisted { path: PathBuf },
}

#[model]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum RegistryItem {
    Special { id: Id },
    Buildable { buildable: BuildableItem },
    Description {description: DescriptionItem},
    Recipe {recipe: RecipeItem},
    Research {research: ResearchItem}
}

impl RegistryItem {
    pub fn id(&self) -> Id {
        match self {
            RegistryItem::Special { id } => id.clone(),
            RegistryItem::Buildable { buildable } => buildable.id.clone(),
            RegistryItem::Description { description } => description.id.clone(),
            RegistryItem::Recipe { recipe } => recipe.id.clone(),
            RegistryItem::Research { research } => research.id.clone(),
        }
    }

    pub fn is_special(&self) -> bool {
        if let Self::Special { .. } = self.clone() {true} else {false}
    }

    pub fn is_buildable(&self) -> bool {
        if let Self::Buildable { .. } = self.clone() {true} else {false}
    }

    pub fn is_description(&self) -> bool {
        if let Self::Description { .. } = self.clone() {true} else {false}
    }

    pub fn is_recipe(&self) -> bool {
        if let Self::Recipe { .. } = self.clone() {true} else {false}
    }
    
    pub fn is_research(&self) -> bool {
        if let Self::Research { .. } = self.clone() {true} else {false}
    }

    pub fn as_buildable(&self) -> Option<&BuildableItem> {
        if let Self::Buildable { buildable } = self {Some(buildable)} else {None}
    }

    pub fn as_description(&self) -> Option<&DescriptionItem> {
        if let Self::Description { description } = self {Some(description)} else {None}
    }

    pub fn as_recipe(&self) -> Option<&RecipeItem> {
        if let Self::Recipe { recipe } = self {Some(recipe)} else {None}
    }

    pub fn as_research(&self) -> Option<&ResearchItem> {
        if let Self::Research { research } = self {Some(research)} else {None}
    }
}

impl From<BuildableItem> for RegistryItem {
    fn from(value: BuildableItem) -> Self {
        Self::Buildable { buildable: value }
    }
}

impl From<DescriptionItem> for RegistryItem {
    fn from(value: DescriptionItem) -> Self {
        Self::Description { description: value }
    }
}

impl From<RecipeItem> for RegistryItem {
    fn from(value: RecipeItem) -> Self {
        Self::Recipe { recipe: value }
    }
}

impl From<ResearchItem> for RegistryItem {
    fn from(value: ResearchItem) -> Self {
        Self::Research { research: value }
    }
}

impl TryFrom<RegistryItem> for BuildableItem {
    type Error = crate::Error;
    fn try_from(value: RegistryItem) -> Result<Self, Self::Error> {
        if let Some(item) = value.as_buildable() {
            Ok(item.clone())
        } else {
            Err(crate::Error::registry_extract_mismatch(value.id()))
        }
    }
}

impl TryFrom<RegistryItem> for DescriptionItem {
    type Error = crate::Error;
    fn try_from(value: RegistryItem) -> Result<Self, Self::Error> {
        if let Some(item) = value.as_description() {
            Ok(item.clone())
        } else {
            Err(crate::Error::registry_extract_mismatch(value.id()))
        }
    }
}

impl TryFrom<RegistryItem> for RecipeItem {
    type Error = crate::Error;
    fn try_from(value: RegistryItem) -> Result<Self, Self::Error> {
        if let Some(item) = value.as_recipe() {
            Ok(item.clone())
        } else {
            Err(crate::Error::registry_extract_mismatch(value.id()))
        }
    }
}

impl TryFrom<RegistryItem> for ResearchItem {
    type Error = crate::Error;
    fn try_from(value: RegistryItem) -> Result<Self, Self::Error> {
        if let Some(item) = value.as_research() {
            Ok(item.clone())
        } else {
            Err(crate::Error::registry_extract_mismatch(value.id()))
        }
    }
}

/// The main interface to this crate, holding a map of generated data and a handle to a tempdir containing extracted assets
///
#[derive(Clone, Debug)]
pub struct Registry {
    raw: Arc<RwLock<RawRegistry>>,
    asset_pack: Arc<RegistryHandle>,
}

/// Getters
impl Registry {
    pub fn get_raw(&self) -> RawRegistry {
        self.raw.read().clone()
    }

    pub fn get_metadata(&self) -> RegistryMeta {
        self.raw.read().metadata.clone()
    }

    pub fn get_registry_reference(&self) -> RegistryReference {
        self.asset_pack.reference()
    }

    pub fn get_registry_path(&self) -> PathBuf {
        self.asset_pack.path()
    }

    pub fn get_item(&self, id: impl AsRef<str>) -> crate::Result<Option<RegistryItem>> {
        let id = Id::try_from(id.as_ref().to_string())?;
        let raw = self.raw.read();
        match id.kind() {
            IdKind::Research(_) => Ok(raw.research.get(&id.to_string()).and_then(|v| Some(v.clone().into()))),
            IdKind::Description(_) => Ok(raw.descriptions.get(&id.to_string()).and_then(|v| Some(v.clone().into()))),
            IdKind::Build => Ok(raw.buildables.get(&id.to_string()).and_then(|v| Some(v.clone().into()))),
            IdKind::Recipe => Ok(raw.recipes.get(&id.to_string()).and_then(|v| Some(v.clone().into()))),
            IdKind::Special(_) => Ok(Some(RegistryItem::Special { id })),
        }
    }

    pub fn get_buildables(&self) -> HashMap<Id, BuildableItem> {
        let raw = self.raw.read();
        raw.buildables.values().map(|v| (v.id.clone(), v.clone())).collect()
    }

    pub fn get_descriptions(&self) -> HashMap<Id, DescriptionItem> {
        let raw = self.raw.read();
        raw.descriptions.values().map(|v| (v.id.clone(), v.clone())).collect()
    }

    pub fn get_recipes(&self) -> HashMap<Id, RecipeItem> {
        let raw = self.raw.read();
        raw.recipes.values().map(|v| (v.id.clone(), v.clone())).collect()
    }

    pub fn get_research(&self) -> HashMap<Id, ResearchItem> {
        let raw = self.raw.read();
        raw.research.values().map(|v| (v.id.clone(), v.clone())).collect()
    }
}

/// Setters/mutation
impl Registry {
    fn raise_mutable(&self) -> crate::Result<()> {
        if self.is_mutable() {
            Ok(())
        } else {
            Err(crate::Error::ImmutableRegistry)
        }
    }

    pub fn is_mutable(&self) -> bool {
        if let RegistryReference::Persisted { .. } = self.get_registry_reference() {
            true
        } else {
            false
        }
    }

    pub fn insert(&self, item: impl Into<RegistryItem>) -> crate::Result<()> {
        self.raise_mutable()?;
        let item = item.into();
        let mut raw = self.raw.write();
        match item {
            RegistryItem::Special { id } => {
                warn!("Tried to insert special item with id {id}, which is a no-op");
            },
            RegistryItem::Buildable { buildable } => {
                let _ = raw.buildables.insert(buildable.id.to_string(), buildable);
            },
            RegistryItem::Description { description } => {
                let _ = raw.descriptions.insert(description.id.to_string(), description);
            },
            RegistryItem::Recipe { recipe } => {
                let _ = raw.recipes.insert(recipe.id.to_string(), recipe);
            },
            RegistryItem::Research { research } => {
                let _ = raw.research.insert(research.id.to_string(), research);
            },
        }

        Ok(())
    }

    pub fn remove(&self, id: impl Into<String>) -> crate::Result<()> {
        self.raise_mutable()?;
        let id = Id::try_from(Into::<String>::into(id))?;
        let mut raw = self.raw.write();
        match id.kind() {
            IdKind::Research(..) => {
                let _ = raw.research.remove(&id.to_string());
            },
            IdKind::Description(..) => {
                let _ = raw.descriptions.remove(&id.to_string());
            },
            IdKind::Build => {
                let _ = raw.buildables.remove(&id.to_string());
            },
            IdKind::Recipe => {
                let _ = raw.recipes.remove(&id.to_string());
            },
            IdKind::Special(_) => {
                warn!("Tried to remove special item with id {id}, which is a no-op");
            },
        }
        Ok(())
    }

    pub fn save(&self) -> crate::Result<()> {
        self.raise_mutable()?;
        let target = self.asset_pack.path().join("registry.json");
        std::fs::write(target, serde_json::to_string_pretty(&self.get_raw())?)?;
        Ok(())
    }
}

/// Openers
impl Registry {
    fn _load_registry(directory: PathBuf) -> crate::Result<RawRegistry> {
        let reg_data = std::fs::read_to_string(directory.join("registry.json"))?;
        Ok(serde_json::from_str::<RawRegistry>(&reg_data)?)
    }

    fn _open_file_internal(path: PathBuf) -> crate::Result<Self> {
        info!(
            "Attempting to open registry from local asset pack: {}",
            path.to_string_lossy()
        );
        let file = std::fs::File::open(path.clone())?;
        let mut zipfile = zip::ZipArchive::new(file)?;
        let target = TempDir::new()?;
        zipfile.extract(target.path())?;
        Ok(Self {
            raw: Arc::new(RwLock::new(Self::_load_registry(
                target.path().to_path_buf(),
            )?)),
            asset_pack: Arc::new(RegistryHandle::Archive {
                source: path.clone(),
                path: target,
            }),
        })
    }

    fn _open_url_internal(url: String) -> crate::Result<Self> {
        info!(
            "Attempting to open registry from remote asset pack: {}",
            url.clone()
        );
        let response = reqwest::blocking::get(url.clone())?.error_for_status()?;
        let mut staging_zip = tempfile()?;
        staging_zip.write_all(&response.bytes()?.to_vec())?;
        let mut zipfile = zip::ZipArchive::new(staging_zip)?;
        let target = TempDir::new()?;
        zipfile.extract(target.path())?;
        Ok(Self {
            raw: Arc::new(RwLock::new(Self::_load_registry(
                target.path().to_path_buf(),
            )?)),
            asset_pack: Arc::new(RegistryHandle::Remote {
                url: url.clone(),
                path: target,
            }),
        })
    }

    fn _open_persisted_internal(path: PathBuf) -> crate::Result<Self> {
        info!(
            "Attempting to open registry from persisted asset pack: {}",
            path.to_string_lossy()
        );
        Ok(Self {
            raw: Arc::new(RwLock::new(Self::_load_registry(path.clone())?)),
            asset_pack: Arc::new(RegistryHandle::Persisted { path }),
        })
    }

    /// Load a Registry from a local zip file (generated with aextract)
    ///
    /// # Arguments
    ///
    /// - `path` (`impl AsRef<Path>`) - Path to load from
    ///
    /// # Returns
    ///
    /// - `crate::Result<Self>` - New Registry object, if successful
    ///
    #[cfg(not(feature = "async"))]
    pub fn from_file(path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref().to_path_buf();
        match Self::_open_file_internal(path.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(crate::Error::registry_load(
                format!("file: {}", path.to_string_lossy()),
                e,
            )),
        }
    }

    /// Load a Registry from a URL to a zip file (probably from this crate's github repository)
    ///
    /// # Arguments
    ///
    /// - `url` (`impl Into<String>`) - URL to load from. Should return a zip file
    ///
    /// # Returns
    ///
    /// - `crate::Result<Self>` - New Registry object, if successful
    ///
    #[cfg(not(feature = "async"))]
    pub fn from_url(url: impl Into<String>) -> crate::Result<Self> {
        let url = url.into();
        match Self::_open_url_internal(url.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(crate::Error::registry_load(format!("url: {}", url), e)),
        }
    }

    /// Load a Registry from a local zip file (generated with aextract)
    ///
    /// # Arguments
    ///
    /// - `path` (`impl AsRef<Path>`) - Path to load from
    ///
    /// # Returns
    ///
    /// - `crate::Result<Self>` - New Registry object, if successful
    ///
    #[cfg(feature = "async")]
    pub async fn from_file(path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let cloned_path = path.clone();
        match tokio::task::spawn_blocking(move || Self::_open_file_internal(cloned_path))
            .await
            .unwrap()
        {
            Ok(r) => Ok(r),
            Err(e) => Err(crate::Error::registry_load(
                format!("file: {}", path.to_string_lossy()),
                e,
            )),
        }
    }

    /// Load a Registry from a URL to a zip file (probably from this crate's github repository)
    ///
    /// # Arguments
    ///
    /// - `url` (`impl Into<String>`) - URL to load from. Should return a zip file
    ///
    /// # Returns
    ///
    /// - `crate::Result<Self>` - New Registry object, if successful
    ///
    #[cfg(feature = "async")]
    pub async fn from_url(url: impl Into<String>) -> crate::Result<Self> {
        let url = url.into();
        let cloned_url = url.clone();
        match tokio::task::spawn_blocking(move || Self::_open_url_internal(cloned_url))
            .await
            .unwrap()
        {
            Ok(r) => Ok(r),
            Err(e) => Err(crate::Error::registry_load(format!("url: {}", url), e)),
        }
    }

    /// Loads Registry from a persisted path (ie unpacked in local storage)
    ///
    /// # Arguments
    ///
    /// - `path` (`impl AsRef<Path>`) - Path to the root directory of this asset pack - should contain a `registry.json` file
    ///
    /// # Returns
    ///
    /// - `crate::Result<Self>` - New Registry object, if successful
    ///
    pub fn from_persisted(path: impl AsRef<Path>) -> crate::Result<Self> {
        let path = path.as_ref().to_path_buf();
        match Self::_open_persisted_internal(path.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(crate::Error::registry_load(
                format!("persisted: {}", path.to_string_lossy()),
                e,
            )),
        }
    }

    /// Persist this registry and its assets to a mutable persistent directory. This allows future modifications to the registry.
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`) - Describe this parameter.
    /// - `target_directory` (`impl AsRef<Path>`) - Directory path to persist to. Will be created if it doesn't exist
    ///
    /// # Returns
    ///
    /// - `crate::Result<Self>` - A new Registry containing the same data as the current one, but with an updated Reference
    ///
    /// # Errors
    ///
    /// In addition to standard IO errors, this method will also fail if the target directory exists and is not empty.
    ///
    pub fn persist(&self, target_directory: impl AsRef<Path>) -> crate::Result<Self> {
        let target_directory = target_directory.as_ref().to_path_buf();
        let source = self.asset_pack.path();
        info!(
            "Persisting current registry ({:?}) to {}",
            self.asset_pack.clone(),
            target_directory.to_string_lossy()
        );
        if target_directory.exists() {
            if target_directory.read_dir()?.count() > 0 {
                return Err(crate::Error::registry_persist(
                    "Target directory exists and is not empty",
                ));
            }
        }

        std::fs::create_dir_all(target_directory.join("icons"))?;
        std::fs::create_dir_all(target_directory.join("map"))?;
        debug!(
            "Copying {:?} to {:?}",
            source.join("registry.json"),
            target_directory.join("registry.json")
        );
        std::fs::copy(
            source.join("registry.json"),
            target_directory.join("registry.json"),
        )?;
        for entry in glob::glob(source.join("**/*.png").to_str().unwrap())
            .unwrap()
            .filter_map(Result::ok)
        {
            if entry.starts_with(source.join("icons")) {
                debug!(
                    "Copying {:?} to {:?}",
                    entry,
                    target_directory
                        .join("icons")
                        .join(entry.file_name().unwrap())
                );
                std::fs::copy(
                    entry.clone(),
                    target_directory
                        .join("icons")
                        .join(entry.file_name().unwrap()),
                )?;
            } else if entry.starts_with(source.join("map")) {
                debug!(
                    "Copying {:?} to {:?}",
                    entry,
                    target_directory
                        .join("map")
                        .join(entry.file_name().unwrap())
                );
                std::fs::copy(
                    entry.clone(),
                    target_directory
                        .join("map")
                        .join(entry.file_name().unwrap()),
                )?;
            }
        }

        Ok(Self {
            raw: Arc::new(RwLock::new(self.get_raw())),
            asset_pack: Arc::new(RegistryHandle::Persisted {
                path: target_directory.clone(),
            }),
        })
    }
}

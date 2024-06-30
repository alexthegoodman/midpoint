// NOTE: this context is saved in real-time to the database for each project

use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct File {
    pub id: String,
    pub fileName: String,
    pub cloudfrontUrl: String,
    pub normalFilePath: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct LandscapeData {
    pub id: String,
    pub heightmap: Option<File>,
    pub rockmap: Option<File>,
    pub soil: Option<File>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ComponentKind {
    Model,
    Landscape,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum LandscapeTextureKinds {
    Primary,
    PrimaryMask,
    Rockmap,
    RockmapMask,
    Soil,
    SoilMask,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct GenericProperties {
    pub name: String,
    // position / transform
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct LandscapeProperties {
    pub primary_texture_id: Option<String>,
    pub rockmap_texture_id: Option<String>,
    pub soil_texture_id: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct ModelProperties {
    // pub id: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct ComponentData {
    pub id: String,
    pub kind: Option<ComponentKind>,
    pub asset_id: String, // File.id or LandscapeData.id
    pub generic_properties: GenericProperties,
    pub landscape_properties: Option<LandscapeProperties>,
    pub model_properties: Option<ModelProperties>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct LevelData {
    pub id: String,
    pub components: Option<Vec<ComponentData>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SavedState {
    pub concepts: Vec<File>,
    pub models: Vec<File>,
    pub landscapes: Option<Vec<LandscapeData>>,
    pub textures: Option<Vec<File>>,
    pub levels: Option<Vec<LevelData>>,
}

pub enum SavedAction {
    RefreshContext(SavedState),
    AddLevel(LevelData),
    AddComponent(ComponentData),
    SetLandscapeTexture(String, LandscapeTextureKinds, String),
}

impl Default for SavedState {
    fn default() -> Self {
        Self {
            concepts: Vec::new(),
            models: Vec::new(),
            landscapes: None,
            textures: Some(Vec::new()),
            levels: Some(Vec::new()),
        }
    }
}

impl Reducible for SavedState {
    type Action = SavedAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            SavedAction::RefreshContext(saved_state) => saved_state,
            SavedAction::AddLevel(level_data) => {
                let mut new_levels = self.levels.clone().unwrap_or_default();
                new_levels.push(level_data);
                SavedState {
                    levels: Some(new_levels),
                    ..(*self).clone()
                }
            }
            SavedAction::AddComponent(component_data) => {
                let mut new_levels = self.levels.clone().unwrap_or_default();
                if let Some(level) = new_levels.last_mut() {
                    let mut new_components = level.components.clone().unwrap_or_default();
                    new_components.push(component_data);
                    level.components = Some(new_components);
                }
                SavedState {
                    levels: Some(new_levels),
                    ..(*self).clone()
                }
            }
            SavedAction::SetLandscapeTexture(component_id, texture_kind, value) => {
                let mut new_levels = self.levels.clone().unwrap_or_default();
                if let Some(level) = new_levels.last_mut() {
                    if let Some(components) = &mut level.components {
                        if let Some(component) =
                            components.iter_mut().find(|c| c.id == component_id)
                        {
                            if let Some(landscape_properties) = &mut component.landscape_properties
                            {
                                match texture_kind {
                                    LandscapeTextureKinds::Primary => {
                                        landscape_properties.primary_texture_id = Some(value)
                                    }
                                    LandscapeTextureKinds::Rockmap => {
                                        landscape_properties.rockmap_texture_id = Some(value)
                                    }
                                    LandscapeTextureKinds::Soil => {
                                        landscape_properties.soil_texture_id = Some(value)
                                    }
                                    _ => {
                                        web_sys::console::error_1(
                                            &format!("Invalid texture kind: {}", value).into(),
                                        );
                                        // return;
                                    }
                                }
                            }
                        }
                    }
                }
                SavedState {
                    levels: Some(new_levels),
                    ..(*self).clone()
                }
            }
        };

        Rc::new(next_state)
    }
}

pub type SavedContextType = UseReducerHandle<SavedState>;

// NOTE: this context is saved in real-time to the database for each project

use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub fileName: String,
    pub cloudfrontUrl: String,
    pub normalFilePath: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedState {
    pub concepts: Vec<File>,
    pub models: Vec<File>,
    pub landscapes: Option<Vec<File>>,
    pub textures: Option<Vec<File>>,
    pub levels: Option<Vec<File>>,
}

pub enum SavedAction {
    RefreshContext(SavedState),
    // AddConcept(String),
    // AddModel(String),
}

impl Default for SavedState {
    fn default() -> Self {
        Self {
            concepts: Vec::new(),
            models: Vec::new(),
            landscapes: Some(Vec::new()),
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
        };

        Rc::new(next_state)
    }
}

pub type SavedContextType = UseReducerHandle<SavedState>;

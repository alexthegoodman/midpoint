// NOTE: this context is not saved to the database and refreshed upon engine startup

use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

use crate::gql::getMdProjects::get_md_projects;

use super::saved::SavedState;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MdProject {
    pub id: String,
    pub title: String,
    // pub context: SavedState,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalState {
    pub route: String,
    pub token: Option<String>,
    pub current_project_id: Option<String>,
    pub projects: Option<Vec<MdProject>>,
}

pub enum LocalAction {
    SetRoute(String),
    SetToken(String),
    ClearToken,
    SetCurrentProject(String),
    ClearCurrentProject,
    SetProjects(Vec<MdProject>),
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            route: "/".to_string(),
            token: None,
            current_project_id: None,
            projects: None,
        }
    }
}

impl Reducible for LocalState {
    type Action = LocalAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state = match action {
            LocalAction::SetRoute(path) => LocalState {
                route: path,
                ..(*self).clone() // Preserve other fields
            },
            LocalAction::SetToken(token) => LocalState {
                token: Some(token),
                ..(*self).clone() // Preserve other fields
            },
            LocalAction::ClearToken => LocalState {
                token: None,
                ..(*self).clone() // Preserve other fields
            },
            LocalAction::SetCurrentProject(project_id) => LocalState {
                current_project_id: Some(project_id),
                ..(*self).clone() // Preserve other fields
            },
            LocalAction::ClearCurrentProject => LocalState {
                current_project_id: None,
                ..(*self).clone() // Preserve other fields
            },
            LocalAction::SetProjects(projects) => LocalState {
                projects: Some(projects),
                ..(*self).clone() // Preserve other fields
            },
        };

        Rc::new(next_state)
    }
}

pub type LocalContextType = UseReducerHandle<LocalState>;

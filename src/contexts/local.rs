// NOTE: this context is not saved to the database and refreshed upon engine startup

use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalState {
    pub route: String,
    pub token: Option<String>,
}

pub enum LocalAction {
    SetRoute(String),
    SetToken(String),
    ClearToken,
}

impl Default for LocalState {
    fn default() -> Self {
        Self {
            route: "/".to_string(),
            token: None,
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
        };

        Rc::new(next_state)
    }
}

pub type LocalContextType = UseReducerHandle<LocalState>;

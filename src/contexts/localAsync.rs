use std::rc::Rc;
use yew::prelude::*;

use crate::gql::getMdProjects::get_md_projects;

use super::local::{LocalAction, LocalState};

#[derive(Clone, PartialEq)]

pub struct LocalAsync {
    state: UseReducerHandle<LocalState>,
}

impl LocalAsync {
    pub fn new(state: UseReducerHandle<LocalState>) -> Self {
        Self { state }
    }

    pub fn refresh_projects(&self) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(token) = &state.token {
                web_sys::console::log_1(&"Refreshing projects...".into());

                if let Ok(md_projects) = get_md_projects(token.clone()).await {
                    let latest_projects = md_projects.getMdProjects;
                    state.dispatch(LocalAction::SetProjects(latest_projects));
                }
            }
        });
    }
}

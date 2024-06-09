use std::rc::Rc;
use yew::functional::*;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct GlobalState {
    pub route: String,
}

pub enum GlobalAction {
    SetRoute(String),
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            route: "/".to_string(),
        }
    }
}

impl Reducible for GlobalState {
    type Action = GlobalAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            GlobalAction::SetRoute(path) => GlobalState { route: path },
        };

        Self { ..next_ctr }.into()
    }
}

pub type GlobalContextType = UseReducerHandle<GlobalState>;

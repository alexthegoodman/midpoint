use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MdButtonProps {
    pub icon: String,
    pub width: String,
    pub height: String,
}

#[function_component]
pub fn MdIcon(props: &MdButtonProps) -> Html {
    html! {
        <img
            src={format!("public/lib/phosphor-css/assets/thin/{}-thin.svg", &props.icon)}
            alt={props.icon.clone()}
            width={props.width.clone()}
            height={props.height.clone()}
        />
    }
}

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::MdButton::{MdButton, MdButtonKind, MdButtonVariant};

#[function_component]
pub fn ViewNav() -> Html {
    html! {
        <nav
            class="view-nav"
        >
            <MdButton
                label=""
                icon={"map-trifold"}
                on_click={Callback::noop()}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"book-open"}
                on_click={Callback::noop()}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"cube-focus"}
                on_click={Callback::noop()}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
        </nav>
    }
}

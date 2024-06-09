use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
    contexts::global::{GlobalAction, GlobalContextType},
};

#[function_component]
pub fn ViewNav() -> Html {
    let global_context = use_context::<GlobalContextType>().expect("No GlobalContext found");

    html! {
        <nav
            class="view-nav"
        >
            <MdButton
                label=""
                icon={"panorama"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        web_sys::console::log_1(&"Enter concepts".into());
                        global_context.dispatch(GlobalAction::SetRoute("/concepts".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"map-trifold"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        global_context.dispatch(GlobalAction::SetRoute("/map".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"book-open"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        global_context.dispatch(GlobalAction::SetRoute("/story".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"cube-focus"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        global_context.dispatch(GlobalAction::SetRoute("/scene".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"faders"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        global_context.dispatch(GlobalAction::SetRoute("/audio".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"speedometer"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        global_context.dispatch(GlobalAction::SetRoute("/performance".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"gear"}
                on_click={Callback::from({
                    let global_context = global_context.clone();
                    move |_| {
                        global_context.dispatch(GlobalAction::SetRoute("/settings".to_string()));
                    }
                })}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
        </nav>
    }
}

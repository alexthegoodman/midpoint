use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
    contexts::local::{LocalAction, LocalContextType},
};

#[function_component]
pub fn ViewNav() -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");

    if local_context.current_project_id.is_none() {
        return html! {
            <nav
                class="view-nav"
            >
                <MdButton
                    label=""
                    icon={"sparkle"}
                    on_click={Callback::from({
                        let local_context = local_context.clone();
                        move |_| {
                            local_context.dispatch(LocalAction::SetRoute("/".to_string()));
                        }
                    })}
                    disabled={false}
                    loading={false}
                    kind={MdButtonKind::MediumShadow}
                    variant={MdButtonVariant::Light}
                />
            </nav>
        };
    }

    html! {
        <nav
            class="view-nav"
        >
            <MdButton
                label=""
                icon={"panorama"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        web_sys::console::log_1(&"Enter concepts".into());
                        local_context.dispatch(LocalAction::SetRoute("/concepts".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"map-trifold"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        local_context.dispatch(LocalAction::SetRoute("/map".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"book-open"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        local_context.dispatch(LocalAction::SetRoute("/story".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"cube-focus"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        local_context.dispatch(LocalAction::SetRoute("/scene".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"faders"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        local_context.dispatch(LocalAction::SetRoute("/audio".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"speedometer"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        local_context.dispatch(LocalAction::SetRoute("/performance".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label=""
                icon={"gear"}
                on_click={Callback::from({
                    let local_context = local_context.clone();
                    move |_| {
                        local_context.dispatch(LocalAction::SetRoute("/settings".to_string()));
                    }
                })}
                disabled={false}
                loading={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
        </nav>
    }
}

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::MdIcon::MdIcon;

#[derive(Clone, PartialEq)]
pub enum MdButtonVariant {
    Green,
    Dark,
    Light,
}

#[derive(Clone, PartialEq)]
pub enum MdButtonKind {
    ThinIcon,
    ThinShort,
    ThinWide,
    SmallIcon,
    SmallShort,
    SmallWide,
    LargeIcon,
    LargeShort,
    LargeWide,
    MediumShadow,
}

#[derive(Clone, PartialEq, Properties)]
pub struct MdButtonProps {
    pub label: String,
    pub icon: String,
    pub on_click: Callback<()>,
    pub disabled: bool,
    pub kind: MdButtonKind,
    pub variant: MdButtonVariant,
}

#[function_component]
pub fn MdButton(props: &MdButtonProps) -> Html {
    let variant = match props.variant {
        MdButtonVariant::Green => "green",
        MdButtonVariant::Dark => "dark",
        MdButtonVariant::Light => "light",
    };

    let kind = match props.kind {
        MdButtonKind::ThinIcon => "thin-icon",
        MdButtonKind::ThinShort => "thin-short",
        MdButtonKind::ThinWide => "thin-wide",
        MdButtonKind::SmallIcon => "small-icon",
        MdButtonKind::SmallShort => "small-short",
        MdButtonKind::SmallWide => "small-wide",
        MdButtonKind::LargeIcon => "large-icon",
        MdButtonKind::LargeShort => "large-short",
        MdButtonKind::LargeWide => "large-wide",
        MdButtonKind::MediumShadow => "medium-shadow",
    };

    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |_: MouseEvent| {
            on_click.emit(());
        })
    };

    html! {
        <button
            class={format!("btn {} {}", variant, kind)}
            onclick={onclick}
            disabled={props.disabled}
        >
            if !props.icon.is_empty() {
                <MdIcon
                    icon={props.icon.clone()}
                    width={"40px".to_string()}
                    height={"40px".to_string()}
                />
            }
            { &props.label }
        </button>
    }
}

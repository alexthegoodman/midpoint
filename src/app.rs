use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

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
    pub icon: Option<String>,
    pub on_click: Callback<()>,
    pub disabled: bool,
    pub kind: MdButtonKind,
    pub variant: MdButtonVariant,
}

#[function_component]
fn MdButton(props: &MdButtonProps) -> Html {
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

    html! {
        <button
            class={format!("btn {} {}", variant, kind)}
            // onclick={props.on_click.clone()}
            disabled={props.disabled}
        >
            { &props.label }
        </button>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <p>{"Midpoint Game Engine!"}</p>
            <MdButton
                label="X"
                icon={""}
                on_click={Callback::noop()}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label="Y"
                icon={""}
                on_click={Callback::noop()}
                disabled={false}
                kind={MdButtonKind::MediumShadow}
                variant={MdButtonVariant::Light}
            />
            <MdButton
                label="Get Started"
                icon={""}
                on_click={Callback::noop()}
                disabled={false}
                kind={MdButtonKind::SmallShort}
                variant={MdButtonVariant::Green}
            />
        </main>
    }
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

// #[function_component(App)]
// pub fn app() -> Html {
//     let greet_input_ref = use_node_ref();

//     let name = use_state(|| String::new());

//     let greet_msg = use_state(|| String::new());
//     {
//         let greet_msg = greet_msg.clone();
//         let name = name.clone();
//         let name2 = name.clone();
//         use_effect_with(
//             name2,
//             move |_| {
//                 spawn_local(async move {
//                     if name.is_empty() {
//                         return;
//                     }

//                     let args = to_value(&GreetArgs { name: &*name }).unwrap();
//                     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//                     let new_msg = invoke("greet", args).await.as_string().unwrap();
//                     greet_msg.set(new_msg);
//                 });

//                 || {}
//             },
//         );
//     }

//     let greet = {
//         let name = name.clone();
//         let greet_input_ref = greet_input_ref.clone();
//         Callback::from(move |e: SubmitEvent| {
//             e.prevent_default();
//             name.set(
//                 greet_input_ref
//                     .cast::<web_sys::HtmlInputElement>()
//                     .unwrap()
//                     .value(),
//             );
//         })
//     };

//     html! {
//         <main class="container">
//             <div class="row">
//                 <a href="https://tauri.app" target="_blank">
//                     <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
//                 </a>
//                 <a href="https://yew.rs" target="_blank">
//                     <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
//                 </a>
//             </div>

//             <p>{"Click on the Tauri and Yew logos to learn more."}</p>

//             <form class="row" onsubmit={greet}>
//                 <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
//                 <button type="submit">{"Greet"}</button>
//             </form>

//             <p><b>{ &*greet_msg }</b></p>
//         </main>
//     }
// }

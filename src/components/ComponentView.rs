use lazy_static::lazy_static;
use std::sync::Arc;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::contexts::{
    local::LocalContextType,
    saved::{
        ComponentData, ComponentKind, File, GenericProperties, LandscapeData,
        LandscapeTextureKinds, SavedAction, SavedContextType,
    },
};

const EMPTY_COMPONENT_DATA: ComponentData = ComponentData {
    id: String::new(),
    kind: None,
    asset_id: String::new(),
    generic_properties: GenericProperties {
        name: String::new(),
    },
    landscape_properties: None,
    model_properties: None,
};

lazy_static! {
    static ref NONE_STRING: Arc<String> = Arc::new(String::new());
    static ref NONE_VEC: Arc<Vec<ComponentData>> = Arc::new(Vec::new());
}

#[function_component(ComponentView)]
pub fn component_view() -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");
    let saved_context = use_context::<SavedContextType>().expect("No SavedContext found");

    let selected_component_id = local_context
        .selected_component_id
        .clone()
        .unwrap_or_else(|| NONE_STRING.to_string().clone());

    let selected_component_id = selected_component_id.clone();

    let selected_component_data = saved_context.levels.as_ref().and_then(|levels| {
        levels
            .iter()
            .flat_map(|level| level.components.as_ref().unwrap_or(&NONE_VEC))
            .find(|component| component.id == selected_component_id)
            .cloned()
    });

    let selected_component_data = selected_component_data.unwrap_or(EMPTY_COMPONENT_DATA);

    let landscape_asset_data = if selected_component_data.kind == Some(ComponentKind::Landscape) {
        saved_context.landscapes.as_ref().and_then(|landscapes| {
            landscapes
                .iter()
                .find(|landscape| landscape.id == selected_component_data.asset_id)
        })
    } else {
        None
    };

    let empty_landscape_asset_data = &&LandscapeData {
        id: "".to_string(),
        heightmap: None,
        rockmap: None,
        soil: None,
    };

    let landscape_asset_data = landscape_asset_data.unwrap_or(empty_landscape_asset_data);

    let available_textures = saved_context.textures.clone().unwrap_or(Vec::new());

    // components = instances
    // assets = originals

    html! {
        <>
            if selected_component_data.kind.is_some() {
                <section>
                    <h1>{"Property Management"}</h1>
                    <div class="view-row">
                        <div class="panel">
                            <span>{"Generic Properties"}</span>
                        </div>
                        if selected_component_data.kind == Some(ComponentKind::Landscape) {
                            <div class="panel">
                                <span>{"Landscape Properties"}</span>
                                <p>{"This is where you will associate textures with maps (soil, rocks)."}</p>
                                {selected_component_data.id.clone()}
                                {landscape_asset_data.id.clone()}
                                <div>
                                    <div>
                                        <label>{"Primary Texture"}</label>
                                        <select
                                            onchange={{
                                                let selected_component_id = selected_component_id.clone();
                                                let saved_context = saved_context.clone();

                                                Callback::from(move |e: Event| {
                                                    let selected_component_id = selected_component_id.clone();
                                                    let saved_context = saved_context.clone();
                                                    let input = e.target_dyn_into::<HtmlSelectElement>();

                                                    if let Some(input) = input {
                                                        saved_context.dispatch(
                                                            SavedAction::SetLandscapeTexture(selected_component_id.clone(),
                                                            LandscapeTextureKinds::Primary,
                                                            input.value()
                                                        ));
                                                    }
                                                })
                                            }}
                                        >
                                            {available_textures.clone().into_iter().map(|texture| {
                                                html!{
                                                    <option value={texture.id}>{texture.fileName}</option>
                                                }
                                            }).collect::<Html>()}
                                        </select>
                                    </div>
                                    <div>
                                        <label>{"RockMap Texture"}</label>
                                        <select
                                            onchange={{
                                                let selected_component_id = selected_component_id.clone();
                                                let saved_context = saved_context.clone();

                                                Callback::from(move |e: Event| {
                                                    let selected_component_id = selected_component_id.clone();
                                                    let saved_context = saved_context.clone();
                                                    let input = e.target_dyn_into::<HtmlSelectElement>();

                                                    if let Some(input) = input {
                                                        saved_context.dispatch(
                                                            SavedAction::SetLandscapeTexture(selected_component_id.clone(),
                                                            LandscapeTextureKinds::Rockmap,
                                                            input.value()
                                                        ));
                                                    }
                                                })
                                            }}
                                        >
                                            {available_textures.clone().into_iter().map(|texture| {
                                                html!{
                                                    <option value={texture.id}>{texture.fileName}</option>
                                                }
                                            }).collect::<Html>()}
                                        </select>
                                    </div>
                                    <div>
                                        <label>{"Soil Texture"}</label>
                                        <select
                                            onchange={{
                                                let selected_component_id = selected_component_id.clone();
                                                let saved_context = saved_context.clone();

                                                Callback::from(move |e: Event| {
                                                    let selected_component_id = selected_component_id.clone();
                                                    let saved_context = saved_context.clone();
                                                    let input = e.target_dyn_into::<HtmlSelectElement>();

                                                    if let Some(input) = input {
                                                        saved_context.dispatch(
                                                            SavedAction::SetLandscapeTexture(selected_component_id.clone(),
                                                            LandscapeTextureKinds::Soil,
                                                            input.value()
                                                        ));
                                                    }
                                                })
                                            }}
                                        >
                                            {available_textures.clone().into_iter().map(|texture| {
                                                html!{
                                                    <option value={texture.id}>{texture.fileName}</option>
                                                }
                                            }).collect::<Html>()}
                                        </select>
                                    </div>
                                </div>
                            </div>
                        }
                        if selected_component_data.kind == Some(ComponentKind::Model) {
                            <div class="panel">
                                <span>{"Model Properties"}</span>
                            </div>
                        }
                    </div>
                </section>
            }
        </>
    }
}

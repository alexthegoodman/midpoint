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

use crate::renderer::core::handle_add_landscape_texture;

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

const EMPTY_LANDSCAPE_ASSET_DATA: LandscapeData = LandscapeData {
    id: String::new(),
    heightmap: None,
    rockmap: None,
    soil: None,
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

    web_sys::console::log_1(&format!("selected_component_id..., {}", selected_component_id).into());
    web_sys::console::log_1(&format!("saved_context.levels: {:#?}", saved_context.levels).into());

    let selected_component_data = saved_context.levels.as_ref().and_then(|levels| {
        levels
            .iter()
            .flat_map(|level| level.components.as_ref().unwrap_or(&NONE_VEC))
            .find(|component| component.id == selected_component_id)
            .cloned()
    });

    let selected_component_data = selected_component_data.unwrap_or(EMPTY_COMPONENT_DATA);

    web_sys::console::log_1(
        &format!(
            "selected_component kind..., {:#?}",
            selected_component_data.kind
        )
        .into(),
    );

    let landscape_asset_data = if selected_component_data.kind == Some(ComponentKind::Landscape) {
        saved_context.landscapes.as_ref().and_then(|landscapes| {
            landscapes
                .iter()
                .find(|landscape| landscape.id == selected_component_data.asset_id)
                .cloned()
        })
    } else {
        None
    };

    let landscape_asset_data = landscape_asset_data.unwrap_or(EMPTY_LANDSCAPE_ASSET_DATA);

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
                                                let local_context = local_context.clone();
                                                let available_textures = available_textures.clone();
                                                let selected_component_id = selected_component_id.clone();
                                                let asset_id = landscape_asset_data.clone().id;
                                                let saved_context = saved_context.clone();

                                                Callback::from(move |e: Event| {
                                                    let local_context = local_context.clone();
                                                    let available_textures = available_textures.clone();
                                                    let selected_component_id = selected_component_id.clone();
                                                    let saved_context = saved_context.clone();
                                                    let asset_id = asset_id.clone();
                                                    let input = e.target_dyn_into::<HtmlSelectElement>();

                                                    if let Some(input) = input {
                                                        saved_context.dispatch(
                                                            SavedAction::SetLandscapeTexture(
                                                                selected_component_id.clone(),
                                                                LandscapeTextureKinds::Primary,
                                                                input.value()
                                                            )
                                                        );

                                                        let landscape = saved_context.landscapes.as_ref().expect("No landscapes?").iter().find(|l| l.id == asset_id);

                                                        if let Some(texture) = available_textures.clone().iter().find(|t| t.id.clone() == input.value()) {
                                                            handle_add_landscape_texture(
                                                                local_context.current_project_id.clone().expect("Couldn't get project id"),
                                                                selected_component_id.clone(),
                                                                asset_id.clone(),
                                                                texture.fileName.clone(),
                                                                "Primary".to_string(),
                                                                landscape.clone().expect("No landscape?").heightmap.clone().expect("No heightmap?").fileName
                                                            );
                                                        } else {
                                                            web_sys::console::error_1(
                                                                &"Couldn't add landscape texture".into(),
                                                            );
                                                        }
                                                    }
                                                })
                                            }}
                                        >
                                            <option value="">{"Select Texture"}</option>
                                            {available_textures.clone().into_iter().map(|texture| {
                                                html!{
                                                    <option value={texture.id.clone()}>{texture.fileName.clone()}</option>
                                                }
                                            }).collect::<Html>()}
                                        </select>
                                    </div>
                                    <div>
                                        <label>{"RockMap Texture"}</label>
                                        <select
                                            onchange={{
                                                let local_context = local_context.clone();
                                                let available_textures = available_textures.clone();
                                                let selected_component_id = selected_component_id.clone();
                                                let asset_id = landscape_asset_data.clone().id;
                                                let saved_context = saved_context.clone();


                                                Callback::from(move |e: Event| {
                                                    let local_context = local_context.clone();
                                                    let available_textures = available_textures.clone();
                                                    let selected_component_id = selected_component_id.clone();
                                                    let saved_context = saved_context.clone();
                                                    let asset_id = asset_id.clone();
                                                    let input = e.target_dyn_into::<HtmlSelectElement>();

                                                    if let Some(input) = input {
                                                        saved_context.dispatch(
                                                            SavedAction::SetLandscapeTexture(
                                                                selected_component_id.clone(),
                                                                LandscapeTextureKinds::Rockmap,
                                                                input.value()
                                                            )
                                                        );

                                                        let landscape = saved_context.landscapes.as_ref().expect("No landscapes?").iter().find(|l| l.id == asset_id);

                                                        if let Some(texture) = available_textures.clone().iter().find(|t| t.id.clone() == input.value()) {
                                                            handle_add_landscape_texture(
                                                                local_context.current_project_id.clone().expect("Couldn't get project id"),
                                                                selected_component_id.clone(),
                                                                asset_id.clone(),
                                                                texture.fileName.clone(),
                                                                "Rockmap".to_string(),
                                                                landscape.clone().expect("No landscape?").rockmap.clone().expect("No rockmap?").fileName
                                                            );
                                                        } else {
                                                            web_sys::console::error_1(
                                                                &"Couldn't add landscape texture".into(),
                                                            );
                                                        }
                                                    }
                                                })
                                            }}
                                        >
                                            <option value="">{"Select Texture"}</option>
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
                                                let local_context = local_context.clone();
                                                let available_textures = available_textures.clone();
                                                let selected_component_id = selected_component_id.clone();
                                                let asset_id = landscape_asset_data.clone().id;
                                                let saved_context = saved_context.clone();

                                                Callback::from(move |e: Event| {
                                                    let local_context = local_context.clone();
                                                    let available_textures = available_textures.clone();
                                                    let selected_component_id = selected_component_id.clone();
                                                    let saved_context = saved_context.clone();
                                                    let asset_id = asset_id.clone();
                                                    let input = e.target_dyn_into::<HtmlSelectElement>();

                                                    if let Some(input) = input {
                                                        saved_context.dispatch(
                                                            SavedAction::SetLandscapeTexture(
                                                                selected_component_id.clone(),
                                                                LandscapeTextureKinds::Soil,
                                                                input.value()
                                                            )
                                                        );

                                                        let landscape = saved_context.landscapes.as_ref().expect("No landscapes?").iter().find(|l| l.id == asset_id);

                                                        if let Some(texture) = available_textures.clone().iter().find(|t| t.id.clone() == input.value()) {
                                                            handle_add_landscape_texture(
                                                                local_context.current_project_id.clone().expect("Couldn't get project id"),
                                                                selected_component_id.clone(),
                                                                asset_id.clone(),
                                                                texture.fileName.clone(),
                                                                "Soil".to_string(),
                                                                landscape.clone().expect("No landscape?").soil.clone().expect("No soil?").fileName
                                                            );
                                                        } else {
                                                            web_sys::console::error_1(
                                                                &"Couldn't add landscape texture".into(),
                                                            );
                                                        }
                                                    }
                                                })
                                            }}
                                        >
                                            <option value="">{"Select Texture"}</option>
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

use regex::bytes::Regex;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::{ops::Deref, path::PathBuf};
use uuid::Uuid;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use web_sys::{Event, FileReader, HtmlInputElement};
use yew::prelude::*;

use crate::contexts::local::LocalAction;
use crate::contexts::saved::{
    ComponentData, ComponentKind, GenericProperties, LandscapeData, LandscapeProperties,
    SavedAction, SavedContextType,
};
use crate::gql::generateTexture::generate_texture;
use crate::renderer::core::handle_add_landscape;
use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
    components::MdFileInput::MdFileInput,
    contexts::{local::LocalContextType, saved::File},
    gql::generateConcept::generate_concept,
    gql::generateModel::generate_model,
    renderer::core::handle_add_model,
};

#[derive(Clone, PartialEq)]
pub enum FileVariant {
    Asset,
    Concept,
}

#[derive(Clone, PartialEq)]
pub enum FileKind {
    Model,
    Image,
    Landscape,
    Texture,
}

#[derive(Clone, PartialEq, Properties)]
pub struct FileBrowserProps {
    pub variant: FileVariant,
    pub kind: FileKind,
    pub files: Option<Vec<File>>,
    pub landscapes: Option<Vec<LandscapeData>>,
}

#[derive(Serialize)]
struct SaveConceptParams {
    projectId: String,
    conceptBase64: String,
    conceptFilename: String,
}

#[derive(Serialize)]
struct SaveTextureParams {
    projectId: String,
    textureBase64: String,
    textureFilename: String,
}

#[derive(Serialize)]
struct SaveModelParams {
    projectId: String,
    modelBase64: String,
    modelFilename: String,
}

#[derive(Serialize)]
struct SaveLandscapeParams {
    projectId: String,
    landscapeBase64: String,
    landscapeFilename: String,
    rockmapFilename: String,
    rockmapBase64: String,
    soilFilename: String,
    soilBase64: String,
}

pub fn getFilename(concept_prompt_str: String) -> String {
    let conceptFilename: String = concept_prompt_str.chars().skip(0).take(20).collect();

    let re = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
    let conceptFilename = re.replace_all(conceptFilename.as_bytes(), b"_");
    let conceptFilename = std::str::from_utf8(&conceptFilename).expect("Couldn't convert filename");

    let conceptFilename = format!("{}-{}", conceptFilename, Uuid::new_v4());

    conceptFilename
}

fn change_extension_to_glb(filename: &str) -> String {
    let mut path = PathBuf::from(filename);
    path.set_extension("glb");
    path.to_string_lossy().into_owned()
}

#[function_component]
pub fn FileBrowser(props: &FileBrowserProps) -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");
    let saved_context = use_context::<SavedContextType>().expect("No SavedContext found");

    let loading = use_state(|| false);

    let prompt_value = use_state(String::default);

    let landscape_filename = use_state(|| "".to_string());
    let landscape_base64 = use_state(|| "".to_string());
    let rockmap_filename = use_state(|| "".to_string());
    let rockmap_base64 = use_state(|| "".to_string());
    let soil_filename = use_state(|| "".to_string());
    let soil_base64 = use_state(|| "".to_string());

    let loading = use_state(|| false);

    let handle_prompt_change = {
        let prompt_value = prompt_value.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>();

            if let Some(input) = input {
                prompt_value.set(input.value());
            }
        })
    };

    let callback = {
        let loading = loading.clone();

        Callback::from(move |_| {
            loading.set(false);
        })
    };

    // Convert the callback to a js_sys::Function
    let js_callback: js_sys::Function = Closure::wrap(Box::new(move || {
        callback.emit(());
    }) as Box<dyn FnMut()>)
    .into_js_value()
    .unchecked_into();

    html! {
        <section class="file-browser">
            if props.variant == FileVariant::Concept {
                <div class="file-prompt">
                    <label>{"Describe your concept"}</label>
                    <textarea onchange={handle_prompt_change.clone()} rows="3">{(*prompt_value).clone()}</textarea>
                    <MdButton
                        label="Generate"
                        icon={""}
                        on_click={Callback::from({
                            let prompt_value = prompt_value.clone();
                            let local_context = local_context.clone();
                            let loading = loading.clone();

                            move |_| {
                                let prompt_value = prompt_value.clone();
                                let local_context = local_context.clone();
                                let loading = loading.clone();

                                loading.set(true);

                                web_sys::console::log_1(&"Generating concept...".into());

                                spawn_local(async move {
                                    // generate concept via GraphQL or Socket
                                    let concept_data = generate_concept(local_context.token.clone().expect("Failed token fetch"), (*prompt_value).clone()).await;

                                    web_sys::console::log_1(&"Concept generated, saving now...".into());

                                    let conceptBase64 = concept_data.expect("Couldn't unwrap concept data").generateConcept;

                                    // determine filename
                                    let concept_prompt_str: String = (*prompt_value).clone();
                                    let conceptFilename = getFilename(concept_prompt_str);
                                    let conceptFilename = conceptFilename + ".png";

                                    web_sys::console::log_1(&conceptFilename.clone().into());

                                    // save as image inside folder within sync folder: /CommonOSFiles/midpoint/projects/project_id/concepts/
                                    let projectId = local_context.current_project_id.clone().expect("No project selected?");
                                    let params = to_value(&SaveConceptParams {
                                        projectId,
                                        conceptBase64,
                                        conceptFilename
                                    }).unwrap();
                                    let result = crate::app::invoke("save_concept", params).await;

                                    loading.set(false);
                                });
                            }
                        })}
                        disabled={*loading}
                        loading={*loading}
                        kind={MdButtonKind::SmallShort}
                        variant={MdButtonVariant::Green}
                    />
                </div>
            }
            if props.kind == FileKind::Landscape {
                <>
                    <h5>{"Select Landscape Heightmap (.TIF)"}</h5>
                    <MdFileInput
                        filename={landscape_filename.clone()}
                        base64={landscape_base64.clone()}
                    />

                    <p>{"Now select masks for rocks, soil, etc (.PNG)"}</p>
                    <div>
                        <label>{"RockMap"}</label>
                        <MdFileInput
                            filename={rockmap_filename.clone()}
                            base64={rockmap_base64.clone()}
                        />
                    </div>
                    <div>
                        <label>{"Soil"}</label>
                        <MdFileInput
                            filename={soil_filename.clone()}
                            base64={soil_base64.clone()}
                        />
                    </div>

                    <MdButton
                        label="Save Landscape"
                        icon={""}
                        on_click={Callback::from({
                            let local_context = local_context.clone();
                            let loading = loading.clone();

                            let landscape_filename = landscape_filename.clone();
                            let landscape_base64 = landscape_base64.clone();
                            let rockmap_filename = rockmap_filename.clone();
                            let rockmap_base64 = rockmap_base64.clone();
                            let soil_filename = soil_filename.clone();
                            let soil_base64 = soil_base64.clone();

                            move |_| {
                                let local_context = local_context.clone();
                                let loading = loading.clone();

                                let landscape_filename = landscape_filename.clone();
                                let landscape_base64 = landscape_base64.clone();
                                let rockmap_filename = rockmap_filename.clone();
                                let rockmap_base64 = rockmap_base64.clone();
                                let soil_filename = soil_filename.clone();
                                let soil_base64 = soil_base64.clone();

                                loading.set(true);

                                web_sys::console::log_1(&"Saving landscape...".into());

                                spawn_local(async move {
                                    // determine filename
                                    let landscapeFilename = (*landscape_filename).clone();
                                    let landscapeBase64 = (*landscape_base64).clone();
                                    let rockmapFilename = (*rockmap_filename).clone();
                                    let rockmapBase64 = (*rockmap_base64).clone();
                                    let soilFilename = (*soil_filename).clone();
                                    let soilBase64 = (*soil_base64).clone();

                                    web_sys::console::log_1(&landscapeFilename.clone().into());

                                    // save as image inside folder within sync folder: /CommonOSFiles/midpoint/projects/project_id/concepts/
                                    let projectId = local_context.current_project_id.clone().expect("No project selected?");
                                    let params = to_value(&SaveLandscapeParams {
                                        projectId,
                                        landscapeBase64,
                                        landscapeFilename,
                                        rockmapFilename,
                                        rockmapBase64,
                                        soilFilename,
                                        soilBase64,
                                    }).unwrap();
                                    let result = crate::app::invoke("save_landscape", params).await;

                                    loading.set(false);
                                });
                            }
                        })}
                        disabled={*loading}
                        loading={*loading}
                        kind={MdButtonKind::SmallShort}
                        variant={MdButtonVariant::Green}
                    />



                    <div class="file-grid">
                        {
                            props.landscapes.clone().unwrap_or_default().into_iter().map(|landscape| {
                                let landscape_id = landscape.id.clone();
                                let hasRockmap = landscape.rockmap.is_some();
                                let hasSoil = landscape.soil.is_some();
                                let loading = loading.clone();
                                let heightmapCloudfrontUrl = landscape.heightmap.clone().unwrap_or_default().cloudfrontUrl.clone();
                                let heightmapFilename = landscape.heightmap.clone().unwrap_or_default().fileName.clone();

                                // let heightmapId = landscape.heightmap.clone().unwrap_or_default().id.clone();
                                // let rockmapId = landscape.rockmap.clone().unwrap_or_default().id.clone();
                                // let soilId = landscape.soil.clone().unwrap_or_default().id.clone();

                                html!{
                                    <div class="file-item" key={landscape_id.clone()}>
                                        <span>{heightmapFilename.clone()}</span>
                                        if hasRockmap {
                                            <span>{"Has RockMap"}</span>
                                        }
                                        if hasSoil {
                                            <span>{"Has Soil"}</span>
                                        }
                                        <MdButton
                                            label="Add to Scene"
                                            icon={""}
                                            on_click={Callback::from({
                                                let local_context = local_context.clone();
                                                let saved_context = saved_context.clone();
                                                let loading = loading.clone();
                                                let js_callback = js_callback.clone();
                                                let fileName = heightmapFilename.clone();

                                                move |_| {
                                                    let local_context = local_context.clone();
                                                    let saved_context = saved_context.clone();
                                                    let loading = loading.clone();
                                                    let js_callback = js_callback.clone();

                                                    web_sys::console::log_1(&"Adding landscape to scene...".into());

                                                    loading.set(true);

                                                    let projectId = local_context.current_project_id.clone().expect("No project selected?");
                                                    let landscapeFilename = fileName.clone();
                                                    let landscapeComponentId = Uuid::new_v4().to_string();

                                                    // add to `levels.components` in SavedContext
                                                    saved_context.dispatch(SavedAction::AddComponent(ComponentData {
                                                        id: landscapeComponentId.clone(),
                                                        kind: Some(ComponentKind::Landscape),
                                                        asset_id: landscape_id.clone(),
                                                        generic_properties: GenericProperties {
                                                            name: "New Landscape Component".to_string()
                                                        },
                                                        landscape_properties: Some(LandscapeProperties {
                                                            // primary_texture_id: heightmapId.clone(),
                                                            // rockmap_texture_id: rockmapId.clone(),
                                                            // soil_texture_id: soilId.clone()
                                                            // these are the visible texture ids, not the map ids, so are added after adding
                                                            primary_texture_id: None,
                                                            rockmap_texture_id: None,
                                                            soil_texture_id: None
                                                        }),
                                                        model_properties: None
                                                    }));

                                                    // update selected_component_id in LocalContext
                                                    local_context.dispatch(LocalAction::SetSelectedComponent(landscapeComponentId.clone()));

                                                    // actually render the landscape in wgpu
                                                    handle_add_landscape(projectId, landscape_id.clone(), landscapeComponentId.clone(), landscapeFilename, js_callback);
                                                }
                                            })}
                                            disabled={*loading}
                                            loading={*loading}
                                            kind={MdButtonKind::SmallShort}
                                            variant={MdButtonVariant::Green}
                                        />
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </>
            }
            if props.kind == FileKind::Image {
                <div class="file-grid">
                    {
                        props.files.clone().unwrap_or_default().into_iter().map(|file| {

                            html!{
                                <div class="file-item" key={file.clone().id}>
                                    <img src={file.clone().cloudfrontUrl} />
                                    <span>{file.clone().fileName}</span>
                                    <MdButton
                                        label="Generate Model"
                                        icon={""}
                                        on_click={Callback::from({
                                            let local_context = local_context.clone();
                                            let loading = loading.clone();
                                            let file = file.clone();
                                            let cloudfrontUrl = file.cloudfrontUrl.clone();
                                            let fileName = file.fileName.clone();

                                            move |_| {
                                                let local_context = local_context.clone();
                                                let loading = loading.clone();
                                                let cloudfrontUrl = cloudfrontUrl.clone();
                                                let fileName = fileName.clone();

                                                loading.set(true);
                                                // local_context.dispatch(LocalAction::SetRoute("/".to_string()));

                                                web_sys::console::log_1(&"Generating model...".into());

                                                spawn_local(async move {

                                                    let model_data = generate_model(local_context.token.clone().expect("Failed token fetch"), cloudfrontUrl).await;

                                                    web_sys::console::log_1(&"Concept generated, saving now...".into());

                                                    let modelBase64 = model_data.expect("Couldn't unwrap model data").generateModel;

                                                    // determine filename
                                                    let modelFilename = change_extension_to_glb(&fileName);

                                                    web_sys::console::log_1(&modelFilename.clone().into());

                                                    // save as image inside folder within sync folder: /CommonOSFiles/midpoint/projects/project_id/concepts/
                                                    let projectId = local_context.current_project_id.clone().expect("No project selected?");
                                                    let params = to_value(&SaveModelParams {
                                                        projectId,
                                                        modelBase64,
                                                        modelFilename
                                                    }).unwrap();
                                                    let result = crate::app::invoke("save_model", params).await;

                                                    loading.set(false);
                                                });
                                            }
                                        })}
                                        disabled={*loading}
                                        loading={*loading}
                                        kind={MdButtonKind::SmallShort}
                                        variant={MdButtonVariant::Green}
                                    />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
            if props.kind == FileKind::Model {
                <div class="file-grid">
                    {
                        props.files.clone().unwrap_or_default().into_iter().map(|file| {
                            let cloudfrontUrl = file.cloudfrontUrl.clone();

                            html!{
                                <div class="file-item" key={file.id}>
                                    <span>{file.fileName.clone()}</span>
                                    <MdButton
                                        label="Add to Scene"
                                        icon={""}
                                        on_click={Callback::from({
                                            let local_context = local_context.clone();
                                            let loading = loading.clone();
                                            let fileName = file.fileName.clone();

                                            move |_| {
                                                let local_context = local_context.clone();
                                                let loading = loading.clone();

                                                web_sys::console::log_1(&"Adding model to scene...".into());

                                                let projectId = local_context.current_project_id.clone().expect("No project selected?");
                                                let modelFilename = fileName.clone();

                                                handle_add_model(projectId, modelFilename);
                                            }
                                        })}
                                        disabled={*loading}
                                        loading={*loading}
                                        kind={MdButtonKind::SmallShort}
                                        variant={MdButtonVariant::Green}
                                    />
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
            if props.kind == FileKind::Texture {
                <>
                    <div class="file-prompt">
                        <label>{"Describe your texture"}</label>
                        <textarea onchange={handle_prompt_change.clone()} rows="3">{(*prompt_value).clone()}</textarea>
                        <MdButton
                            label="Generate"
                            icon={""}
                            on_click={Callback::from({
                                let prompt_value = prompt_value.clone();
                                let local_context = local_context.clone();
                                let loading = loading.clone();

                                move |_| {
                                    let prompt_value = prompt_value.clone();
                                    let local_context = local_context.clone();
                                    let loading = loading.clone();

                                    loading.set(true);

                                    web_sys::console::log_1(&"Generating texture...".into());

                                    spawn_local(async move {
                                        // generate concept via GraphQL or Socket
                                        let texture_data = generate_texture(local_context.token.clone().expect("Failed token fetch"), (*prompt_value).clone()).await;

                                        web_sys::console::log_1(&"Texture generated, saving now...".into());

                                        let textureBase64 = texture_data.expect("Couldn't unwrap texture data").generateTexture;

                                        // determine filename
                                        let texture_prompt_str: String = (*prompt_value).clone();
                                        let textureFilename = getFilename(texture_prompt_str);
                                        let textureFilename = textureFilename + ".png";

                                        web_sys::console::log_1(&textureFilename.clone().into());

                                        // save as image inside folder within sync folder: /CommonOSFiles/midpoint/projects/project_id/concepts/
                                        let projectId = local_context.current_project_id.clone().expect("No project selected?");
                                        let params = to_value(&SaveTextureParams {
                                            projectId,
                                            textureBase64,
                                            textureFilename
                                        }).unwrap();
                                        let result = crate::app::invoke("save_texture", params).await;

                                        loading.set(false);
                                    });
                                }
                            })}
                            disabled={*loading}
                            loading={*loading}
                            kind={MdButtonKind::SmallShort}
                            variant={MdButtonVariant::Green}
                        />
                    </div>
                </>
                <div class="file-grid">
                    {
                        props.files.clone().unwrap_or_default().into_iter().map(|file| {

                            html!{
                                <div class="file-item" key={file.clone().id}>
                                    <img src={file.clone().cloudfrontUrl} />
                                    <span>{file.clone().fileName}</span>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </section>
    }
}

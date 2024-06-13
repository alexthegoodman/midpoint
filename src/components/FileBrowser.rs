use regex::bytes::Regex;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::path::PathBuf;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
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
}

#[derive(Clone, PartialEq, Properties)]
pub struct FileBrowserProps {
    pub variant: FileVariant,
    pub kind: FileKind,
    pub files: Vec<File>,
}

#[derive(Serialize)]
struct SaveConceptParams {
    projectId: String,
    conceptBase64: String,
    conceptFilename: String,
}

#[derive(Serialize)]
struct SaveModelParams {
    projectId: String,
    modelBase64: String,
    modelFilename: String,
}

#[derive(Serialize)]
pub struct ReadModelParams {
    pub projectId: String,
    pub modelFilename: String,
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

    let loading = use_state(|| false);

    let concept_prompt_value = use_state(String::default);

    let handle_concept_prompt_change = {
        let concept_prompt_value = concept_prompt_value.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>();

            if let Some(input) = input {
                concept_prompt_value.set(input.value());
            }
        })
    };

    html! {
        <section class="file-browser">
            if props.variant == FileVariant::Concept {
                <div class="file-prompt">
                    <label>{"Describe your concept"}</label>
                    <textarea onchange={handle_concept_prompt_change} rows="3">{(*concept_prompt_value).clone()}</textarea>
                    <MdButton
                        label="Generate"
                        icon={""}
                        on_click={Callback::from({
                            let local_context = local_context.clone();
                            let loading = loading.clone();

                            move |_| {
                                let concept_prompt_value = concept_prompt_value.clone();
                                let local_context = local_context.clone();
                                let loading = loading.clone();

                                loading.set(true);

                                web_sys::console::log_1(&"Generating concept...".into());

                                spawn_local(async move {
                                    // generate concept via GraphQL or Socket
                                    let concept_data = generate_concept(local_context.token.clone().expect("Failed token fetch"), (*concept_prompt_value).clone()).await;

                                    web_sys::console::log_1(&"Concept generated, saving now...".into());

                                    let conceptBase64 = concept_data.expect("Couldn't unwrap concept data").generateConcept;

                                    // determine filename
                                    let concept_prompt_str: String = (*concept_prompt_value).clone();
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

                                    // local_context.dispatch(LocalAction::SetCurrentProject(projectId.clone()));

                                    loading.set(false);

                                    // local_context.dispatch(LocalAction::SetRoute("/concepts".to_string()));
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
            if props.kind == FileKind::Image {
                <div class="file-grid">
                    {
                        props.files.clone().into_iter().map(|file| {

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
                        props.files.clone().into_iter().map(|file| {
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
        </section>
    }
}

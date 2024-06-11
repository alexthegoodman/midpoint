use regex::bytes::Regex;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
    contexts::local::LocalContextType,
    gql::generateConcept::generate_concept,
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
}

#[derive(Serialize)]
struct SaveConceptParams {
    projectId: String,
    conceptBase64: String,
    conceptFilename: String,
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
        <section>
            if props.variant == FileVariant::Concept {
                <div>
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

                                    let conceptBase64 = concept_data.expect("Couldn't unwrap project").generateConcept;

                                    // determine filename
                                    let concept_prompt_str: String = (*concept_prompt_value).clone();
                                    let conceptFilename: String = concept_prompt_str.chars().skip(0).take(20).collect();

                                    let re = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
                                    let conceptFilename = re.replace_all(conceptFilename.as_bytes(), b"_");
                                    let conceptFilename = std::str::from_utf8(&conceptFilename).expect("Couldn't convert filename");

                                    let conceptFilename = format!("{}-{}", conceptFilename, Uuid::new_v4());

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
        </section>
    }
}

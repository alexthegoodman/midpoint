use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::{
    components::MdButton::{MdButton, MdButtonKind, MdButtonVariant},
    contexts::local::LocalContextType,
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
                <>
                    <label>{"Describe your concept"}</label>
                    <textarea onchange={handle_concept_prompt_change} rows="3">{(*concept_prompt_value).clone()}</textarea>
                    <MdButton
                        label="Generate"
                        icon={""}
                        on_click={Callback::from({
                            let local_context = local_context.clone();
                            move |_| {
                                loading.set(true);
                                // local_context.dispatch(LocalAction::SetRoute("/map".to_string()));
                                loading.set(false);
                            }
                        })}
                        disabled={false}
                        loading={false}
                        kind={MdButtonKind::SmallShort}
                        variant={MdButtonVariant::Green}
                    />
                </>
            }
        </section>
    }
}

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, FileReader, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MdFileInputProps {
    pub filename: UseStateHandle<String>,
    pub base64: UseStateHandle<String>,
}

#[function_component]
pub fn MdFileInput(props: &MdFileInputProps) -> Html {
    html! {
        <input
            type="file"
            onchange={{
                let filename = props.filename.clone();
                let base64 = props.base64.clone();
                Callback::from(move |e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    if let Some(files) = input.files() {
                        if let Some(file) = files.get(0) {
                            let file_name = file.name();
                            let filename = filename.clone();
                            let base64 = base64.clone();
                            let reader = FileReader::new().unwrap();

                            filename.set(file_name);

                            let reader_clone = reader.clone();
                            let reader_onload = Closure::wrap(Box::new(move |e: Event| {
                                let result = reader_clone.result().unwrap();
                                let result_str = result.as_string().unwrap();
                                let base64_content = result_str.split(',').nth(1).unwrap().to_string();
                                base64.set(base64_content);
                            }) as Box<dyn FnMut(_)>);

                            reader.set_onload(Some(reader_onload.as_ref().unchecked_ref()));
                            reader.read_as_data_url(&file).unwrap();
                            reader_onload.forget();
                        }
                    }
                })
            }}
        />
    }
}

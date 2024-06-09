use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::contexts::local::{LocalAction, LocalContextType};

#[derive(Clone, PartialEq, Properties)]
pub struct FileBrowserProps {}

#[derive(Serialize)]
struct ReadAuthTokenParams {
    // token: String,
}

#[function_component]
pub fn TokenLoader(props: &FileBrowserProps) -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");

    use_effect_with((), move |_| {
        spawn_local(async move {
            // invoke to read token from local file
            let params = to_value(&ReadAuthTokenParams {}).unwrap();
            let result = crate::app::invoke("read_token", params).await;
            let token = result.as_string().expect("Couldn't unwrap auth token");

            local_context.dispatch(LocalAction::SetToken(token));
        });
    });

    html! {
        <></>
    }
}

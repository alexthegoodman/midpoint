use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MessageEvent, WebSocket};
use yew::functional::*;
use yew::prelude::*;

use crate::contexts::local::LocalContextType;
use crate::contexts::saved::SavedAction;
use crate::contexts::saved::SavedContextType;
use crate::gql::getMdProject::get_md_project;

#[derive(Serialize)]
struct JoinGroupPayload {
    groupId: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectSocketProps {
    // pub project_id: String,
}

#[function_component(ProjectSocket)]
pub fn project_socket(props: &ProjectSocketProps) -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");
    let saved_context = use_context::<SavedContextType>().expect("No SavedContext found");

    // Use state to hold the WebSocket connection
    // let ws: UseStateHandle<Option<WebSocket>> = use_state(|| None);
    let ws: UseStateHandle<Option<Rc<RefCell<WebSocket>>>> = use_state(|| None);
    let ws_ref = ws.clone();

    // Function to send a message
    let join_group = {
        // let ws = ws.clone();
        let props = props.clone();
        let local_context = local_context.clone();

        move |ws: Option<Rc<RefCell<WebSocket>>>| {
            web_sys::console::info_1(&"About to join...".into());

            if let Some(ws) = &ws {
                web_sys::console::info_1(
                    &format!(
                        "Joining group: {}",
                        local_context
                            .current_project_id
                            .clone()
                            .expect("Couldn't log project id")
                    )
                    .into(),
                );

                let msg = serde_json::json!({
                    "Authorization": "Bearer ".to_owned() + &local_context.token.clone().expect("Token not found during socket message"),
                    "event": "join",
                    "payload": &JoinGroupPayload {
                        groupId: local_context.current_project_id.clone().expect("Couldn't fetch project id")
                    }
                })
                .to_string();
                ws.borrow().send_with_str(&msg).unwrap();
            }
        }
    };

    // Function to disconnect
    // let disconnect = {
    //     let ws = ws.clone();
    //     Callback::from(move |_| {
    //         if let Some(ws) = &*ws {
    //             ws.close().unwrap();
    //             // ws.set(None);
    //         }
    //     })
    // };

    let local_context = local_context.clone();

    // Effect to establish WebSocket connection
    use_effect_with(local_context.current_project_id.clone(), move |_| {
        web_sys::console::info_1(&"Check ws".into());

        if local_context.current_project_id.is_some() {
            web_sys::console::info_1(&"Check ws again".into());

            let ws = WebSocket::new("ws://localhost:4000").unwrap();
            let ws_rc = Rc::new(RefCell::new(ws));

            // Setup onmessage event
            let onmessage_callback = {
                let ws_rc = ws_rc.clone();
                // let local_context = local_context.clone();
                // let saved_context = saved_context.clone();

                Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        web_sys::console::info_1(&format!("Received message: {}", txt).into());

                        let local_context = local_context.clone();
                        let saved_context = saved_context.clone();

                        // {"command":"refreshContext"}
                        if txt == r#"{"command":"refreshContext"}"#.to_string() {
                            spawn_local(async move {
                                let md_project = get_md_project(
                                    local_context.token.clone().expect("Failed token fetch"),
                                    local_context
                                        .current_project_id
                                        .clone()
                                        .expect("Couldn't fetch project id"),
                                )
                                .await;
                                let latest_project = md_project
                                    .expect("Couldn't unwrap project context")
                                    .getMdProject;
                                let updated_context = latest_project.context;

                                saved_context
                                    .dispatch(SavedAction::RefreshContext(updated_context));
                            });
                        }
                    }
                })
            };

            ws_rc
                .borrow()
                .set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();

            // Setup onopen event
            let join_group_clone = join_group.clone();
            let onopen_callback = {
                let ws_rc = ws_rc.clone();
                Closure::<dyn FnMut(Event)>::new(move |_| {
                    web_sys::console::info_1(&"WebSocket connection established".into());
                    join_group_clone(Some(ws_rc.clone()));
                })
            };

            ws_rc
                .borrow()
                .set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();

            // Update state with the WebSocket connection
            ws_ref.set(Some(ws_rc));
        }

        // || () // Cleanup function if needed
    });

    html! {
        <></>
    }
}

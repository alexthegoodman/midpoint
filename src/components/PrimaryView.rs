use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::MdButton::{MdButton, MdButtonKind, MdButtonVariant};
use crate::components::SceneView::SceneView;
use crate::contexts::local::LocalContextType;
use crate::gql::createMdProject::create_md_project;

#[derive(Serialize)]
struct CreateProjectParams {
    projectId: String,
}

#[function_component(PrimaryView)]
pub fn primary_view() -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");

    // let local_state = &*local_context;

    let loading = use_state(|| false);

    let mut scene_display = "none".to_string();
    if local_context.route == "/scene".to_string() {
        scene_display = "block".to_string();
    }

    html! {
        <section>
            if local_context.route == "/".to_string() {
                <>
                    <h1>{"Welcome to Midpoint."}</h1>
                    <h2>{"Let's get started"}</h2>

                    if local_context.token.is_some() {
                        <h6>{"Great! You're logged in via CommonOS File Manager."}</h6>
                    } else {
                        <h6>{"Oh no! Please login via CommonOS File Manager."}</h6>
                    }

                    <pre>{"CommonOS File Manager should be running to assure file syncing. New projects are automatically placed into your Sync Folder."}</pre>
                    <p>{"Begin by creating a new project (with default settings) or opening an existing one:"}</p>

                    <div class="btn-row">
                        <MdButton
                            label="New Project"
                            icon={""}
                            on_click={Callback::from({
                                let local_context = local_context.clone();
                                let loading = loading.clone();
                                move |_| {
                                    let local_context = local_context.clone();
                                    let loading = loading.clone();

                                    loading.set(true);

                                    web_sys::console::log_1(&"Creating project...".into());

                                    spawn_local(async move {
                                        // create cloud project via GraphQL or Socket
                                        let md_project = create_md_project(local_context.token.clone().expect("Failed token fetch")).await;
                                        let projectId = md_project.expect("Couldn't unwrap project").createMdProject.id;

                                        // create project folder within sync folder: /CommonOSFiles/midpoint/projects/project_id/
                                        let params = to_value(&CreateProjectParams { projectId }).unwrap();
                                        let result = crate::app::invoke("create_project", params).await;

                                        loading.set(false);
                                    });

                                }
                            })}
                            disabled={*loading}
                            kind={MdButtonKind::SmallShort}
                            variant={MdButtonVariant::Green}
                        />
                        <MdButton
                            label="Open Project"
                            icon={""}
                            on_click={Callback::from({
                                let local_context = local_context.clone();
                                move |_| {
                                    // local_context.dispatch(LocalAction::SetRoute("/map".to_string()));
                                }
                            })}
                            disabled={false}
                            kind={MdButtonKind::SmallShort}
                            variant={MdButtonVariant::Green}
                        />
                    </div>
                </>
            } else if local_context.route == "/concepts".to_string() {
                <>
                    // <FileBrowser />
                    // <section>
                    //     <FileViewer />
                    // </section>
                </>
            }

            <div style={"display: ".to_owned() + &scene_display}>
                <SceneView />
            </div>
        </section>
    }
}

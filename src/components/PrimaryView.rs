use yew::prelude::*;

use crate::components::MdButton::{MdButton, MdButtonKind, MdButtonVariant};
use crate::components::SceneView::SceneView;
use crate::contexts::local::LocalContextType;

#[function_component(PrimaryView)]
pub fn primary_view() -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");

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
                                move |_| {
                                    // local_context.dispatch(LocalAction::SetRoute("/map".to_string()));

                                    // create cloud project via GraphQL or Socket
                                    // create project folder within sync folder: /CommonOSFiles/midpoint/projects/project_id/
                                }
                            })}
                            disabled={false}
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

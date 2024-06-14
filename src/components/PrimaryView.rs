use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::FileBrowser::{FileBrowser, FileKind, FileVariant};
use crate::components::FileViewer::FileViewer;
use crate::components::MdButton::{MdButton, MdButtonKind, MdButtonVariant};
use crate::components::SceneView::SceneView;
use crate::contexts::local::{LocalAction, LocalContextType};
use crate::contexts::localAsync::LocalAsync;
use crate::contexts::saved::{SavedAction, SavedContextType};
use crate::gql::createMdProject::create_md_project;
use crate::gql::deleteMdProject::delete_md_project;
use crate::gql::getMdProject::get_md_project;

#[derive(Serialize)]
struct CreateProjectParams {
    projectId: String,
}

#[function_component(PrimaryView)]
pub fn primary_view() -> Html {
    let local_context = use_context::<LocalContextType>().expect("No LocalContext found");
    let saved_context = use_context::<SavedContextType>().expect("No SavedContext found");

    let loading = use_state(|| false);
    let browser_tab = use_state(|| "models".to_string());

    let local_async = LocalAsync::new(local_context.clone());

    let on_refresh = {
        let local_async = local_async.clone();
        move || {
            local_async.refresh_projects();
        }
    };

    {
        let local_context = local_context.clone();

        use_effect_with(local_context.token.clone(), move |_| {
            if local_context.token.is_some() {
                on_refresh();
            }
        });
    }

    let mut scene_display = "none".to_string();
    if local_context.route == "/scene".to_string() {
        scene_display = "flex".to_string();
    }

    html! {
        <section>
            if local_context.route == "/".to_string() {
                <>
                    <h1>{"Welcome to Midpoint."}</h1>
                    <h2>{"Let's get started"}</h2>

                    <pre>{"CommonOS File Manager should be running to assure file syncing. New projects are automatically placed into your Sync Folder."}</pre>

                    if local_context.token.is_some() {
                        <>
                            <h6>{"Great! You're logged in via CommonOS File Manager."}</h6>
                            <p>{"Begin by creating a new project (with default settings) or opening an existing one:"}</p>

                            if local_context.projects.is_some() {
                                <h5>{"Existing Projects"}</h5>
                                <section>
                                    {
                                        local_context.projects.clone().expect("Couldn't unwrap existing projects").into_iter().map(|project| {
                                            html!{
                                                <div class="project-item" key={project.clone().id}>
                                                    <span>{project.clone().title}</span>
                                                    <span>{"Created At: "}{project.clone().createdAt}</span>
                                                    <MdButton
                                                        label="Open Project"
                                                        icon={""}
                                                        on_click={Callback::from({
                                                            let local_context = local_context.clone();
                                                            let saved_context = saved_context.clone();
                                                            let loading = loading.clone();
                                                            let project = project.clone();

                                                            move |_| {
                                                                let local_context = local_context.clone();
                                                                let saved_context = saved_context.clone();
                                                                let loading = loading.clone();
                                                                let project = project.clone();

                                                                loading.set(true);

                                                                web_sys::console::log_1(&"Opening project...".into());

                                                                spawn_local(async move {
                                                                    let projectId = project.clone().id;

                                                                    let md_project = get_md_project(
                                                                        local_context.token.clone().expect("Failed token fetch"),
                                                                        projectId.clone(),
                                                                    )
                                                                    .await;
                                                                    let the_project = md_project
                                                                        .expect("Couldn't unwrap project context")
                                                                        .getMdProject;
                                                                    let updated_context = the_project.context;

                                                                    saved_context
                                                                        .dispatch(SavedAction::RefreshContext(updated_context));

                                                                    local_context.dispatch(LocalAction::SetCurrentProject(projectId.clone()));

                                                                    loading.set(false);

                                                                    local_context.dispatch(LocalAction::SetRoute("/concepts".to_string()));
                                                                });

                                                            }
                                                        })}
                                                        disabled={*loading}
                                                        loading={*loading}
                                                        kind={MdButtonKind::SmallShort}
                                                        variant={MdButtonVariant::Green}
                                                    />
                                                    <MdButton
                                                        label=""
                                                        icon={"trash"}
                                                        on_click={Callback::from({
                                                            let local_context = local_context.clone();
                                                            let loading = loading.clone();
                                                            let project = project.clone();
                                                            let local_async = local_async.clone();

                                                            move |_| {
                                                                let local_context = local_context.clone();
                                                                let loading = loading.clone();
                                                                let project = project.clone();
                                                                let local_async = local_async.clone();

                                                                loading.set(true);

                                                                web_sys::console::log_1(&"Deleting project...".into());

                                                                spawn_local(async move {
                                                                    let message = delete_md_project(
                                                                        local_context.token.clone().expect("Failed token fetch"),
                                                                        project.id,
                                                                    )
                                                                    .await;

                                                                    local_async.refresh_projects();

                                                                    loading.set(false);
                                                                });

                                                            }
                                                        })}
                                                        disabled={*loading}
                                                        loading={*loading}
                                                        kind={MdButtonKind::SmallShort}
                                                        variant={MdButtonVariant::Negative}
                                                    />
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </section>
                            }

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
                                                let params = to_value(&CreateProjectParams { projectId: projectId.clone() }).unwrap();
                                                let result = crate::app::invoke("create_project", params).await;

                                                local_context.dispatch(LocalAction::SetCurrentProject(projectId.clone()));

                                                loading.set(false);

                                                local_context.dispatch(LocalAction::SetRoute("/concepts".to_string()));
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
                    } else {
                        <h6>{"Oh no! Please login via CommonOS File Manager."}</h6>
                    }
                </>
            } else if local_context.route == "/concepts".to_string() {
                <div class="view-row">
                    <FileBrowser variant={FileVariant::Concept} kind={FileKind::Image} files={saved_context.concepts.clone()} />
                    <section>
                        <FileViewer />
                    </section>
                </div>
            }

            <div class="view-row" style={"display: ".to_owned() + &scene_display}>
                <section>
                    <div class="btn-row">
                        <MdButton
                            label="Models"
                            icon={""}
                            on_click={Callback::from({
                                let browser_tab = browser_tab.clone();

                                move |_| {
                                    let browser_tab = browser_tab.clone();

                                    browser_tab.set("models".to_string());
                                }
                            })}
                            disabled={*loading}
                            loading={*loading}
                            kind={MdButtonKind::SmallShort}
                            variant={MdButtonVariant::Green}
                        />
                        <MdButton
                            label="Landscapes"
                            icon={""}
                            on_click={Callback::from({
                                let browser_tab = browser_tab.clone();

                                move |_| {
                                    let browser_tab = browser_tab.clone();

                                    browser_tab.set("landscapes".to_string());
                                }
                            })}
                            disabled={*loading}
                            loading={*loading}
                            kind={MdButtonKind::SmallShort}
                            variant={MdButtonVariant::Green}
                        />
                    </div>
                    if *browser_tab == "models".to_string() {
                        <FileBrowser variant={FileVariant::Asset} kind={FileKind::Model} files={saved_context.models.clone()} />
                    }
                    if *browser_tab == "landscapes".to_string() {
                        <FileBrowser variant={FileVariant::Asset} kind={FileKind::Landscape} files={saved_context.landscapes.clone().unwrap_or(Vec::new())} />
                    }
                </section>
                <section>
                    <SceneView />
                </section>
            </div>
        </section>
    }
}

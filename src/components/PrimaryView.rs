use yew::prelude::*;

use crate::components::SceneView::SceneView;
use crate::contexts::global::GlobalContextType;

#[function_component(PrimaryView)]
pub fn primary_view() -> Html {
    let global_context = use_context::<GlobalContextType>().expect("No GlobalContext found");

    let mut scene_display = "none".to_string();
    if global_context.route == "/scene".to_string() {
        scene_display = "block".to_string();
    }

    html! {
        <section>
            if global_context.route == "/".to_string() {
                <>
                    {"Welcome to Midpoint. Let's get started"}
                </>
            } else if global_context.route == "/concepts".to_string() {
                <>
                    {"Concepts"}
                </>
            }

            <div style={"display: ".to_owned() + &scene_display}>
                <SceneView />
            </div>
        </section>
    }
}

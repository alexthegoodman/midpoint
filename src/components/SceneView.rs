use yew::prelude::*;

use crate::components::MdButton::{MdButton, MdButtonKind, MdButtonVariant};
use crate::components::SceneCanvas::SceneCanvas;
use crate::contexts::local::LocalContextType;

#[function_component(SceneView)]
pub fn scene_view() -> Html {
    // let local_context = use_context::<LocalContextType>().expect("No LocalContext found");

    html! {
        <>
            <div class="toolbar">
                <MdButton
                    label="Import"
                    icon={""}
                    on_click={Callback::noop()}
                    disabled={false}
                    kind={MdButtonKind::SmallShort}
                    variant={MdButtonVariant::Green}
                />
            </div>

            <SceneCanvas />
        </>
    }
}

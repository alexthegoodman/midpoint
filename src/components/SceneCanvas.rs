use yew::prelude::*;

use crate::renderer::core::init_wgpu;

#[function_component]
pub fn SceneCanvas() -> Html {
    use_effect(move || {
        web_sys::console::log_1(&"Init SceneCanvas".into());
        wasm_bindgen_futures::spawn_local(async {
            init_wgpu().await.unwrap();
        });
        // || ()
    });

    html! {
        <div>
            <canvas id="scene-canvas" width="1000" height="600"></canvas>
        </div>
    }
}

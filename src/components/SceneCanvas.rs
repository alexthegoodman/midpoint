use yew::prelude::*;

use crate::renderer::core::handle_key_press;
// use crate::renderer::core::init_wgpu;
use crate::renderer::core::start_render_loop;

#[function_component]
pub fn SceneCanvas() -> Html {
    use_effect(move || {
        web_sys::console::log_1(&"Init SceneCanvas".into());
        wasm_bindgen_futures::spawn_local(async {
            // init_wgpu().await.unwrap();
            start_render_loop().await;
        });
        // || ()
    });

    let onkeydown = Callback::from(|event: KeyboardEvent| {
        let key = event.key();
        web_sys::console::log_1(&format!("Key pressed (1): {}", key).into());
        handle_key_press(key, true);
    });

    html! {
        <div>
            <canvas id="scene-canvas" width="1000" height="600" onkeydown={onkeydown}></canvas>
        </div>
    }
}

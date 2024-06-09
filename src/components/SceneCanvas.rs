use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::renderer::core::handle_key_press;
use crate::renderer::core::handle_mouse_move;
use crate::renderer::core::start_render_loop;

#[function_component]
pub fn SceneCanvas() -> Html {
    // gizmo tool (translate, rotate, scale)
    let gizmo = "translate";

    // camera tool (pan, zoom, rotate, orbit)
    let camera = "rotate";

    let is_dragging = Rc::new(RefCell::new(false));
    let last_mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));

    {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();

        use_effect_with((), move |_| {
            web_sys::console::log_1(&"Init SceneCanvas".into());
            wasm_bindgen_futures::spawn_local(async {
                start_render_loop().await;
            });

            let document = web_sys::window().unwrap().document().unwrap();
            let mouse_move_callback = Closure::wrap(Box::new(move |event: web_sys::PointerEvent| {
                if *is_dragging.borrow() {
                    let dx = event.client_x() as f32 - last_mouse_pos.borrow().0;
                    let dy = event.client_y() as f32 - last_mouse_pos.borrow().1;

                    web_sys::console::log_1(&format!("dx: {}, dy: {}", dx, dy).into());

                    // Call a function to rotate the camera based on dx and dy
                    handle_mouse_move(dx, dy);

                    *last_mouse_pos.borrow_mut() =
                        (event.client_x() as f32, event.client_y() as f32);
                }
            }) as Box<dyn FnMut(_)>);

            document
                .add_event_listener_with_callback(
                    "pointermove",
                    mouse_move_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            mouse_move_callback.forget();

            // TODO: mouseup on document in case the mouse is released outside the canvas

            // || ()
        });
    }

    let onmousedown = {
        let is_dragging = is_dragging.clone();
        let last_mouse_pos = last_mouse_pos.clone();

        web_sys::console::log_1(&"onmousedown (1)".into());

        Callback::from(move |event: web_sys::PointerEvent| {
            web_sys::console::log_1(&"onmousedown (2)".into());

            *is_dragging.borrow_mut() = true;
            *last_mouse_pos.borrow_mut() = (event.client_x() as f32, event.client_y() as f32);
        })
    };

    let onmouseup = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |_event: web_sys::PointerEvent| {
            *is_dragging.borrow_mut() = false;
        })
    };

    let onkeydown = Callback::from(|event: KeyboardEvent| {
        let key = event.key();
        web_sys::console::log_1(&format!("Key pressed (1): {}", key).into());
        handle_key_press(key, true);
    });

    html! {
        <div>
            <canvas
                id="scene-canvas"
                width="1000"
                height="600"
                onkeydown={onkeydown}
                onpointerdown={onmousedown}
                onpointerup={onmouseup}
            ></canvas>
        </div>
    }
}

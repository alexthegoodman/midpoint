mod app;

use app::App;

mod components;
mod contexts;
mod gql;
pub mod renderer;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

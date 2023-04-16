pub mod app;
pub mod components;
pub mod prueba;
pub mod routes;
pub mod stores;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

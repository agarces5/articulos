// use gloo_console::log;
// use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(FormAuto)]
fn autocomplete() -> Html {
    let onsubmit = { |e: SubmitEvent| e.prevent_default() };

    html! {
        <form class="p-2 mb-2 bg-light border-bottom" autocomplete="off" {onsubmit}>
            <div class="autocomplete" style="width:300px;">
                <Input />
            </div>
        </form>
    }
}

#[function_component(Input)]
fn input() -> Html {
    let input_state_handler = use_state(String::default);
    // let input_state = (*input_state_handler).clone();

    let oninput = {
        let input_state_handler = input_state_handler;

        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                input_state_handler.set(input.value());
            }
        })
    };

    html! {
        <input type="search" class="form-control" placeholder="Type to filter..." {oninput} />
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="dropdown-menu d-block position-static pt-0 mx-0 rounded-3 shadow overflow-hidden w-280px">
            <form class="p-2 mb-2 bg-light border-bottom" autocomplete="off">
            <input type="search" class="form-control" placeholder="Type to filter..."/>
            </form>
            <ul class="list-unstyled mb-0">
            <li><a class="dropdown-item d-flex align-items-center gap-2 py-2" href="#">
                <span class="d-inline-block bg-success rounded-circle p-1"></span>
                {"Action"}
                </a></li>
            <li><a class="dropdown-item d-flex align-items-center gap-2 py-2" href="#">
                <span class="d-inline-block bg-primary rounded-circle p-1"></span>
                {"Another action"}
                </a></li>
            <li><a class="dropdown-item d-flex align-items-center gap-2 py-2" href="#">
                <span class="d-inline-block bg-danger rounded-circle p-1"></span>
                {"Something else here"}
                </a></li>
            <li><a class="dropdown-item d-flex align-items-center gap-2 py-2" href="#">
                <span class="d-inline-block bg-info rounded-circle p-1"></span>
                {"Separated link"}
                </a></li>
            </ul>
            <FormAuto />
        </div>
    }
}

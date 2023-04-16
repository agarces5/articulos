use crate::components::autocomplete::HtmlAutocomplete;
use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(FormAuto)]
pub fn form_auto() -> Html {
    let suggestions = vec![
        "Manzana".to_string(),
        "Platano".to_string(),
        "Limon".to_string(),
        "SANDIA".to_string(),
        "BANANA".to_string(),
        "KIWI".to_string(),
    ];
    let input_ref = use_node_ref();

    let onsubmit = {
        let input_ref = input_ref.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let input_value = input_ref.cast::<HtmlInputElement>();
            let input_value = match input_value {
                Some(input) => input.value(),
                None => String::default(),
            };
            log!(input_value);
        }
    };

    html! {
        <form class="bg-light border-bottom" autocomplete="off" {onsubmit}>
            <HtmlAutocomplete suggestions={suggestions} input_ref={input_ref}/>
        </form>
    }
}

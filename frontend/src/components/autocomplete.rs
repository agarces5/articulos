use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct AutocompleteState {
    pub active_suggestion: usize,
    pub filtered_suggestions: Vec<String>,
    pub show_suggestions: bool,
    pub user_input: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct AutocompleteProps {
    pub suggestions: Vec<String>,
    pub input_ref: NodeRef,
}
#[function_component(HtmlAutocomplete)]
pub fn html_autocomplete(props: &AutocompleteProps) -> Html {
    let AutocompleteProps {
        suggestions,
        input_ref,
    } = props;
    let input_state = use_state(String::default);
    let onchange = {
        let input_state = input_state.clone();
        Callback::from(move |e: Event| {
            let input_element = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input_element {
                input_state.set(input.value());
            }
        })
    };
    html! {
        <>
            <input list="databases" type="search" class="form-control" placeholder="Introduce la base de datos"
            ref={input_ref} value={input_state.to_string()} {onchange}/>
            <datalist id="databases">
                {
                    suggestions.iter().map(|database| {
                        html! {
                            <option value={database.to_string()}></option>

                        }
                    }).collect::<Html>()
                }
            </datalist>
        </>
    }
}

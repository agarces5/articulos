use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};

use crate::stores::login_info::LoginInfo;

pub fn onchange_input(key: String, dispatch: Dispatch<LoginInfo>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input_value = e.target_unchecked_into::<HtmlInputElement>().value();
        let input_value = if input_value.is_empty() {
            None
        } else {
            Some(input_value)
        };
        dispatch.reduce(|login_state| match key.as_str() {
            "username" => LoginInfo {
                username: input_value,
                ..login_state.as_ref().clone()
            }
            .into(),
            "password" => LoginInfo {
                password: input_value,
                ..login_state.as_ref().clone()
            }
            .into(),
            _ => LoginInfo {
                ..login_state.as_ref().clone()
            }
            .into(),
        });
    })
}

#[derive(Debug, Properties, PartialEq, PartialOrd)]
pub struct LoginProps {
    pub _type: String,
    pub placeholder: String,
    pub _key: String,
}

#[function_component(LoginInput)]
pub fn login_input(props: &LoginProps) -> Html {
    let (_, login_dispatch) = use_store::<LoginInfo>();
    html! {
        <>
            <fieldset class="form-group">
                <input
                    class="form-control form-control-lg"
                    type={props._type.clone()}
                    placeholder={props.placeholder.clone()}
                    onchange={onchange_input(props._key.clone(), login_dispatch)}
                />
            </fieldset>
        </>
    }
}

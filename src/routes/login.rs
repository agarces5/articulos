use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::stores::login_info::LoginInfo;

#[function_component(Login)]
pub fn login_page() -> Html {
    let (store, login_dispatch) = use_store::<LoginInfo>();
    let username = format!("Usuario: {}", store.username.as_deref().unwrap_or_default());
    let password = format!(
        "Password: {}",
        store.password.as_deref().unwrap_or_default()
    );
    let is_logged = format!("Logged: {}", store.is_logged);
    let onchange_username = {
        let login_dispatch = login_dispatch.clone();
        Callback::from(move |e: Event| {
            let username = e.target_unchecked_into::<HtmlInputElement>().value();
            let username = if username.is_empty() {
                None
            } else {
                Some(username)
            };
            login_dispatch.reduce(|login_state| {
                LoginInfo {
                    username,
                    ..login_state.as_ref().clone()
                }
                .into()
            });
        })
    };
    let onchange_password = {
        let login_dispatch = login_dispatch;
        Callback::from(move |e: Event| {
            let password = e.target_unchecked_into::<HtmlInputElement>().value();
            let password = if password.is_empty() {
                None
            } else {
                Some(password)
            };
            login_dispatch.reduce(|login_state| {
                LoginInfo {
                    password,
                    ..login_state.as_ref().clone()
                }
                .into()
            })
        })
    };
    let onsubmit = {
        |e: SubmitEvent| {
            e.prevent_default();
        }
    };
    html! {
        <>
            <div class="container col-md-6 offset-md-3 col-xs-12 text-center">
                <h1 class="text-xs-center">{ "Sign In" }</h1>
                <form {onsubmit}>
                    <fieldset class="d-grid gap-3" >
                        <fieldset class="form-group">
                            <input
                                class="form-control form-control-lg"
                                type="text"
                                placeholder="Usuario"
                                onchange={onchange_username}
                                />
                        </fieldset>
                        <fieldset class="form-group">
                            <input
                                class="form-control form-control-lg"
                                type="password"
                                placeholder="Password"
                                onchange={onchange_password}
                                />
                        </fieldset>
                        <button
                            class="btn btn-lg btn-primary pull-xs-right"
                            type="submit"
                            disabled=false>
                            { "Entrar" }
                        </button>
                    </fieldset>
                </form>
            </div>
            <div class="container p-4">
                <p>{username}</p>
                <p>{password}</p>
                <p>{is_logged}</p>
            </div>
        </>
    }
}

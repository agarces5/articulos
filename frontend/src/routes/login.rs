use reqwasm::http::{self, Response};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{components::login_input::LoginInput, routes::AppRoute, stores::login_info::LoginInfo};

async fn login_req(body: &LoginInfo) -> anyhow::Result<Response> {
    let response = http::Request::post("http://localhost:8080/api/login")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(body)?)
        .send()
        .await?;
    match response.status() {
        200 => Ok(response),
        _ => Err(anyhow::Error::msg("Bad request!")),
    }
}

#[function_component(Login)]
pub fn login_page() -> Html {
    let (_, login_dispatch) = use_store::<LoginInfo>();
    let onsubmit = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let navigator = navigator.clone();
            let login_dispatch = login_dispatch.clone();
            let login_data = login_dispatch.get();
            spawn_local(async move {
                let response = login_req(&login_data).await;
                match response {
                    Ok(_) => {
                        login_dispatch.reduce(|login_state| {
                            LoginInfo {
                                is_logged: true,
                                ..login_state.as_ref().clone()
                            }
                            .into()
                        });
                        navigator.push(&AppRoute::DatabaseChooser)
                    }
                    Err(_) => gloo_dialogs::alert("Usuario invalido!"),
                }
            });
        })
    };
    html! {
        <>
            <div class="container col-md-6 offset-md-3 col-xs-12 text-center">
                <h1 class="text-xs-center">{ "Sign In" }</h1>
                <form {onsubmit}>
                    <fieldset class="d-grid gap-3" >
                        <LoginInput
                            _type={String::from("text")}
                            placeholder={String::from("Usuario")}
                            _key={String::from("username")}
                        />
                        <LoginInput
                        _type={String::from("password")}
                        placeholder={String::from("Password")}
                        _key={String::from("password")}
                        />
                        <button
                            class="btn btn-lg btn-primary pull-xs-right"
                            type="submit"
                            disabled=false>
                            { "Entrar" }
                        </button>
                    </fieldset>
                </form>
            </div>
        </>
    }
}

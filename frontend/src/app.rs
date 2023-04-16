use yew::prelude::*;
use yew_bootstrap::util::{include_cdn, include_cdn_js};
use yew_router::prelude::*;

use crate::{
    components::header::Header,
    routes::{switch, AppRoute},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            {include_cdn()}
            <HashRouter>
                <Header />
                <Switch<AppRoute> render={switch} />
            </HashRouter>
            // <div class="min-vh-100 min-vw-100 container-fluid d-flex justify-content-center align-items-center">
            //     <div class="dropdown-menu d-flex m-auto rounded-3 shadow overflow-hidden text-center">
            //         <FormAuto />
            //     </div>
            // </div>
            {include_cdn_js()}
        </>
    }
}

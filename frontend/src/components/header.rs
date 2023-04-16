use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { "Home" }
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Login} classes="navbar-brand">
                    { "Login" }
                </Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::DatabaseChooser} classes="navbar-brand">
                    { "DatabaseChooser" }
                </Link<AppRoute>>

                // {
                //     if user_ctx.is_authenticated() {
                //         logged_in_view((*user_ctx).clone())
                //     } else {
                //         logged_out_view()
                //     }
                // }
            </div>
        </nav>
    }
}

fn _logged_out_view() -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                    { "Sign in" }
                </Link<AppRoute>>
            </li>
            // <li class="nav-item">
            //     <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
            //         { "Sign up" }
            //     </Link<AppRoute>>
            // </li>
        </ul>
    }
}

fn _logged_in_view() -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}

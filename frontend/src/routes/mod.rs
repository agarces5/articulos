pub mod database_chooser;
pub mod home;
pub mod login;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{database_chooser::DatabaseChooser, home::Home, login::Login};

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/database")]
    DatabaseChooser,
    // #[at("/editor/:slug")]
    // Editor { slug: String },
    // #[at("/editor")]
    // EditorCreate,
    // #[at("/article/:slug")]
    // Article { slug: String },
    // #[at("/settings")]
    // Settings,
    // #[at("/:username/favorites")]
    // ProfileFavorites { username: String },
    // #[at("/:username")]
    // Profile { username: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::DatabaseChooser => html! {<DatabaseChooser />},
        AppRoute::Home => html! {<Home />},
        // AppRoute::Editor { slug } => html! {<Editor slug={Some(slug)}/>},
        // AppRoute::EditorCreate => html! {<Editor />},
        // AppRoute::Article { slug } => html! {<Article slug={slug} />},
        // AppRoute::Settings => html! {<Settings />},
        // AppRoute::ProfileFavorites { username } => html! {
        //     <Profile username={username} tab={ProfileTab::FavoritedBy} />
        // },
        // AppRoute::Profile { username } => html! {
        //     <Profile username={username} tab={ProfileTab::ByAuthor} />
        // },
        AppRoute::NotFound => html! { "Page not found" },
    }
}

#![allow(unused_variables)]
#![allow(dead_code)]
use leptos::prelude::*;
use leptos_router::{
    components::{
        Router, 
        Route, 
        Routes
    },
    params::Params,
    hooks::{
        use_query
    }
};

#[derive(Params,PartialEq, Clone)]
pub struct FourUsers {
    A: Option<String>,
    B: Option<String>,
    C: Option<String>,
    D: Option<String>
}

fn main() {
    mount_to_body(|| view! { <App/> });
}

#[component]
pub fn App() -> impl IntoView {
    let players_from_url = use_query::<FourUsers>();
    let untracked_players_from_url = players_from_url.get_untracked();
    view! {<Router>
        <nav>
        </nav>
        <main>
        </main>
    </Router>}.into_any()
}
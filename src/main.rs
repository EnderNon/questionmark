#![allow(unused_variables)]
#![allow(dead_code)]
use leptos::prelude::*;
use leptos_router::{
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
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}

#[component]
pub fn App() -> impl IntoView {
    let players_from_url = use_query::<FourUsers>();
    let untracked_players_from_url = players_from_url.get_untracked().unwrap_or(
        FourUsers { A: Some("Player1".to_owned()), B: Some("Player2".to_owned()), C: Some("Player3".to_owned()), D: Some("Player4".to_owned()) }
    );
    let a = RwSignal::new(untracked_players_from_url.A);
    let b = RwSignal::new(untracked_players_from_url.B);
    let c = RwSignal::new(untracked_players_from_url.C);
    let d = RwSignal::new(untracked_players_from_url.D);
    view! {
        <p>"fr"</p>
    }.into_any()
}
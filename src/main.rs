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
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
    d: Option<String>
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { 
        <Router>
            <App/> 
        </Router>
    });
}

#[component]
pub fn App() -> impl IntoView {
    let players_from_url = use_query::<FourUsers>();
    let untracked_players_from_url = players_from_url.get_untracked().unwrap_or(
        FourUsers { a: Some("Player1".to_owned()), b: Some("Player2".to_owned()), c: Some("Player3".to_owned()), d: Some("Player4".to_owned()) }
    );
    // yes, four signals are very necessary
    let a = RwSignal::new(untracked_players_from_url.a.unwrap_or("???".to_owned()));
    let b = RwSignal::new(untracked_players_from_url.b.unwrap_or("???".to_owned()));
    let c = RwSignal::new(untracked_players_from_url.c.unwrap_or("???".to_owned()));
    let d = RwSignal::new(untracked_players_from_url.d.unwrap_or("???".to_owned()));
    view! {
        
        <p>"User A: " {a}</p>
        <p>"User B: " {b}</p>
        <p>"User C: " {c}</p>
        <p>"User D: " {d}</p>
        
    }.into_any()
}
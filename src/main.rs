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
    // get from url bar once
    let players_from_url = use_query::<FourUsers>();
    // theres a reason I made untracked a seperate variable I promise
    let untracked_players_from_url = players_from_url.get_untracked().unwrap_or(
        FourUsers { a: Some("Player1".to_owned()), b: Some("Player2".to_owned()), c: Some("Player3".to_owned()), d: Some("Player4".to_owned()) }
    );
    // yes, four signals are very necessary
    let a = RwSignal::new(untracked_players_from_url.a.unwrap_or("???".to_owned()));
    let b = RwSignal::new(untracked_players_from_url.b.unwrap_or("???".to_owned()));
    let c = RwSignal::new(untracked_players_from_url.c.unwrap_or("???".to_owned()));
    let d = RwSignal::new(untracked_players_from_url.d.unwrap_or("???".to_owned()));
    view! {
        <h1> QuestionMark Quest - Maze puzzle</h1>
        <p>{d} " walks through RED door and stands on PURPLE"</p>
        <p>{a} " walks through PURPLE door, on the right, and stands on GREEN"</p>
        <p>{d} " stands on BLUE"</p>
        <p>{b} " and " {c} " goes through the BLUE door, on the left"</p>
        <p>{b} " stands on GREEN"</p>
        <p>{c} " goes through GREEN door and stands on  PINK"</p>
        <p>{b} " goes to starting room and stands on ORANGE"</p>
        <p>{d} " goes through the ORANGE door, on the left, and stands on PINK"</p>
        <p>{a} " goes through the PINK door and stands on LIGHT BLUE"</p>
        <p>{c} " and " {d} " goes through the ORANGE door and " {c} " stands on LIGHT BLUE"</p>
        <p>{d} " stands on CYAN (GRAY)"</p>
        <p>{b} " walks through LIGHT BLUE door, in the top left corner, and stands on CYAN (GRAY) "</p>
        <p>{a} " stands on CYAN (GRAY) "</p>
        <p>{c} " goes through the CYAN (GRAY) door and stands on YELLOW"</p>
        <p>{a} " and " {d} " stand on LIGHT BLUE"</p>
        <p>{b} " walks to starting room"</p>
        <p>{a} " stands on BROWN"</p>
        <p>{b} " walks to the end"</p>
        <p>"Everyone walks to the end"</p>
        
    }.into_any()
}
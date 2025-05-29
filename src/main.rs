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
        
        <p>"A: "{a} ", B: "{b} ", C: " {c} ", D: " {d}</p>
        <br/>
        // from replaced.txt do this.
        // Using regex, add proper brackets for variables:
        // - Find \$(.)
        // - Repl {\l$1}
        // <pre> bracket is to show the text, exactly as is.
        <pre>
        {d} walks through RED door and stands on PURPLE
        {a} walks through PURPLE door, on the right, and stands on GREEN
        {d} stands on BLUE
        {b} and {c} goes through the BLUE door, on the left
        {b} stands on GREEN
        {c} goes through GREEN door and stands on  PINK
        {b} goes to starting room and stands on ORANGE
        {d} goes through the ORANGE door, on the left, and stands on PINK
        {a} goes through the PINK door and stands on LIGHT BLUE
        {c} and {d} goes through the ORANGE door and {c} stands on LIGHT BLUE
        {d} stands on CYAN (GRAY)
        {b} walks through LIGHT BLUE door, in the top left corner, and stands on CYAN (GRAY) 
        {a} stands on CYAN (GRAY) 
        {c} goes through the CYAN (GRAY) door and stands on YELLOW
        {a} and {d} stand on LIGHT BLUE
        {b} walks to starting room
        {a} stands on BROWN
        {b} walks to the end
        Everyone walks to the end
        </pre>
        
    }.into_any()
}
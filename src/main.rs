use leptos::prelude::*;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
pub struct FourUsers {
    A: Option<String>,
    B: Option<String>,
    C: Option<String>,
    D: Option<String>
}

fn main() {
    println!("Hello, world!");
}

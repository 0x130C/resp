#![allow(dead_code)]
#![recursion_limit="128"]
extern crate stdweb;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate pulldown_cmark;

mod root;
mod routes;
mod services;
mod components;
mod _utils;

pub use root::RootComponent;

pub mod utils {
    pub use super::_utils::render_markdown;
}



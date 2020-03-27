#[macro_use]
extern crate lazy_static;

#[macro_use]
mod util;
mod boilerplate;
mod client;
mod config;
mod html;
mod line;
mod metadata;
mod spec;

fn main() {
    client::run();
}

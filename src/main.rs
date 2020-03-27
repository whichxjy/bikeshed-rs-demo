#[macro_use]
extern crate lazy_static;

#[macro_use]
mod util;
mod client;
mod boilerplate;
mod config;
mod html;
mod line;
mod metadata;
mod spec;

fn main() {
    client::run();
}

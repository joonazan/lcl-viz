#![recursion_limit = "1024"]
#![feature(iter_intersperse)]
mod necklace;
mod no_ids;
mod utils;

pub fn main() {
    yew::start_app::<necklace::Necklace>();
}

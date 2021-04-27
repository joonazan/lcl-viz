#![recursion_limit = "1024"]
#![feature(iter_intersperse)]
mod necklace;

pub fn main() {
    yew::start_app::<necklace::Necklace>();
}

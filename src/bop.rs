extern crate clap;
use clap::{Arg, App};

mod arguments;

fn main() {
    let matches = arguments::parse().get_matches();
}

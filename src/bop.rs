#[macro_use]
extern crate clap;

use std::io;
use clap::Shell;

mod arguments;

fn main() {
    let matches = arguments::parse().get_matches();

    match matches.subcommand() {
        ("completions", Some(m)) => {
            let sh = m.value_of("SHELL").unwrap();
            arguments::parse()
                .gen_completions_to(
                    "bop",
                    sh.parse::<Shell>().unwrap(),
                    &mut io::stdout());
        }
        (_, _) => unimplemented!(),
    }
}

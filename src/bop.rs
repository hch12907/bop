extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("bop")
                    .version("0.1")
                    .author("zypeh <zypeh.geek@gmail.com>")
                    .about("Experimental editor infrastructure")
                    .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("TOML FILE")
                               .help("Sets a custom config file")
                               .takes_value(true))
                    .get_matches();

    //
}
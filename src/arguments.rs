use clap::{App, Arg};

pub fn parse() -> App<'static, 'static> {
    App::new("Bop")
        // version
        .version("0.1")
        // author information
        .author("zypeh <zypeh.geek@gmail.com>")
        // description
        .about("Experimental editor infrastructure")

        // test
        .arg(Arg::with_name("test")
             .long("test")
             .short("t")
             .help("Prints the version number and license"))
}
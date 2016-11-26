use clap::{App, Arg, SubCommand};

pub fn parse() -> App<'static, 'static> {
    App::new("Bop")
        // version
        .version(crate_version!())
        // author information
        .author(crate_authors!())
        // description
        .about("Experimental editor infrastructure")

        // test
        .arg(Arg::with_name("test")
            .long("test")
            .short("t")
            .help("Prints the version number and license"))
        
        // generate the completion file for terminal
        .subcommand(SubCommand::with_name("completions")
            .about("Generates completion scripts for your shell")
            .arg(Arg::with_name("SHELL")
                .required(true)
                .possible_values(&["bash", "fish", "zsh"])
                .help("The shell to geenerate the script for")))
}
extern crate aoc2018;
extern crate failure;
#[macro_use]
extern crate quicli;

use aoc2018::one;
use failure::bail;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
    day: String,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| match args.day.as_ref() {
    "1" => println!("{}", one::run()?),
    _ => bail!(
        "pass the day to run as the first argument, ie, 1. Not '{}'",
        args.day
    ),
});

use std::process;
use structopt::StructOpt;

use myparse;

fn main() {
    let opt = myparse::Opt::from_args();
    process::exit(myparse::run(&opt));
}

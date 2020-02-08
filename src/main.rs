#![forbid(unsafe_code)]

mod input;
mod output;

use crate::input::cli::cli_args::CliArgs;
use crate::output::write_latex::WriteLatex;

use structopt::StructOpt;

fn main() {
    let args = CliArgs::from_args();
    WriteLatex::write(args.book_dir());
}

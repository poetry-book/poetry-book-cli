use std::path::{Path, PathBuf};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CliArgs {
    /// The path of the book directory
    #[structopt(parse(from_os_str))]
    book_dir: PathBuf,
}

impl CliArgs {
    pub fn book_dir(&self) -> &Path {
        &self.book_dir
    }
}

use std::path::PathBuf;

use crate::input::files::file_utils::FileUtils;
use poetry_book::Preface;

pub struct PrefaceBuilder {
    file_path: PathBuf,
}

impl PrefaceBuilder {
    pub fn new(file_path: PathBuf) -> PrefaceBuilder {
        PrefaceBuilder { file_path }
    }

    pub fn build(&self) -> Preface {
        let body = FileUtils::read_required_file(&self.file_path);
        let title = FileUtils::get_filename(&self.file_path);
        Preface {
            title: title.to_string(),
            body,
        }
    }
}

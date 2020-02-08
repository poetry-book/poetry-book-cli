use std::fs;
use std::path::Path;

pub struct FileUtils;

impl FileUtils {
    pub fn get_filename(file_path: &Path) -> &str {
        file_path.file_stem().unwrap().to_str().unwrap()
    }

    pub fn read_required_file(path: &Path) -> String {
        let error_message = format!("cannot read {}", path.to_str().unwrap());
        fs::read_to_string(path).expect(&error_message)
    }
}

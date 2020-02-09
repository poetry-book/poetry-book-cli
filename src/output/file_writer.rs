use std::path::Path;
use std::path::PathBuf;
use std::fs;

/// Write book to file
pub struct FileWriter {
    book_content: String,
    book_dir_path: PathBuf,
}

impl FileWriter {
    pub fn new(book_content: String, book_dir_path: &Path) -> FileWriter {
        FileWriter { book_content, book_dir_path: book_dir_path.to_path_buf() }
    }

    pub fn write(self) {
        let output_dir = self.book_dir_path.join("out/");
        if !output_dir.exists() {
            let error_message = format!(
                "unable to create {} directory",
                output_dir.to_str().unwrap()
            );
            fs::create_dir(&output_dir).expect(&error_message);
        }
        let output_file = output_dir.join("book.tex");
        fs::write(output_file, self.book_content).expect("Unable to write book to file");
    }
}

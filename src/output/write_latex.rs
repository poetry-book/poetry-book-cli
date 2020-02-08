use crate::input::files::book_dir::BookDir;
use crate::output::file_writer::FileWriter;

use poetry_book::Latex;
use std::path::Path;

pub struct WriteLatex;

impl WriteLatex {
    /// Take the directory path that contains all the files of the book and
    /// write the resulting latex in a file.
    pub fn write(book_dir_path: &Path) {
        let book_dir = BookDir::new(book_dir_path);
        let book = book_dir.build();
        let book_content = book.latex();
        let fs_presenter = FileWriter::new(book_content, book_dir_path);
        fs_presenter.write();
    }
}

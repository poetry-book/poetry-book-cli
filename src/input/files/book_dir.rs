use std::path::{Path, PathBuf};

use crate::input::files::{
    config, config::Config, file_utils::FileUtils, preface_builder::PrefaceBuilder,
};
use poetry_book::{Book, BookAttributes, BookAttributesBuilder, BookBuilder, Poem, Preface};

pub struct BookDir {
    path: PathBuf,
}

impl BookDir {
    pub fn new(path: &Path) -> BookDir {
        BookDir {
            path: PathBuf::from(path),
        }
    }

    pub fn build(&self) -> Book {
        let config: Config = self.read_book_config();

        let mut book_attributes_builder =
            BookAttributesBuilder::new(config.author(), config.title());

        if let Some(language) = config.language() {
            book_attributes_builder = book_attributes_builder.language(language);
        }

        if let Some(toc_title) = config.toc_title() {
            book_attributes_builder = book_attributes_builder.toc_title(toc_title);
        }

        let book_attributes: BookAttributes = book_attributes_builder.finish();

        let preface = self.build_preface(config.preface());

        let poems_titles: Vec<PathBuf> = config.poems().iter().map(PathBuf::from).collect();

        let poems: Vec<Poem> = self.build_poems(&poems_titles);

        let mut book_builder = BookBuilder::new(book_attributes, poems);
        if let Some(pref) = preface {
            book_builder = book_builder.preface(pref);
        }

        if let Some(poem_formatting) = config.poem_formatting() {
            book_builder = book_builder.poem_formatting(poem_formatting.get());
        }

        book_builder.finish()
    }

    fn read_book_config(&self) -> Config {
        let book_config_path = self.path.join(config::CONFIG_FILE);

        let book_config: String = FileUtils::read_required_file(&book_config_path);

        Config::new(&book_config)
    }

    fn build_poems(&self, titles: &[PathBuf]) -> Vec<Poem> {
        titles
            .iter()
            .map(|title| {
                let poem_path: PathBuf = self.path.join("poems/").join(title);
                let titles = FileUtils::get_filename(title);
                let poem_body: String = FileUtils::read_required_file(&poem_path);
                Poem::new(titles, &poem_body)
            })
            .collect()
    }

    fn build_preface(&self, preface_filename: Option<&str>) -> Option<Preface> {
        match preface_filename {
            Some(pref) => {
                let preface_path = self.path.join(PathBuf::from(pref));
                let preface_builder = PrefaceBuilder::new(preface_path);
                let built_preface = preface_builder.build();
                Some(built_preface)
            }
            None => None,
        }
    }
}

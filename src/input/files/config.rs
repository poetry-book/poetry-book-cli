use poetry_book::PoemFormatting;
use poetry_book::CenteredVerse;
use serde::Deserialize;

pub const CONFIG_FILE: &str = "book.json";

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    author: String,
    title: String,
    toc_title: Option<String>,
    preface: Option<String>,
    poem_formatting: Option<PoemFormattingJson>,
    poems: Vec<String>,
    language: Option<String>,
}

impl Config {
    pub fn new(json: &str) -> Config {
        let error_string = format!("{} bad formatted", CONFIG_FILE);
        serde_json::from_str(json).expect(&error_string)
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    /// book title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// poems filenames
    pub fn poems(&self) -> &Vec<String> {
        &self.poems
    }

    /// table of contents title
    pub fn toc_title(&self) -> Option<&str> {
        self.toc_title.as_deref()
    }

    pub fn poem_formatting(&self) -> Option<PoemFormattingJson> {
        self.poem_formatting
    }

    /// preface filename
    pub fn preface(&self) -> Option<&str> {
        self.preface.as_deref()
    }

    /// English, Italian, and so on..
    pub fn language(&self) -> Option<&str> {
        self.preface.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_with_optional_fields() {
        let expected_config = Config {
            author: "Marco".to_string(),
            title: "my book".to_string(),
            toc_title: Some("Index".to_string()),
            preface: Some("Preface".to_string()),
            language: None,
            poem_formatting: Some(PoemFormattingJson {
                centered_verse: CenteredVerseJson::Average
            }),
            poems: vec!["poem_a".to_string(), "poem_b".to_string()],
        };

        let config_json = r#"
            {
                "author": "Marco",
                "title": "my book",
                "toc_title": "Index",
                "preface": "Preface",
                "poem_formatting": {
                    "centered_verse": "average"
                },
                "poems": [
                    "poem_a",
                    "poem_b"
                ]
            }
        "#;

        let actual_config = Config::new(config_json);
        assert_eq!(expected_config, actual_config);
    }

    #[test]
    fn config_without_optional_fiels() {
        let expected_config = Config {
            author: "Marco".to_string(),
            title: "my book".to_string(),
            toc_title: None,
            preface: None,
            language: None,
            poem_formatting: None,
            poems: vec!["poem_a".to_string(), "poem_b".to_string()],
        };

        let config_json = r#"
            {
                "author": "Marco",
                "title": "my book",
                "poems": [
                    "poem_a",
                    "poem_b"
                ]
            }
        "#;

        let actual_config = Config::new(config_json);
        assert_eq!(expected_config, actual_config);
    }
}

#[derive(Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct PoemFormattingJson {
    centered_verse: CenteredVerseJson
}

impl PoemFormattingJson {
    pub fn get(&self) -> PoemFormatting {
        let centered_verse = match self.centered_verse {
            CenteredVerseJson::Average => CenteredVerse::Average,
            CenteredVerseJson::Longest => CenteredVerse::Longest,
        };
        PoemFormatting::new(centered_verse)
    }
}

#[derive(Deserialize, PartialEq, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CenteredVerseJson {
    Average,
    Longest
}

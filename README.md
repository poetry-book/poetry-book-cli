# Poetry book cli

[![Crates.io](https://img.shields.io/crates/v/poetry-book-cli.svg)](https://crates.io/crates/poetry-book-cli)
[![CI](https://github.com/poetry-book/poetry-book-cli/workflows/Rust/badge.svg)](https://github.com/poetry-book/poetry-book-cli/actions)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

[Poetry-book](https://github.com/poetry-book/poetry-book) Command line interface.

Create a latex poetry book starting from poems written in plain text files.

## Install

```
$ cargo install poetry-book-cli
```

## Usage

```
USAGE:
    poetry-book-cli <book-dir>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <book-dir>    The path of the book directory
```

## Directory

The directory structure that contains the input of the program looks like this:

```
.
├── book.json
├── poems/
│   ├── A Silvia
│   ├── La ginestra.txt
│   └── L'infinito
└── Preface.txt
```

### book.json

`book.json` file has the following format:

```json
{
    "author": "Giacomo Leopardi",
    "title": "Leopardi's poetry book",
    "toc_title": "Table of Contents",
    "language": "Italian",
    "preface": "Preface.txt",
    "poem_formatting": {
        "centered_verse": "average"
    },
    "poems": [
        "La ginestra.txt",
        "L'infinito",
        "A Silvia"
    ]
}
```

* author: book author
* title: book title
* toc_title: table of contents title (optional)
* language: language of the poems (optional)
* preface: filename of the preface (optional). The preface (without extension) is used as preface title.
* poem_formatting: how poems should be formatted (optional)
  * centered_verse: Determine which verse will be centered. There are two options:
    * longest: The verse with the longest length (default)
    * average: A verse with a number of 'x' equal to the average length among all verses
* poems: ordered list of poems filename. The filename (without extension) is used as poem title.

### Poems

Poems are just plain text files, with an empty line between stanzas.
The name of the file (without extension) is the poem title, the content of the file is the poem body.

### Preface

Preface is just a plain text file, with an empty line between paragraphs.
The name of the file (without extension) is the preface title, the content of the file is the preface body.

## Output

When you run the executable, the output is places in the `out/` directory.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

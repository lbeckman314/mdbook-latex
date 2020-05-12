extern crate md2tex;
extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tectonic;

use md2tex::markdown_to_tex;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use pulldown_cmark::{Event, Options, Parser, Tag};
use std::path::PathBuf;

// config definition.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    // Chapters that will not be exported.
    pub ignores: Vec<String>,

    // Output latex file.
    pub latex: bool,

    // Output PDF.
    pub pdf: bool,

    // Output markdown file.
    pub markdown: bool,

    // Use user's LaTeX template file instead of default (template.tex).
    pub custom_template: Option<String>,
}

fn main() -> std::io::Result<()> {
    let mut stdin = io::stdin();

    // Get markdown source.
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    // Get configuration options from book.toml.
    let cfg: Config = ctx.config
                          .get_deserialized("output.latex")
                          .unwrap_or_default();
    Ok(())
}


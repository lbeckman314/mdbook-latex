extern crate mdbook;

use failure::{Error, SyncFailure};
use mdbook::MDBook;
use mdbook::renderer::RenderContext;
use std::env;
use std::io;
use std::path::Path;

pub fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let book = &args[1];

    let md = MDBook::load(book).unwrap();
    let dir = env::current_dir()?;

    let ctx = RenderContext::new(
        md.root.clone(),
        md.book.clone(),
        md.config.clone(),
        dir);

    mdbook_latex::generate(&ctx).unwrap();

    Ok(())
}


extern crate failure;
extern crate mdbook;
extern crate mdbook_latex;
extern crate serde_derive;
extern crate tempdir;

use failure::{Error, SyncFailure};
use mdbook::MDBook;
use mdbook::renderer::RenderContext;
use std::path::Path;
use tempdir::TempDir;

#[test]
fn output_files_exists() {
    let (ctx, _md, tmp) = create_book().unwrap();
    let output_file_tex = mdbook_latex::output_filename(tmp.path(), &ctx.config, "tex");
    let output_file_pdf = mdbook_latex::output_filename(tmp.path(), &ctx.config, "pdf");

    assert!(!output_file_tex.exists());
    assert!(!output_file_pdf.exists());
    mdbook_latex::generate(&ctx).unwrap();
    assert!(output_file_tex.exists());
    assert!(output_file_pdf.exists());
}

fn create_book() -> Result<(RenderContext, MDBook, TempDir), Error> {
    let tmp = TempDir::new("mdbook-latex")?;
    let test_book = Path::new(env!("CARGO_MANIFEST_DIR"))
                             .join("tests")
                             .join("test_book");

    let md = MDBook::load(test_book).map_err(SyncFailure::new)?;

    let ctx = RenderContext::new(
        md.root.clone(),
        md.book.clone(),
        md.config.clone(),
        tmp.path().to_path_buf());

    Ok((ctx, md, tmp))
}


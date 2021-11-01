extern crate mdbook;
extern crate mdbook_latex;
extern crate serde_derive;
extern crate tempdir;

use anyhow::Result;
use mdbook::renderer::RenderContext;
use mdbook::MDBook;
use std::path::Path;
use tempdir::TempDir;

fn output_files_exists(path: &str) -> bool {
    let (ctx, _md, tmp) = create_book(path).unwrap();
    let output_file_tex = mdbook_latex::output_filename(tmp.path(), &ctx.config, "tex");
    let output_file_pdf = mdbook_latex::output_filename(tmp.path(), &ctx.config, "pdf");

    if output_file_tex.exists() || output_file_pdf.exists() {
        return false;
    }

    mdbook_latex::generate(&ctx).unwrap();

    if !output_file_tex.exists() || !output_file_pdf.exists() {
        return false;
    }

    true
}

fn create_book(path: &str) -> Result<(RenderContext, MDBook, TempDir)> {
    let tmp = TempDir::new("mdbook-latex")?;
    let test_book = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);

    let md = MDBook::load(test_book)?;

    let ctx = RenderContext::new(
        md.root.clone(),
        md.book.clone(),
        md.config.clone(),
        tmp.path().to_path_buf(),
    );

    Ok((ctx, md, tmp))
}

#[test]
fn build_books() {
    assert!(output_files_exists("test/test_book"));
    assert!(output_files_exists("docs"));
}

use std::io;
use mdbook::renderer::RenderContext;

pub fn main() -> std::io::Result<()> {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    mdbook_latex::generate(ctx);
    Ok(())
}


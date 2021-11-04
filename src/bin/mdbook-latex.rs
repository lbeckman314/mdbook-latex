extern crate env_logger;
extern crate mdbook;
extern crate mdbook_latex;
extern crate serde_json;
extern crate structopt;

use anyhow::{Context, Result};
use mdbook::renderer::RenderContext;
use mdbook::MDBook;
use std::io;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

pub fn main() {
    env_logger::init();
    let args = Args::from_args();

    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);

        for cause in e.chain() {
            eprintln!("\t Caused By: {}", cause);
        }

        #[cfg(any(backtrace, feature = "backtrace"))]
        if std::env::var("RUST_BACKTRACE").is_ok() {
            eprintln!();
            eprintln!("{}", e.backtrace());
        }

        process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    let ctx: RenderContext = if args.standalone {
        let md = MDBook::load(&args.root)?;
        let dest = md.build_dir_for("latex");

        RenderContext::new(md.root, md.book, md.config, dest)
    } else {
        serde_json::from_reader(io::stdin()).context("Unable to parse RenderContext")?
    };

    mdbook_latex::generate(&ctx)?;

    Ok(())
}

#[derive(Debug, Clone, StructOpt)]
struct Args {
    #[structopt(
        short = "s",
        long = "standalone",
        help = "Run standalone (i.e. not as a mdbook plugin)"
    )]
    standalone: bool,
    #[structopt(help = "The book to render.", parse(from_os_str), default_value = ".")]
    root: PathBuf,
}

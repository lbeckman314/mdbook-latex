// latex \and operator for multiple authors
//   https://tex.stackexchange.com/questions/4805/whats-the-correct-use-of-author-when-multiple-authors

//extern crate latex;
extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate md2tex;
extern crate tectonic;

//use latex::*;
use md2tex::markdown_to_latex;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use walkdir::WalkDir;

// config definition.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct LatexConfig {
    // chapters that will not be exported.
    pub ignores: Vec<String>,
    pub latex: bool,
    pub pdf: bool,
}

pub const LATEX_BEGIN: &str = r#"
\begin{document}
\maketitle
\clearpage
\tableofcontents
\clearpage
"#;

pub const LATEX_FOOTER: &str = "\n\\end{document}\n";

fn main() -> std::io::Result<()> {

    //for entry in WalkDir::new("../../src") {
        //let entry = entry.unwrap();
        //println!("{}", entry.path().display());
    //}

    //Ok(())

    let mut stdin = io::stdin();

    // get markdown source.
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    // get configuration options.
    let cfg: LatexConfig = ctx
        .config
        .get_deserialized("output.latex")
        .unwrap_or_default();

    // read book's config values (title, authors).
    let title = ctx.config.book.title.unwrap();
    let authors = ctx.config.book.authors.join(" \\and ");

    let mut latex = String::new();

    let latex_header = include_str!("header.tex");
    let latex_languages = include_str!("languages.tex");

    latex.push_str(&latex_header);
    latex.push_str(&latex_languages);
    latex.push_str(&format!("\\title{{{title}}}", title = title));
    latex.push_str(&format!("\\author{{{authors}}}", authors = authors));

    latex.push_str(&LATEX_BEGIN);

    // iterate through markdown source.
    let mut content = String::new();
    for item in ctx.book.iter() {
        // iterate through each chapter.
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            content.push_str(&ch.content);
            //content.push_str("\n");
            //content.push_str("\n");
            latex.push_str(&markdown_to_latex(ch.content.to_string()));
        }
    }

    // output markdown file.
    let mut file_md = title.clone();
    file_md.push_str(".md");
    let path = Path::new(&file_md);
    let display = path.display();
    let mut _file = match File::create(&file_md) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // write to file.
    match _file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    //latex.push_str(&markdown_to_latex(content.to_string()));

    latex.push_str(&LATEX_FOOTER);

    // output latex file.
    if cfg.latex {
        let mut file_latex = title.clone();
        file_latex.push_str(".tex");
        let path = Path::new(&file_latex);
        let display = path.display();

        // create output directory/file.
        let _ = fs::create_dir_all(&ctx.destination);

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // write to file.
        match file.write_all(latex.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // output PDF file.
    if cfg.pdf {
        // write PDF with tectonic.
        let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
        println!("Output PDF size is {} bytes", pdf_data.len());

        let mut pos = 0;

        let mut file_pdf = title.clone();
        file_pdf.push_str(".pdf");
        let mut buffer = File::create(&file_pdf)?;

        while pos < pdf_data.len() {
            let bytes_written = buffer.write(&pdf_data[pos..])?;
            pos += bytes_written;
        }
    }

    Ok(())
}

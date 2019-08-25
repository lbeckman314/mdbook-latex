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
use md2tex::markdown_to_tex;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// config definition.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct LatexConfig {
    // chapters that will not be exported.
    pub ignores: Vec<String>,

    // output latex file.
    pub latex: bool,

    // output PDF.
    pub pdf: bool,

    // output markdown file.
    pub markdown: bool,

    // TODO use user's LaTeX template file instead of default (template.tex).
    // pub custom_template: bool,
}

fn main() -> std::io::Result<()> {
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

    // copy template data into memory.
    let mut template = include_str!("template.tex").to_string();

    let mut latex = String::new();

    // iterate through markdown source.
    let mut content = String::new();
    for item in ctx.book.iter() {
        // iterate through each chapter.
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            content.push_str(&ch.content);
        }
    }

    if cfg.markdown {
        // output markdown file.
        output(".md".to_string(), title.clone(), &latex, &ctx.destination);
    }

    if cfg.latex || cfg.pdf {
        // convert markdown data to LaTeX
        latex.push_str(&markdown_to_tex(content.to_string()));

        // insert new LaTeX data into template after "%% mdbook-latex begin".
        let begin = "mdbook-latex begin";
        let pos = template.find(&begin).unwrap() + begin.len();
        template.insert_str(pos, &latex);
    }

    if cfg.latex {
        // output latex file.
        output(".tex".to_string(), title.clone(), &template, &ctx.destination);
    }

    // output PDF file.
    if cfg.pdf {
        // write PDF with tectonic.
        println!("Writing PDF with Tectonic...");
        let pdf_data: Vec<u8> = tectonic::latex_to_pdf(&template).expect("processing failed");
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

/// Output plain text file.
///
/// Used for writing markdown and latex data to files.
fn output<P: AsRef<Path>>(extension: String, mut filename: String, data: &String, destination: P) {
    filename.push_str(&extension);
    let path = Path::new(&filename);
    let display = path.display();

    // create output directory/file.
    let _ = fs::create_dir_all(destination);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // write to file.
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

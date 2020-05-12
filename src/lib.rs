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

pub fn generate(ctx: &RenderContext) -> std::io::Result<()> {
    // Get configuration options from book.toml.
    let cfg: Config = ctx.config
                         .get_deserialized("output.latex")
                         .unwrap_or_default();

    // Read book's config values (title, authors).
    let title = ctx.config.book.title.unwrap();
    let authors = ctx.config.book.authors.join(" \\and ");

    // Copy template data into memory.
    let mut template = if let Some(custom_template) = cfg.custom_template {
            let mut custom_template_path = ctx.root;
            custom_template_path.push(custom_template);
            std::fs::read_to_string(custom_template_path)?
        } else {
            include_str!("template.tex").to_string()
        };

    // Add title and author information.
    template = template.replace(r"\title{}", &format!("\\title{{{}}}", title));
    template = template.replace(r"\author{}", &format!("\\author{{{}}}", authors));

    let mut latex = String::new();

    // Iterate through markdown source.
    let mut content = String::new();
    for item in ctx.book.iter() {
        // Iterate through each chapter.
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            // Add chapter path to relative links.
            content.push_str(&path_adder(&ch.content, &ch.path.parent()));
        }
    }

    if cfg.markdown {
        // Output markdown file.
        output(".md".to_string(), title.clone(), &content, &ctx.destination);
    }

    if cfg.latex || cfg.pdf {
        // Insert new LaTeX data into template after "%% mdbook-latex begin".
        let begin = "mdbook-latex begin";
        let pos = template.find(&begin).unwrap() + begin.len();
        template.insert_str(pos, &latex);
    }

    if cfg.latex {
        // Output latex file.
        output(".tex".to_string(), title.clone(), &template, &ctx.destination);
    }

    // Output PDF file.
    if cfg.pdf {
        // Write PDF with tectonic.
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

    // Create output directory/file.
    let _ = fs::create_dir_all(destination);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write to file.
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

///
fn path_adder(content: &str, chapter_path: &PathBuf) -> String {
    let mut output = String::new();
    let mut options = Options::empty();
    let parser = Parser::new_ext(content, options);
    for event in parser {
        match event {
            Event::Start(Tag::Image(_, path, title)) => {
                // TODO Append chapter_path to path.
            }
            _ => (),
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_adder() {
        let content = "![xyz](./xyz.png)";
        let path = Path::new("/a/b/c");
        let new_path = path_adder(content, &path);
        assert_eq!(new_path, "![xyz])(/a/b/c/./xyz.png)";
    }
}


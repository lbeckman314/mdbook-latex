#[cfg(feature = "latex")]
extern crate md2tex;
extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "pdf")]
extern crate tectonic;

#[cfg(feature = "md2tex")]
use md2tex::markdown_to_tex;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::path::Path;

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

pub fn generate(ctx: RenderContext) -> std::io::Result<()> {
    // Get configuration options from book.toml.
    let cfg: Config = ctx.config
                         .get_deserialized("output.latex")
                         .unwrap_or_default();

    // Read book's config values (title, authors).
    let title = ctx.config.book.title.as_ref().unwrap();
    let authors = ctx.config.book.authors.join(" \\and ");

    // Copy template data into memory.
    let mut template = if let Some(custom_template) = cfg.custom_template {
        let mut custom_template_path = ctx.root.clone();
        custom_template_path.push(custom_template);
        std::fs::read_to_string(custom_template_path)?
    } else {
        include_str!("template.tex").to_string()
    };

    // Add title and author information.
    template = template.replace(r"\title{}", &format!("\\title{{{}}}", title));
    template = template.replace(r"\author{}", &format!("\\author{{{}}}", authors));

    // Iterate through markdown source.
    let mut content = String::new();
    for item in ctx.book.iter() {
        // Iterate through each chapter.
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            content.push_str(&ch.content);
        }
    }

    if cfg.markdown {
        // Output markdown file.
        let mut filename = title.clone();
        filename.push_str(".md");
        write_file(&content, filename);
    }

    #[cfg(feature = "latex")]
    {
        let mut latex = String::new();
        if cfg.latex || cfg.pdf {
            latex = get_latex(content, template);
        }

        // Output latex file.
        if cfg.latex {
            let mut filename = title.clone();
            filename.push_str(".tex");
            write_file(&latex, filename);
        }

        #[cfg(feature = "pdf")]
        {
            // Output PDF file.
            if cfg.pdf {
                write_pdf(latex, title.clone());
            }
        }
    }

    Ok(())
}

#[cfg(feature = "pdf")]
fn write_pdf(latex: String, filename: String) {
    // Write PDF with tectonic.
    let data: Vec<u8> = tectonic::latex_to_pdf(&latex).expect("processing failed");
    let mut file = filename.clone();
    file.push_str(".pdf");
    let mut output = File::create(file).unwrap();
    output.write(&data).unwrap();
}

#[cfg(feature = "latex")]
fn get_latex(content: String, template: String) -> String {
    // Insert new LaTeX data into template after "%% mdbook-latex begin".
    let begin = "mdbook-latex begin";
    let target = template.find(&begin).unwrap() + begin.len();
    let mut output = template.clone();
    let latex = markdown_to_tex(content.clone());
    output.insert_str(target, &latex);
    output
}

/// Output plain text file.
///
/// Used for writing markdown and latex data to files.
fn write_file(data: &String, filename: String) {
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    // Write to file.
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.to_string()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}



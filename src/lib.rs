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
use mdbook::config::Config as MdConfig;
use mdbook::renderer::RenderContext;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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
    let context = ctx.config.get("output.latex");

    let cfg: Config = match context {
        Some(table) => table.clone().try_into()?,
        None => panic!("Could not read configuration."),
    };

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
        let filename = output_filename(&ctx.destination, &ctx.config, "md");
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
            let filename = output_filename(&ctx.destination, &ctx.config, "tex");
            write_file(&latex, filename);
        }

        #[cfg(feature = "pdf")]
        {
            // Output PDF file.
            if cfg.pdf {
                let filename = output_filename(&ctx.destination, &ctx.config, "pdf");
                write_pdf(latex, filename);
            }
        }
    }

    Ok(())
}

#[cfg(feature = "pdf")]
fn write_pdf(latex: String, filename: PathBuf) {
    // Write PDF with tectonic.
    let data: Vec<u8> = tectonic::latex_to_pdf(&latex).expect("processing failed");
    let mut output = File::create(filename).unwrap();
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
fn write_file(data: &String, filename: PathBuf) {
    let display = filename.display();

    let mut file = match File::create(&filename) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    // Write to file.
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.to_string()),
        Ok(_) => (),
    }
}

pub fn output_filename(dest: &Path, config: &MdConfig, extension: &str) -> PathBuf {
    match config.book.title {
        Some(ref title) => dest.join(title).with_extension(extension),
        None => dest.join("book").with_extension(extension),
    }
}

extern crate md2tex;
extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tectonic;

use md2tex::markdown_to_tex;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use pulldown_cmark::{Event, Options, Parser, Tag};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// config definition.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct LatexConfig {
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

    // Date to be used in the LaTeX \date{} macro
    pub date: Option<String>,
}

impl Default for LatexConfig {
    fn default() -> Self {
        Self {
            ignores: Default::default(),
            latex: true,
            pdf: false,
            markdown: false,
            custom_template: None,
            date: Some(r#"\today"#.to_string()),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stdin = io::stdin();

    // Get markdown source.
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    println!("{:?}", ctx);

    // Get configuration options from book.toml.
    let cfg: LatexConfig = ctx
        .config
        .get_deserialized_opt("output.latex")
        .expect("Error reading \"output.latex\" configuration")
        .unwrap_or_default();

    //if !cfg.latex && !cfg.pdf && !cfg.markdown {
    //Err("No configurations selected.")
    //}

    // Read book's config values (title, authors).
    let title = ctx.config.book.title.unwrap();
    let authors = ctx.config.book.authors.join(" \\and ");

    let date = cfg.date.unwrap_or_default();

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
    template = template.replace(r"\date{}", &format!("\\date{{{}}}", date));

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
            content.push_str(&relative_path(&ch.content, &ch.path.parent().unwrap()));
        }
    }

    if cfg.markdown {
        // Output markdown file.
        output_markdown(".md".to_string(), title.clone(), &content, &ctx.destination);
    }

    if cfg.latex || cfg.pdf {
        // convert markdown data to LaTeX
        latex.push_str(&markdown_to_tex(content));
        println!("latex: {}", latex);

        // Insert new LaTeX data into template after "%% mdbook-latex begin".
        let begin = "mdbook-latex begin";
        let pos = template.find(&begin).unwrap() + begin.len();
        template.insert_str(pos, &latex);
    }

    if cfg.latex {
        // Output latex file.
        output_markdown(
            ".tex".to_string(),
            title.clone(),
            &template,
            &ctx.destination,
        );
    }

    // Output PDF file.
    if cfg.pdf {
        // Write PDF with tectonic.
        println!("Writing PDF with Tectonic...");
        let pdf_data: Vec<u8> = tectonic::latex_to_pdf(&template).expect("processing failed");
        println!("Output PDF size is {} bytes", pdf_data.len());

        let mut pos = 0;

        let mut file_pdf = title;
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
fn output_markdown<P: AsRef<Path>>(
    extension: String,
    mut filename: String,
    data: &str,
    destination: P,
) {
    filename.push_str(&extension);
    let path = Path::new(&filename);
    let display = path.display();

    // Create output directory/file.
    let _ = fs::create_dir_all(destination);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write to file.
    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

///
fn relative_path(content: &str, chapter_path: &Path) -> String {
    let mut new_content = String::from(content);
    let mut new_path = String::new();
    let parser = Parser::new_ext(content, Options::empty());
    for event in parser {
        match event {
            Event::Start(Tag::Image(_, path, _)) => {
                new_path.push_str(chapter_path.to_str().unwrap());
                new_path.push_str("/");
                new_path.push_str(&path.clone().into_string());
                new_content = content.replace(&path.into_string(), &new_path);
            }
            _ => (),
        }
    }

    new_content
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_relative_path() {
        let content = "![123](./xyz.png)";
        let path = PathBuf::from(r"/a/b/c");
        let new_content = relative_path(content, &path);
        assert_eq!("![123](/a/b/c/./xyz.png)", new_content);
    }
}

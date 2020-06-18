extern crate md2tex;
extern crate mdbook;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tectonic;

use md2tex::markdown_to_tex;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use pulldown_cmark::{Event, Options, Parser, Tag, LinkType, CowStr};
use pulldown_cmark_to_cmark::cmark;

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

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();

    // Get markdown source from the mdbook command via stdin
    let ctx = RenderContext::from_json(&mut stdin).unwrap();


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
    let title = ctx.config.book.title.clone().unwrap();
    let authors = ctx.config.book.authors.join(" \\and ");

    // Copy template data into memory.
    let mut template = if let Some(custom_template) = cfg.custom_template {
            let mut custom_template_path = ctx.root.clone();
            custom_template_path.push(custom_template);
            fs::read_to_string(custom_template_path)?
        } else {
            include_str!("template.tex").to_string()
        };

    // Add title and author information.
    template = template.replace(r"\title{}", &format!("\\title{{{}}}", title));
    template = template.replace(r"\author{}", &format!("\\author{{{}}}", authors));
    template = template.replace(r"\date{}", &format!("\\date{{{}}}", date));

    let mut latex = String::new();

    // Iterate through markdown source and push the chapters onto one single string.
    let mut content = String::new();
    for item in ctx.book.iter() {

        // Iterate through each chapter.
        if let BookItem::Chapter(ref ch) = *item {
            if cfg.ignores.contains(&ch.name) {
                continue;
            }

            // Add chapter path to relative links.
            content.push_str(&traverse_markdown(&ch.content, &ch.path.parent().unwrap(), &ctx));
        }
    }

    // println!("{}", content);
    if cfg.markdown {
        // Output markdown file.
        output_markdown(".md".to_string(), title.clone(), &content, &ctx.destination);
    }

    if cfg.latex || cfg.pdf {
        // convert markdown data to LaTeX
        latex.push_str(&markdown_to_tex(content));

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

/// This Function parses the markdown file, alters some elements and writes it back to markdown.
///
/// Changes done:
///   * change image paths to be relative to images
///   * copy the image files into the images directory in the target directory
fn traverse_markdown(content: &str, chapter_path: &Path, context: &RenderContext) -> String {
    let parser = Parser::new_ext(content, Options::all());
    let parser = parser.map(|event| match event {
            Event::Start(Tag::Image(link_type, path, title)) => {
                //Event::Start(Tag::Image(link_type, imagepathcowstr, title))
                Event::Start(parse_image_tag(link_type, path, title, chapter_path, context))
            },
            Event::End(Tag::Image(link_type, path, title)) => {
                //Event::Start(Tag::Image(link_type, imagepathcowstr, title))
                Event::End(parse_image_tag(link_type, path, title, chapter_path, context))
            },
            _ => event,
        });
    let mut new_content = String::new();

    cmark(parser, &mut new_content, None).expect("failed to convert back to markdown");
    return new_content;
}

fn parse_image_tag<'a> (link_type: LinkType, path: CowStr<'a>, title: CowStr<'a>, chapter_path: &'a Path, context: &'a RenderContext) -> Tag <'a> {
    //! Take the values of a Tag::Image and create a new Tag::Image
    //! while simplyfying the path and also copying the image file to the target directory

    // cleaning and converting the path found.
    let pathstr: String = path.replace("./", "");
    let imagefn = Path::new(&pathstr);
    // creating the source path of the mdbook
    let source = context.root.join(context.config.book.src.clone());
    // creating the relative path of the image by prepending the chapterpath

    let relpath = chapter_path.join(imagefn);
    // creating the path of the imagesource
    let sourceimage = source.join(&relpath);
    // creating the relative path for the image tag in markdown
    let imagepath = Path::new("images").join(&relpath);
    // creating the path where the image will be copied to
    let targetimage = context.destination.join(&imagepath);

    // creating the directory if neccessary
    fs::create_dir_all(targetimage.parent().unwrap()).expect("Failed to create the directories");
    // copy the image
    fs::copy(&sourceimage, &targetimage).expect("Failed to copy the image");
    // create the new image
    let imagepathc:String = imagepath.to_str().unwrap().into();
    Tag::Image(link_type, imagepathc.into(), title)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use std::fs::OpenOptions;

    #[test]
    fn test_traverse_markdown() {
        let imgpath = Path::new("/tmp/test/src/chap/xyz.png");
        // create a temporary directory in /tmp/
        fs::create_dir_all(imgpath.parent().unwrap()).expect("failure while creating testdirs");
        // touch the mock png file
        let _ = match OpenOptions::new().create(true).write(true).open(imgpath) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
        let content = "![123](./xyz.png)";
        let path = PathBuf::from(r"chap/");
        let context = RenderContext::new(
            Path::new("/tmp/test/"),
            mdbook::book::Book::new(),
            mdbook::Config::default(),
            Path::new("/tmp/dest/")
        );
        let new_content = traverse_markdown(content, &path, &context);
        assert_eq!("![123](images/chap/xyz.png)", new_content);
        let respath = Path::new("/tmp/dest/images/chap/xyz.png");
        assert!(respath.exists());

        fs::remove_dir_all("/tmp/test").unwrap();
        fs::remove_dir_all("/tmp/dest").unwrap();
    }
}

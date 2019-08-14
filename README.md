# mdbook-latex

[![crates badge][crates-badge]][crates.io]
[![docs badge][docs-badge]][docs]

[crates.io]: https://crates.io/crates/mdbook-latex
[crates-badge]: https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?longCache=true

[docs]: https://docs.rs/crate/mdbook-latex/0.1.0
[docs-badge]: https://docs.rs/mdbook-latex/badge.svg

An [mdbook](https://github.com/rust-lang-nursery/mdBook) backend for generating LaTeX and PDF documents.

> **Warning**: Not yet stable — may eat, shred, and atomize your laundry! See the [**Are We Stable Yet?**](#are-we-stable-yet%3F) section for a roadmap to the production release.

Examples of PDF's generated with `mdbook-latex` include:

| Generated PDF                     | Generated LaTeX         | Source                 | Online Version        |
| --------------------------------- | ----------------------- | ---------------------- | --------------------- |
| [mdbook User Guide][mdbook-pdf]   | [LaTeX][mdbook-latex]   | [Source][mdbook-src]   | [HTML][mdbook-html]   |

[mdbook-pdf]: https://github.com/lbeckman314/mdbook-latex/releases/download/v0.1.0/mdBook.Documentation.pdf
[mdbook-latex]: https://github.com/lbeckman314/mdbook-latex/releases/download/v0.1.0/mdBook.Documentation.tex
[mdbook-html]: https://rust-lang-nursery.github.io/mdBook/
[mdbook-src]: https://github.com/rust-lang-nursery/mdBook/tree/master/book-example

## Installation

First, install the following two programs:

- [Rust](https://www.rust-lang.org/)
- [mdbook](https://github.com/rust-lang-nursery/mdBook)

Then, to install `mdbook-latex`, enter the following in a shell:

```sh
cargo install mdbook-latex
```

Finally, add the following `toml` configuration to `book.toml`.

```toml
[output.latex]
latex = true
pdf = true
```

The next `mdbook build` command will produce LaTeX and PDF files in the `book/latex/` directory.

## Uninstallation

To uninstall `mdbook-latex`, enter the following in a shell:

```sh
cargo uninstall mdbook-latex
```

Then delete the `[output.latex]` configuration in `book.toml`:

```diff
- [output.latex]
- latex = true
- pdf = true
```

## Dependencies

`mdbook-latex` is built upon some really wonderful projects, including:

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark): Parses the markdown source AST.
- [Tectonic](https://tectonic-typesetting.github.io/en-US/): Creates the final PDF file from the transformed LaTeX code.
- [md2tex](https://github.com/lbeckman314/md2tex): Transforms the markdown source to LaTeX. This is a fork of [md2pdf](https://gitea.tforgione.fr/tforgione/md2pdf/), a great utility for converting markdown code to LaTeX and PDF's.  I hope to eventually propose some of the updates back upstream. `md2tex` and `mdbook-latex` are developed in tandem, but are meant to be independent programs. Therefore, if one wishes to use an alternative markdown-to-tex conveter, they should be able to plug it in to `mdbook-latex` with ease.

## How's it Work?

Broadly speaking, there are three steps, or "transformations", from `mdbook` source to PDF output:

1) **mdbook source to JSON-organized markdown** (`mdbook-latex`): retreives the JSON formatted data from `mdbook`. Calls `md2tex` and `tectonic` for LaTeX and PDF generation, respectively.
2) **markdown to LaTeX** (`md2tex`): converts markdown input to LaTeX output.
3) **LaTeX to PDF** (`tectonic`): creates PDF document from LaTeX input.

## Contributing

Pull requests, forks, and plain old copy-pasting are actively encouraged! Also, I am relatively new to Rust (and programming in general) so recommendations or advice in general is always appreciated. 

### I found a problem. Should I create an issue with `mdbook-latex` or `md2tex`?

Either one. `mdbook-latex` can be thought of as a frontend for the LaTeX generation done by `md2tex`. So if there is a LaTeX error, you may favor creating an issue with `md2tex`. Otherwise, creating an issue with `mdbook-latex` is a good bet. But any issue is a good issue, so don't worry if it's in the "right" repository or not, I should be able to see it regardless!

## Are We Stable Yet?

- [ ] Allow SVG images (convert to PNG for LaTeX).
- [ ] Compile *The Rust Book* and *mdbook* documentation without any errors or warnings (e.g. missing Unicode characters).
- [ ] Put "tectonic" dependency in "pdf" feature configuration.
- [ ] Add "table of contents" mdbook toml option.
- [ ] Add "markdown" mdbook toml option.
- [ ] Add "number of words" mdbook toml option.
- [ ] Add test suites.
- [ ] Add CI/CD pipeline ([trust](https://github.com/japaric/trust))
- [ ] Add "examples" directory.
- [ ] Create documentation.
- [ ] Add option for alternative markdown-to-latex converter plugin.

## See Also

The following projects served as guidance for `mdbook-latex` (or are simply cool enough that they deserve to be shared!):

- [mdbook-epub](https://github.com/Michael-F-Bryan/mdbook-epub): A backend for mdbook that creates EPUB files.
- [mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck): A backend for `mdbook` that will verify URL links.
- [crowbook](https://github.com/lise-henry/crowbook/): A rich program that can generate HTML, PDF, **and** EPUB files from markdown code. Has a neat [online demo page](http://vps.crowdagger.fr/crowbook/) to try it out interactively. Similar in some respects to `mdbook`, but with an added focus on "novels and fiction". Though general enough to handle a lot of different projects.

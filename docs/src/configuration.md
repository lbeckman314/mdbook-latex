## Configuration options

In the `[output.latex]` section of book.toml it is possible to set a number of configuration options.

### What gets built and retained

There are three options which determine what files will be built and retained.
The markdown file is an intermediate step for creating the LaTeX file, which in turn is needed to build the PDF.

By default only the LaTeX file is created and retained.
The LaTeX file should be processable by tectonic or other TeX engines.

```toml
[output.latex]
latex    = true  # default = true
pdf      = true  # default = false
markdown = true  # default = false
```
### Other options

There are other options which can be used to define how LaTeX file is build

```toml
[output.latex]
# list of chapters (as named in the SUMMARY.md) to be ignored when building
ignores  = ["Introduction", "On UnTeXible Objects"] # default = []

# Custom LaTeX template. It is expected to include a number of LaTeX packages to define the comments
# that get written to the `.tex` file. Path is relative to the book root directory (typically the same
# directory this TOML file lives in)
custom-template = "path/to/my-tempate.tex" # default is None

# date to be used as argument to the \date{} command.
date = "18 January 2038" # default = "\\today"
```

Note that when `pdf = true`, the call to process LaTeX file does not pass in the current date or time, so
the resulting PDF will have a date from the beginning of the Unix Epoch.
To avoid that you will need to explicitly
set a date in this configuration or to generate the LaTeX and process that to PDF on your own.
#! /bin/env bash

if [ ! -d "book" ]
then
	git clone --depth=1 https://github.com/rust-lang/rust-by-example.git
	echo '
	[output.latex]
	pdf = true
	latex = true
	custom-template = "../template.tex"
	' >> rust-by-example/book.toml
fi

cd rust-by-example || exit


mdbook build

cp book/latex/*.pdf ..

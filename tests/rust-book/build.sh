#! /bin/env bash

set -e

if [ ! -d "book" ]
then
	git clone --depth=1 https://github.com/rust-lang/book.git
	echo '
[output.latex]
pdf = true
latex = true
custom-template = "../template.tex"
' >> book/book.toml

fi

cd book || exit


mdbook build

cp book/latex/*.pdf ..

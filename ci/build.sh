#!/bin/bash
# Builds mdbooks with mdbook-latex.

# Sources:
#   https://unix.stackexchange.com/questions/325490/how-to-get-last-part-of-http-link-in-bash/325492
#   https://stackoverflow.com/questions/5947742/how-to-change-the-output-color-of-echo-in-linux/20983251#20983251
#   https://stackoverflow.com/questions/2990414/echo-that-outputs-to-stderr

# mdbooks to build
# Each element is composed of the following "tuple":
# <git url> <path to mdbook directory>
books=(
    # mdbook
    "https://github.com/rust-lang-nursery/mdBook" "book-example"

    # rust programming language
    "https://github.com/rust-lang/book" ""

    # rust by example
    # "https://github.com/rust-lang/rust-by-example" ""

    # edition guide
    # "https://github.com/rust-lang-nursery/edition-guide" ""

    # rustc
    # "https://github.com/rust-lang/rustc-guide" ""

    # cargo
    # "https://github.com/rust-lang/cargo/" "src/doc"

    # rustdoc
    # "https://github.com/rust-lang/rust/" "src/doc/rustdoc"

    # reference
    # "https://github.com/rust-lang-nursery/reference" ""

    # nomicon
    # "https://github.com/rust-lang-nursery/nomicon" ""

    # unstable
    # "https://github.com/rust-lang/rust/" "src/doc/unstable-book"

    # embedded
    # "https://github.com/rust-embedded/book" ""
)


# mdbook-latex config
config="
[output.latex]
latex = true
pdf = true
markdown = true
"

# bold font
BOLD='\033[1m'
# green
COLOR='\033[0;32m'
# red
ERROR='\033[0;31m'
# No Color
NC='\033[0m' 

# reset any changes to the working tree
reset() {
    dir=$1    
    git -C "$dir" checkout -- .
}

main() {
    # check for cargo
    if ! command -v cargo >/dev/null
    then
        >&2 echo -e "${ERROR}${BOLD}Cargo not detected.${NC}"
        >&2 echo -e "See https://www.rust-lang.org/learn/get-started for installation instructions."
        exit 1
    fi

    # check for mdbook
    if [ $(cargo install --list | grep --count mdbook$) -eq 0 ]
    then
        cargo install mdbook
    fi

    # check for mdbook-latex
    if [ $(cargo install --list | grep --count mdbook-latex) -eq 0 ]
    then
        cargo install --path . --force
        cargo test
    fi

    i=1
    elements="${#books[@]}"
    total="$(($elements / 2))"

    # iterate over all books
    for ((n=0; n<"$elements"; n+=2))
    do
        book="${books[n]}"
        book_dir="${books[n+1]}"
        path="${book##*/}"
        title="$(cat $path/$book_dir/book.toml | grep "^title" | sed 's/.*= //;s/"//g')"

        echo -e "${COLOR}${BOLD}Building $i/$total:${NC} $title"

        # clean directory
        reset "$path"

        # build mdbooks with mdbook-latex
        git clone "$book"
        echo "$config" >> "$path/$book_dir"/book.toml
        mdbook build "$path/$book_dir/"

        ((i++))
    done
}

main

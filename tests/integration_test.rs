extern crate serde_derive;
extern crate mdbook_latex;

use std::fs::read_to_string;
use std::error::Error;
use std::io;
use mdbook_latex::generate;

#[test]
fn integration_test() {
    let context_file = "tests/ctx.json";
    let context = read_to_string(context_file).unwrap();
    generate(context);

    assert!(true);
}


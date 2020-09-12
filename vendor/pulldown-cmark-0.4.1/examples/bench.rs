extern crate pulldown_cmark;

use pulldown_cmark::{Parser, Options, html};

fn main() {
    let inp = include_bytes!("../third_party/xi-editor/crdt.md");
    let markdown_input = std::str::from_utf8(inp).unwrap();
    println!("inp: {}", inp.len());

    let options = Options::empty();
    for _ in 0..1000 {
        let parser = Parser::new_ext(markdown_input, options);

        // Write to String buffer.
        let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
        html::push_html(&mut html_output, parser);
    }
}


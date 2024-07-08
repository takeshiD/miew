use markdown::{to_mdast, ParseOptions};
use std::fs::read
fn main() {
    println!("{:#?}", to_mdast("## Helo, *world*!", &ParseOptions::default()).unwrap());
    for n in to_mdast('## Hello , *world*!',&ParseOptions::default()).iter() {
        println!("{:?}", n);
    }
}

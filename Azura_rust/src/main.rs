use std::io::stdout;

use azura::scanner::{Scanner, ScannerError, ScannerErrorKind};
use lending_iterator::LendingIterator;

fn main() {
    let input = "// This is a comment
100 + (4.50 * 24.05) / 10
0.1.1
25 + 30.695;
";
    // let input = "";
    let scanner = Scanner::new(input);

    scanner.for_each(|token| {
        match token {
            Ok(token) => println!("{token:?}"),
            Err(error) => println!("{error}"),
        };
    });
}

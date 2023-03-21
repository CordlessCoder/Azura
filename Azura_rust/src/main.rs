use std::io::{stdout, Write};

use azura::scanner::Scanner;
use lending_iterator::LendingIterator;

fn main() {
    let input = "// This is a comment
100 + (4.50 * 24.05) / 10
0.1.1
25 + 30.695;
";
    // let input = "";
    let scanner = Scanner::new(input);

    let mut stdout = stdout().lock();
    scanner.for_each(|token| {
        let _ = match token {
            Ok(token) => writeln!(stdout, "{token:?}"),
            Err(error) => writeln!(stdout, "{error}"),
        };
    });
}

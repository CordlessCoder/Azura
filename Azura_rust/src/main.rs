use std::io::{stderr, stdout, Write};

use azura::scanner::Scanner;
use lending_iterator::LendingIterator;
use owo_colors::OwoColorize;

fn main() {
    let input = "// This is a comment
100 + (4.50 * 24.05) / 10
0.1
0x01.
25 + 30.695;
2 << 1
";
    // let input = "";
    let scanner = Scanner::new(input);

    let mut stdout = stdout().lock();
    let mut stderr = stderr().lock();
    let mut failure = false;
    scanner.for_each(|token| {
        let _ = match token {
            Ok(token) => writeln!(stdout, "{token:?}"),
            Err(error) => {
                failure = true;
                writeln!(stderr, "{error}")
            }
        };
    });
    if failure {
        let _ = writeln!(
            stderr,
            "{}",
            "An error occured while scanning"
                .if_supports_color(owo_colors::Stream::Stderr, |text| text.red())
        );
    }
}

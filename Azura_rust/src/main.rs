use std::io::{stderr, stdout, Write};

use azura::scanner::Scanner;
use lending_iterator::LendingIterator;
use owo_colors::OwoColorize;

fn main() {
    let input = "// This is a comment
'Incorrect integer:'
228o
\"Unterminated string:\"
'
/* ' */





";
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

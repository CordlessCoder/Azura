use std::io::stdout;

use azura::scanner::{Scanner, ScannerError, ScannerErrorKind};

fn main() {
    let input = "// This is a comment
100 + (4.50 * 24.05) / 10
0.1.1
25 + 30.695;
";
    // let input = "";
    let mut scanner = Scanner::new(input);
    let mut next = scanner.next();
    while !matches!(next, Err(ScannerError { ref kind, .. }) if kind == &ScannerErrorKind::EndOfInput)
    {
        match next {
            Ok(token) => println!("{token:?}"),
            Err(error) => println!("{error}"),
        }
        next = scanner.next();
    }
}

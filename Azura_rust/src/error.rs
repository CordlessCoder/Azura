use std::borrow::Cow;

use crate::scanner::TokenKind;

mod format;

#[derive(Debug)]
pub enum ScannerErrorKind<'a> {
    Unmatched {
        token: Option<TokenKind<'a>>,
    },
    IncompleteToken {
        token: Option<TokenKind<'a>>,
    },
    IncorrectLiteral {
        parse_error: Option<Box<dyn std::error::Error + Send>>,
    },
}

impl<'a> PartialEq for ScannerErrorKind<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unmatched { token }, Self::Unmatched { token: token2 }) if token == token2 => {
                true
            }
            (Self::IncompleteToken { token }, Self::IncompleteToken { token: token2 })
                if token == token2 =>
            {
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ScannerError<'a> {
    pub kind: ScannerErrorKind<'a>,
    pub line: usize,
    pub pos: usize,
    pub message: Option<Cow<'a, str>>,
    pub context: Option<&'a str>,
}

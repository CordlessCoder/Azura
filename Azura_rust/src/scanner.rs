mod tokens;
use std::{
    borrow::Cow,
    iter::{Enumerate, Peekable},
    str::Bytes,
};

use lending_iterator::LendingIterator;

pub use self::tokens::TokenKind;
pub use crate::error::{ScannerError, ScannerErrorKind};

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    line: usize,
}

struct ByteWrapper<'a>(Peekable<Enumerate<Bytes<'a>>>);
impl<'a> ByteWrapper<'a> {
    fn new(iter: Peekable<Enumerate<Bytes<'a>>>) -> Self {
        Self(iter)
    }
    fn next_byte(&mut self) -> Option<u8> {
        self.0.next().map(|(_, b)| b)
    }
    fn next_idx(&mut self) -> Option<usize> {
        self.0.next().map(|(idx, _)| idx)
    }
    fn next_both(&mut self) -> Option<(usize, u8)> {
        self.0.next()
    }
    fn peek(&mut self) -> Option<u8> {
        self.0.peek().map(|(_, b)| *b)
    }
    fn peek_idx(&mut self) -> Option<usize> {
        self.0.peek().map(|(idx, _)| *idx)
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, line: 1 }
    }
}
use lending_iterator::prelude::*;
#[gat]
impl<'iter> LendingIterator for Scanner<'iter> {
    type Item<'next>
    where
        Self: 'next,
    = Result<TokenKind<'next>, ScannerError<'next>>;

    fn next<'next>(
        self: &'next mut Scanner<'iter>,
    ) -> Option<Result<TokenKind<'next>, ScannerError<'next>>> {
        let mut bytes = ByteWrapper::new(self.source.bytes().enumerate().peekable());
        let out = 'mainloop: loop {
            use TokenKind::*;
            let Some((pos,byte)) = bytes.next_both() else {
                return None;
            };
            break match byte {
                b'\n' => {
                    self.line += 1;
                    self.source = self.source.get(pos + 1..).unwrap();
                    // Reset bytes iterator as it needs to stay in sync with the string
                    bytes = ByteWrapper::new(self.source.bytes().enumerate().peekable());
                    continue;
                }
                b'(' => Ok(OPar),
                b')' => Ok(CPar),
                b'{' => Ok(OBrace),
                b'}' => Ok(CBrace),
                b',' => Ok(Comma),
                b'.' => Ok(Dot),
                b';' => Ok(Semicolon),
                b'#' => Ok(Hashtag),
                b'+' => {
                    // + +=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(AddAssign)
                    } else {
                        Ok(Add)
                    }
                }
                b'-' => {
                    // - -=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_byte();
                        Ok(SubAssign)
                    } else {
                        Ok(Sub)
                    }
                }
                b'/' => {
                    // / /=
                    match bytes.peek() {
                        Some(b'=') => {
                            bytes.next_byte();
                            Ok(DivAssign)
                        }
                        // normal comment
                        Some(b'/') => {
                            while {
                                let peek = bytes.peek();
                                peek.is_some() && peek != Some(b'\n')
                            } {
                                bytes.next_both();
                            }
                            // Not consuming the last newline as that would break the line count
                            continue;
                        }
                        /* block comments */
                        Some(b'*') => {
                            let mut current = bytes.next_byte();
                            let mut peek = bytes.peek();
                            while {
                                peek.is_some() && !(current == Some(b'*') && peek == Some(b'/'))
                            } {
                                current = bytes.next_byte();
                                peek = bytes.peek();
                            }
                            bytes.next_byte();
                            continue;
                        }
                        _ => Ok(Div),
                    }
                }
                b'*' => {
                    // * *=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(MulAssign)
                    } else {
                        Ok(Mul)
                    }
                }
                b':' => {
                    // : :=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_byte();
                        Ok(Walrus)
                    } else {
                        Ok(Colon)
                    }
                }
                b'=' => {
                    // = ==
                    if bytes.peek() == Some(b'=') {
                        bytes.next_byte();
                        Ok(Equal)
                    } else {
                        Ok(Reassignment)
                    }
                }
                b'!' => {
                    // ! !=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(NotEqual)
                    } else {
                        Ok(Bang)
                    }
                }
                b'>' => {
                    // > >= >> >>=

                    match bytes.peek() {
                        Some(b'=') => {
                            bytes.next_both();
                            Ok(GreaterOrEqual)
                        }
                        Some(b'>') => {
                            bytes.next_both();
                            if bytes.peek() == Some(b'=') {
                                bytes.next_both();
                                Ok(RightShiftAssign)
                            } else {
                                Ok(RightShift)
                            }
                        }
                        _ => Ok(Greater),
                    }
                }
                b'<' => {
                    // < <= << <<=
                    match bytes.peek() {
                        Some(b'=') => {
                            bytes.next_both();
                            Ok(LessOrEqual)
                        }
                        Some(b'<') => {
                            bytes.next_both();
                            if bytes.peek() == Some(b'=') {
                                bytes.next_both();
                                Ok(LeftShiftAssign)
                            } else {
                                Ok(LeftShift)
                            }
                        }
                        _ => Ok(Less),
                    }
                }
                b'%' => {
                    // % %=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(RemAssign)
                    } else {
                        Ok(Rem)
                    }
                }
                b'|' => {
                    // | |=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(BitOrAssign)
                    } else {
                        Ok(BitOr)
                    }
                }
                b'^' => {
                    // ^ ^=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(BitXorAssign)
                    } else {
                        Ok(BitXor)
                    }
                }
                b'&' => {
                    // & &=
                    if bytes.peek() == Some(b'=') {
                        bytes.next_both();
                        Ok(BitAndAssign)
                    } else {
                        Ok(BitAnd)
                    }
                }
                // String handling logic
                quote if matches!(quote, b'\'' | b'"') => {
                    let Some(start) = bytes.peek_idx() else {
                        break Err(ScannerError { line: self.line, pos,  message: Some("Untermiated string at the end of input".into()), kind: ScannerErrorKind::IncompleteToken {token: Some(Str(""))}, context: None })
                    };
                    let mut end = start;
                    let mut current = None;
                    // Used for escaping quotes with `\`
                    while !(current != Some(b'\\') && bytes.peek() == Some(quote)) {
                        current = bytes.next_byte();
                        if current.is_none() {
                            break 'mainloop Err(ScannerError {
                                line: self.line,
                                pos,
                                kind: ScannerErrorKind::Unmatched {
                                    token: Some(Str(&self.source[start - 1..end])),
                                },
                                context: Some(self.source.get(pos..end).unwrap()),
                                message: Some(Cow::Borrowed("Unterminated string")),
                            });
                        };
                        end += 1;
                    }
                    bytes.next_both(); // Consumes the final quote
                                       // SAFETY: end should never be advanced past the of string
                    Ok(Str(self.source.get(start..end).expect(
                        "Somehow tried to get a string outside of the array",
                    )))
                }
                digit
                    if matches!(
                        digit,
                        b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9'
                    ) =>
                {
                    // Handle integer radix prefixes i.e 0b for binary, 0x for hexadecimal,
                    // 0o for octal
                    let picked = 'ragixpick: {
                        if digit == b'0' {
                            let Some(suffix) = bytes.peek() else {
                            break 'ragixpick None;
                        };
                            match suffix {
                                b'b' => Some((2, 2, "binary")),
                                b'x' => Some((16, 2, "hexadecimal")),
                                b'o' => Some((8, 2, "octal")),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    };
                    let handled_suffix = picked.is_some();
                    let (radix, skip, base_name) = picked.unwrap_or((10, 0, "decimal"));

                    let mut float = false;

                    (0..skip).for_each(|_| {
                        bytes.next_both();
                    });
                    let start = pos + skip;
                    let mut end = start;
                    // Keep "walking" forward until EOF or anything marked in `numeric_terminator`
                    while {
                        let peek = bytes.peek();
                        peek.is_some() && !matches!(peek, Some(ch) if numeric_terminator(ch) )
                    } {
                        float |= bytes.next_byte() == Some(b'.');
                        end += 1;
                    }
                    let number = &self.source[start..=end];

                    if float {
                        if handled_suffix {
                            Err(ScannerError {
                                kind: ScannerErrorKind::IncorrectLiteral { parse_error: None },
                                line: self.line,
                                pos,
                                message: Some(Cow::Owned(format!(
                                    "literal prefix `{}` suggests the float to be {base_name}",
                                    &self.source[pos..pos + skip]
                                ))),
                                context: self.source.get(pos..=end),
                            })
                        } else {
                            match number.parse() {
                                Ok(float) => Ok(Float(float)),
                                Err(error) => Err(ScannerError {
                                    kind: ScannerErrorKind::IncorrectLiteral {
                                        parse_error: Some(Box::new(error)),
                                    },
                                    line: self.line,
                                    pos,
                                    message: Some("Failed to parse float literal".into()),
                                    context: self.source.get(pos..=end),
                                }),
                            }
                        }
                    } else {
                        match isize::from_str_radix(number, radix) {
                            Ok(parsed) => Ok(Integer(parsed)),
                            Err(error) => Err(ScannerError {
                                kind: ScannerErrorKind::IncorrectLiteral {
                                    parse_error: Some(Box::new(error)),
                                },
                                line: self.line,
                                pos,
                                message: Some(
                                    format!("Failed to parse {base_name} integer literal").into(),
                                ),
                                context: self.source.get(pos..=end),
                            }),
                        }
                    }
                }
                ch if ch.is_ascii_whitespace() => continue,
                // fallback for identifiers
                _ => {
                    let start = pos;
                    let mut end = start;
                    // Keep "walking" forward until EOF or anything marked in `numeric_terminator`
                    while {
                        let peek = bytes.peek();
                        peek.is_some() && !matches!(peek, Some(ch) if ch.is_ascii_whitespace() )
                    } {
                        end += 1;
                        bytes.next_both();
                    }
                    let identifier = &self.source[start..=end];
                    Ok(Ident(identifier))
                }
            };
        };
        if let Some(consumed) = bytes.next_idx() {
            self.source = self.source.get(consumed..).unwrap_or_default()
        } else {
            self.source = "";
        }
        Some(out)
    }
}

fn numeric_terminator(check: u8) -> bool {
    check.is_ascii_whitespace() || !check.is_ascii_alphanumeric() && check != b'.'
}

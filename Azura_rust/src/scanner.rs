mod tokens;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use self::tokens::TokenKind;

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    line: usize,
}

struct CharWrapper<'a>(Peekable<Enumerate<Chars<'a>>>);
impl<'a> CharWrapper<'a> {
    fn new(iter: Peekable<Enumerate<Chars<'a>>>) -> Self {
        Self(iter)
    }
    fn next_char(&mut self) -> Option<char> {
        self.0.next().map(|(_, ch)| ch)
    }
    fn next_idx(&mut self) -> Option<usize> {
        self.0.next().map(|(idx, _)| idx)
    }
    fn next_both(&mut self) -> Option<(usize, char)> {
        self.0.next()
    }
    fn peek(&mut self) -> Option<char> {
        self.0.peek().map(|(_, ch)| *ch)
    }
    fn peek_idx(&mut self) -> Option<usize> {
        self.0.peek().map(|(idx, _)| *idx)
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, line: 1 }
    }

    pub fn next(&mut self) -> Result<TokenKind<'a>, ScannerError> {
        let mut chars = CharWrapper::new(self.source.chars().enumerate().peekable());
        let out = 'mainloop: loop {
            use TokenKind::*;
            let Some((pos,ch)) = chars.next_both() else {
                break Err(ScannerError {kind:  ScannerErrorKind::EndOfInput, line: self.line, pos:0, message: None});
            };
            break match ch {
                '\n' => {
                    self.line += 1;
                    continue;
                }
                '(' => Ok(OPar),
                ')' => Ok(CPar),
                '{' => Ok(OBrace),
                '}' => Ok(CBrace),
                ',' => Ok(Comma),
                '.' => Ok(Dot),
                ';' => Ok(Semicolon),
                '#' => Ok(Hashtag),
                '+' => {
                    // + +=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(AddAssign)
                    } else {
                        Ok(Add)
                    }
                }
                '-' => {
                    // - -=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(SubAssign)
                    } else {
                        Ok(Sub)
                    }
                }
                '/' => {
                    // / /=
                    match chars.peek() {
                        Some('=') => {
                            chars.next_char();
                            Ok(DivAssign)
                        }
                        // normal comment
                        Some('/') => {
                            while {
                                let peek = chars.peek();
                                peek.is_some() && peek != Some('\n')
                            } {
                                chars.next_char();
                            }
                            chars.next_char();
                            continue;
                        }
                        /* block comments */
                        Some('*') => {
                            let mut cur_char = chars.next_char();
                            while {
                                let peek = chars.peek();
                                peek.is_some() && !(cur_char == Some('*') && peek == Some('/'))
                            } {
                                cur_char = chars.next_char()
                            }
                            chars.next_char();
                            continue;
                        }
                        _ => Ok(Div),
                    }
                }
                '*' => {
                    // * *=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(MulAssign)
                    } else {
                        Ok(Mul)
                    }
                }
                ':' => {
                    // : :=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(Walrus)
                    } else {
                        Ok(Colon)
                    }
                }
                '=' => {
                    // = ==
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(Equal)
                    } else {
                        Ok(Reassignment)
                    }
                }
                '!' => {
                    // ! !=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(NotEqual)
                    } else {
                        Ok(Bang)
                    }
                }
                '>' => {
                    // > >=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(GreaterOrEqual)
                    } else {
                        Ok(Greater)
                    }
                }
                '<' => {
                    // < <=
                    if chars.peek() == Some('=') {
                        chars.next_char();
                        Ok(LessOrEqual)
                    } else {
                        Ok(Less)
                    }
                }
                // String handling logic
                quote if matches!(quote, '\'' | '"') => {
                    let Some(start) = chars.peek_idx() else {
                        break Err(ScannerError { line: self.line, pos,  message: Some("String start at the end of input"), kind: ScannerErrorKind::IncompleteToken {token: Some(Str(""))} })
                    };
                    let mut end = start;
                    let mut current = None;
                    // Used for escaping quotes with `\`
                    while !(current != Some('\\') && chars.peek() == Some(quote)) {
                        current = chars.next_char();
                        if current.is_none() {
                            break 'mainloop Err(ScannerError {
                                line: self.line,
                                pos,
                                kind: ScannerErrorKind::Unmatched {
                                    token: Some(Str(&self.source[start - 1..end])),
                                },
                                message: Some("Unescaped string"),
                            });
                        };
                        end += 1;
                    }
                    chars.next_both(); // Consumes the final quote
                                       // SAFETY: end should never be advanced past the of string
                    Ok(Str(self.source.get(start..end).expect(
                        "Somehow tried to get a string outside of the array",
                    )))
                }
                digit
                    if matches!(
                        digit,
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                    ) =>
                {
                    // Handle integer radix prefixes i.e 0b for binary, 0x for hexadecimal just 0
                    // for octal
                    let picked = 'ragixpick: {
                        if digit == '0' {
                            let Some(suffix) = chars.peek() else {
                            break 'ragixpick None;
                        };
                            match suffix {
                                'b' => Some((2, 2)),
                                'x' => Some((16, 2)),
                                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                    Some((8, 1))
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    };
                    let handled_suffix = picked.is_some();
                    let (radix, skip) = picked.unwrap_or((10, 0));

                    let mut float = false;

                    (0..skip).for_each(|_| {
                        chars.next_char();
                    });
                    let start = pos + skip;
                    let mut end = start;
                    // Keep "walking" forward until EOF or anything marked in `numeric_terminator`
                    while {
                        let peek = chars.peek();
                        peek.is_some() && !matches!(peek, Some(ch) if numeric_terminator(ch) )
                    } {
                        float |= chars.next_char() == Some('.');
                        end += 1;
                    }
                    let integer = &self.source[start..=end];

                    if float {
                        if handled_suffix {
                            Err(ScannerError { kind: ScannerErrorKind::IncorrectLiteral { length: end - pos }, line: self.line, pos, message: Some("Attempted to create a float literal with a leading zero or base suffix(0, 0b or 0x)") })
                        } else {
                            match integer.parse() {
                                Ok(float) => Ok(Float(float)),
                                Err(_) => Err(ScannerError {
                                    kind: ScannerErrorKind::IncorrectLiteral { length: end + 1 },
                                    line: self.line,
                                    pos,
                                    message: Some("Failed to parse float literal"),
                                }),
                            }
                        }
                    } else {
                        match isize::from_str_radix(integer, radix) {
                            Ok(parsed) => Ok(Integer(parsed)),
                            Err(_) => Err(ScannerError {
                                kind: ScannerErrorKind::IncorrectLiteral { length: end + 1 },
                                line: self.line,
                                pos,
                                message: Some("Failed to parse integer literal"),
                            }),
                        }
                    }
                }
                ch if ch.is_ascii_whitespace() => continue,
                // fallback for identifiers
                _ => {
                    // Assume this is an identity
                    let start = pos;
                    let mut end = start;
                    // Keep "walking" forward until EOF or anything marked in `numeric_terminator`
                    while {
                        let peek = chars.peek();
                        peek.is_some() && !matches!(peek, Some(ch) if ch.is_ascii_whitespace() )
                    } {
                        end += 1;
                        chars.next_char();
                    }
                    let identifier = &self.source[start..=end];
                    Ok(Ident(identifier))
                }
            };
        };
        if let Some(consumed) = chars.next_idx() {
            self.source = self.source.get(consumed..).unwrap_or_default()
        } else {
            self.source = "";
        }
        out
    }
}

fn numeric_terminator(check: char) -> bool {
    check.is_ascii_whitespace() || !check.is_alphanumeric() && check != '.'
}

#[derive(Debug, PartialEq, Default)]
pub enum ScannerErrorKind<'a> {
    #[default]
    EndOfInput,
    Unmatched {
        token: Option<TokenKind<'a>>,
    },
    IncompleteToken {
        token: Option<TokenKind<'a>>,
    },
    IncorrectLiteral {
        length: usize,
    },
}

#[derive(Debug, PartialEq, Default)]
pub struct ScannerError<'a> {
    pub kind: ScannerErrorKind<'a>,
    pub line: usize,
    pub pos: usize,
    pub message: Option<&'a str>,
}

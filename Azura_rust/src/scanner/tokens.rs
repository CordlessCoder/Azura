#[derive(Debug, PartialEq)]
pub enum TokenKind<'a> {
    // Single-character tokens;
    OPar,
    CPar,
    OBrace,
    CBrace,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Hashtag,
    // One or two character tokens
    Walrus,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
    Add,
    AddAssign,
    Equal,
    NotEqual,
    Bang,
    Reassignment,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    BitOr, // Bitwise or, |
    BitOrAssign,
    BitXor, // Bitwise exclusive or, ^
    BitXorAssign,
    Rem, // Remainder, %
    RemAssign,
    BitAnd, // Bitwise and, &
    BitAndAssign,
    LeftShift, // Bitwise left-shift, <<
    LeftShiftAssign,
    RightShift, // Bitwise right-shift, >>
    RightShiftAssign,
    // Literals
    Ident(&'a str),
    Str(&'a str),
    Integer(isize), // TODO: Handle overflows, arbitrary precision
    Float(f64),     // <-/

                    // // Keywords
                    // And,
                    // Class,
                    // Else,
                    // False,
                    // For,
                    // Func,
                    // If,
                    // Nil,
                    // Or,
                    // Info,
                    // Return,
                    // Super,
                    // This,
                    // True,
                    // Var,
                    // While,
                    // Interpolation,
}

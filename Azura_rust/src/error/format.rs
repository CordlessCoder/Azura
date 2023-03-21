use std::fmt::Display;

use owo_colors::{OwoColorize, Stream::Stderr, Style};

use super::{ScannerError, ScannerErrorKind};

impl<'a> Display for ScannerError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ScannerErrorKind::*;
        let ScannerError {
            kind,
            line,
            pos,
            message,
            context,
        } = self;
        // let show_pos = match kind {
        //     EndOfInput => false,
        //     _ => true,
        // };
        let show_pos = true;
        if show_pos {
            write!(
                f,
                "{} at {}:{}\n{}",
                "Lexical error".if_supports_color(Stderr, |text| text.blue()),
                line.if_supports_color(Stderr, |text| text.bright_red()),
                pos.if_supports_color(Stderr, |text| text.red()),
                message
                    .as_ref()
                    .map(|x| x.as_ref())
                    .unwrap_or_default()
                    .if_supports_color(Stderr, |text| text.italic())
            )?
        };
        let end_text = match kind {
            Unmatched { token } => {
                f.write_str("\n")?;
                if let Some(token) = token {
                    write!(f, "Unmatched token: {:?}", token)?
                } else {
                    f.write_str("Unmatched token")?
                };

                "."
            }
            IncompleteToken { token } => {
                f.write_str("\n")?;
                if let Some(token) = token {
                    write!(f, "Unmatched token: {:?}", token)?
                } else {
                    f.write_str("Unmatched token")?
                };
                ""
            }
            IncorrectLiteral { parse_error: error } => {
                if let Some(error) = error {
                    write!(
                        f,
                        "\n{:?}",
                        error.if_supports_color(Stderr, |text| {
                            let style = Style::new().red().bold();
                            text.style(style)
                        })
                    )?
                }
                "\nIncorrect literal"
            }
        };
        f.write_str(end_text)?;
        if let Some(context) = context {
            write!(
                f,
                "\nin {:?}",
                context.if_supports_color(Stderr, |text| text.purple())
            )?
        }
        Ok(())
    }
}

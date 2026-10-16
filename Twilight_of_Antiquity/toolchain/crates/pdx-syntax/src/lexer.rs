use crate::{Diagnostic, TextRange, Token, TokenKind};

pub(crate) fn lex(source: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    let mut pos = 0;
    while pos < source.len() {
        let start = pos;
        let ch = source[pos..].chars().next().unwrap();
        let kind = match ch {
            '\u{feff}' if pos == 0 => {
                pos += ch.len_utf8();
                TokenKind::Bom
            }
            c if c.is_whitespace() => {
                while pos < source.len() {
                    let next = source[pos..].chars().next().unwrap();
                    if !next.is_whitespace() {
                        break;
                    }
                    pos += next.len_utf8();
                }
                TokenKind::Whitespace
            }
            '#' => {
                pos = source[pos..]
                    .find(['\r', '\n'])
                    .map_or(source.len(), |n| pos + n);
                TokenKind::Comment
            }
            '"' => {
                pos += 1;
                let mut closed = false;
                while pos < source.len() {
                    let next = source[pos..].chars().next().unwrap();
                    if matches!(next, '\r' | '\n') {
                        break;
                    }
                    pos += next.len_utf8();
                    if next == '"' {
                        closed = true;
                        break;
                    }
                    if next == '\\' && pos < source.len() {
                        let escaped = source[pos..].chars().next().unwrap();
                        if matches!(escaped, '\r' | '\n') {
                            break;
                        }
                        pos += escaped.len_utf8();
                    }
                }
                if !closed {
                    diagnostics.push(Diagnostic {
                        rule_id: "SYN-001",
                        kind: "unterminated_string",
                        range: TextRange { start, end: pos },
                    });
                }
                TokenKind::Quoted
            }
            '{' => {
                pos += 1;
                TokenKind::OpenBrace
            }
            '}' => {
                pos += 1;
                TokenKind::CloseBrace
            }
            '=' | '<' | '>' | '!' | '?' => {
                pos += 1;
                if source.as_bytes().get(pos) == Some(&b'=') {
                    pos += 1;
                    TokenKind::Operator
                } else if matches!(ch, '!' | '?') {
                    TokenKind::Unsupported
                } else {
                    TokenKind::Operator
                }
            }
            '[' | ']' | '\u{feff}' => {
                pos += ch.len_utf8();
                TokenKind::Unsupported
            }
            c if c.is_control() => {
                pos += c.len_utf8();
                TokenKind::Unsupported
            }
            _ => {
                while pos < source.len() {
                    let next = source[pos..].chars().next().unwrap();
                    if next.is_whitespace()
                        || next.is_control()
                        || matches!(
                            next,
                            '#' | '"'
                                | '{'
                                | '}'
                                | '='
                                | '<'
                                | '>'
                                | '!'
                                | '?'
                                | '['
                                | ']'
                                | '\u{feff}'
                        )
                    {
                        break;
                    }
                    pos += next.len_utf8();
                }
                TokenKind::Atom
            }
        };
        let range = TextRange { start, end: pos };
        if kind == TokenKind::Unsupported {
            diagnostics.push(Diagnostic {
                rule_id: "SYN-002",
                kind: "unsupported_token",
                range,
            });
        }
        tokens.push(Token { kind, range });
    }
    (tokens, diagnostics)
}

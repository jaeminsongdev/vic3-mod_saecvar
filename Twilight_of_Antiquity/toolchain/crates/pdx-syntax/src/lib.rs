//! Independent text-script syntax subset. Syntax acceptance is not engine validation.
mod lexer;
mod parser;

pub const MAX_DEPTH: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Bom,
    Whitespace,
    Comment,
    Atom,
    Quoted,
    OpenBrace,
    CloseBrace,
    Operator,
    Unsupported,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Bom | Self::Whitespace | Self::Comment)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub rule_id: &'static str,
    pub kind: &'static str,
    pub range: TextRange,
}

impl Diagnostic {
    pub fn verification_status(&self) -> &'static str {
        if self.rule_id == "SYN-001" {
            "STATIC_VERIFIED"
        } else {
            "UNSUPPORTED_RULE"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CstItem {
    Property {
        key: usize,
        operator: usize,
        value: CstValue,
        range: TextRange,
    },
    Value(CstValue),
    Error(TextRange),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CstValue {
    pub kind: ValueKind,
    pub range: TextRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind {
    Scalar(usize),
    Block {
        open: usize,
        items: Vec<CstItem>,
        close: Option<usize>,
    },
    Opaque,
    Missing,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstItem<'a> {
    Property {
        key: &'a str,
        operator: &'a str,
        value: AstValue<'a>,
    },
    Value(AstValue<'a>),
    Error(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstValue<'a> {
    Scalar { raw: &'a str, quoted: bool },
    Block(Vec<AstItem<'a>>),
    Opaque(&'a str),
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    /// One-based Unicode scalar column, excluding the initial BOM. Not UTF-16.
    pub column: usize,
}

#[derive(Debug)]
pub struct Parsed<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    pub items: Vec<CstItem>,
    pub diagnostics: Vec<Diagnostic>,
    line_starts: Vec<usize>,
}

pub fn parse(source: &str) -> Parsed<'_> {
    let (tokens, mut diagnostics) = lexer::lex(source);
    let items = parser::parse_items(&tokens, source.len(), &mut diagnostics);
    diagnostics.sort_by_key(|d| (d.range.start, d.range.end, d.kind));
    let mut line_starts = vec![0];
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' {
            i += 1;
            if bytes.get(i) == Some(&b'\n') {
                i += 1;
            }
            line_starts.push(i);
        } else {
            if bytes[i] == b'\n' {
                line_starts.push(i + 1);
            }
            i += 1;
        }
    }
    Parsed {
        source,
        tokens,
        items,
        diagnostics,
        line_starts,
    }
}

impl<'a> Parsed<'a> {
    pub fn text(&self, range: TextRange) -> &'a str {
        &self.source[range.start..range.end]
    }

    pub fn token_text(&self, index: usize) -> &'a str {
        self.text(self.tokens[index].range)
    }

    /// Reconstruct from every token, including all trivia and erroneous bytes.
    pub fn reconstruct(&self) -> String {
        self.tokens.iter().map(|t| self.text(t.range)).collect()
    }

    pub fn position(&self, offset: usize) -> Position {
        assert!(offset <= self.source.len() && self.source.is_char_boundary(offset));
        let line = self.line_starts.partition_point(|&start| start <= offset) - 1;
        let mut start = self.line_starts[line];
        if line == 0 && self.source.starts_with('\u{feff}') {
            start = start.max(3).min(offset);
        }
        Position {
            line: line + 1,
            column: self.source[start..offset].chars().count() + 1,
        }
    }

    pub fn ast(&self) -> Vec<AstItem<'a>> {
        self.ast_items(&self.items)
    }

    fn ast_items(&self, items: &[CstItem]) -> Vec<AstItem<'a>> {
        items
            .iter()
            .map(|item| match item {
                CstItem::Property {
                    key,
                    operator,
                    value,
                    ..
                } => AstItem::Property {
                    key: self.token_text(*key),
                    operator: self.token_text(*operator),
                    value: self.ast_value(value),
                },
                CstItem::Value(value) => AstItem::Value(self.ast_value(value)),
                CstItem::Error(range) => AstItem::Error(self.text(*range)),
            })
            .collect()
    }

    fn ast_value(&self, value: &CstValue) -> AstValue<'a> {
        match &value.kind {
            ValueKind::Scalar(token) => AstValue::Scalar {
                raw: self.token_text(*token),
                quoted: self.tokens[*token].kind == TokenKind::Quoted,
            },
            ValueKind::Block { items, .. } => AstValue::Block(self.ast_items(items)),
            ValueKind::Opaque => AstValue::Opaque(self.text(value.range)),
            ValueKind::Missing => AstValue::Missing,
        }
    }
}

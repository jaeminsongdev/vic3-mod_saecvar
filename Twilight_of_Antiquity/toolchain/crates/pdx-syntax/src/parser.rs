use crate::{CstItem, CstValue, Diagnostic, MAX_DEPTH, TextRange, Token, TokenKind, ValueKind};

pub(crate) fn parse_items(
    tokens: &[Token],
    end: usize,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<CstItem> {
    Parser {
        tokens,
        end,
        cursor: 0,
        diagnostics,
    }
    .items(false, 0)
    .0
}

struct Parser<'a, 'd> {
    tokens: &'a [Token],
    end: usize,
    cursor: usize,
    diagnostics: &'d mut Vec<Diagnostic>,
}

impl Parser<'_, '_> {
    fn next_at(&self, from: usize) -> Option<usize> {
        (from..self.tokens.len()).find(|&i| !self.tokens[i].kind.is_trivia())
    }

    fn error(&mut self, kind: &'static str, range: TextRange) {
        self.diagnostics.push(Diagnostic {
            rule_id: "SYN-001",
            kind,
            range,
        });
    }

    fn items(&mut self, in_block: bool, depth: usize) -> (Vec<CstItem>, Option<usize>) {
        let mut items = Vec::new();
        while let Some(index) = self.next_at(self.cursor) {
            self.cursor = index;
            let token = &self.tokens[index];
            match token.kind {
                TokenKind::CloseBrace if in_block => {
                    self.cursor += 1;
                    return (items, Some(index));
                }
                TokenKind::Atom | TokenKind::Quoted => {
                    let operator = self
                        .next_at(index + 1)
                        .filter(|&i| self.tokens[i].kind == TokenKind::Operator);
                    if let Some(operator) = operator {
                        self.cursor = operator + 1;
                        let value = self.property_value(depth);
                        let range = TextRange {
                            start: token.range.start,
                            end: value.range.end.max(self.tokens[operator].range.end),
                        };
                        items.push(CstItem::Property {
                            key: index,
                            operator,
                            value,
                            range,
                        });
                    } else {
                        items.push(CstItem::Value(self.value(depth)));
                    }
                }
                TokenKind::OpenBrace => items.push(CstItem::Value(self.value(depth))),
                _ => {
                    let range = token.range;
                    if token.kind != TokenKind::Unsupported {
                        self.error(
                            if token.kind == TokenKind::CloseBrace {
                                "unexpected_closing_brace"
                            } else {
                                "unexpected_operator"
                            },
                            range,
                        );
                    }
                    items.push(CstItem::Error(range));
                    self.cursor += 1;
                }
            }
        }
        (items, None)
    }

    fn property_value(&mut self, depth: usize) -> CstValue {
        if let Some(index) = self.next_at(self.cursor) {
            let kind = self.tokens[index].kind;
            let followed_by_operator = matches!(kind, TokenKind::Atom | TokenKind::Quoted)
                && self
                    .next_at(index + 1)
                    .is_some_and(|i| self.tokens[i].kind == TokenKind::Operator);
            if matches!(
                kind,
                TokenKind::Atom | TokenKind::Quoted | TokenKind::OpenBrace
            ) && !followed_by_operator
            {
                self.cursor = index;
                return self.value(depth);
            }
            let range = TextRange {
                start: self.tokens[index].range.start,
                end: self.tokens[index].range.start,
            };
            self.error("missing_value", range);
            CstValue {
                kind: ValueKind::Missing,
                range,
            }
        } else {
            let range = TextRange {
                start: self.end,
                end: self.end,
            };
            self.error("missing_value", range);
            CstValue {
                kind: ValueKind::Missing,
                range,
            }
        }
    }

    fn value(&mut self, depth: usize) -> CstValue {
        let index = self.cursor;
        self.cursor += 1;
        let token = &self.tokens[index];
        if token.kind != TokenKind::OpenBrace {
            if token.kind == TokenKind::Atom
                && let Some(block) = self
                    .next_at(self.cursor)
                    .filter(|&i| self.tokens[i].kind == TokenKind::OpenBrace)
            {
                let start = token.range.start;
                self.cursor = block;
                let value = self.value(depth);
                let range = TextRange {
                    start,
                    end: value.range.end,
                };
                self.diagnostics.push(Diagnostic {
                    rule_id: "SYN-002",
                    kind: "unsupported_typed_value",
                    range,
                });
                return CstValue {
                    kind: ValueKind::Opaque,
                    range,
                };
            }
            return CstValue {
                kind: ValueKind::Scalar(index),
                range: token.range,
            };
        }
        if depth >= MAX_DEPTH {
            let range = self.skip_deep_block(index);
            self.diagnostics.push(Diagnostic {
                rule_id: "SYN-003",
                kind: "nesting_limit",
                range,
            });
            return CstValue {
                kind: ValueKind::Opaque,
                range,
            };
        }
        let (items, close) = self.items(true, depth + 1);
        let end = close.map_or(self.end, |i| self.tokens[i].range.end);
        if close.is_none() {
            self.error("unclosed_block", token.range);
        }
        CstValue {
            kind: ValueKind::Block {
                open: index,
                items,
                close,
            },
            range: TextRange {
                start: token.range.start,
                end,
            },
        }
    }

    fn skip_deep_block(&mut self, open: usize) -> TextRange {
        let mut balance = 1;
        while self.cursor < self.tokens.len() {
            let index = self.cursor;
            self.cursor += 1;
            match self.tokens[index].kind {
                TokenKind::OpenBrace => balance += 1,
                TokenKind::CloseBrace => balance -= 1,
                _ => {}
            }
            if balance == 0 {
                return TextRange {
                    start: self.tokens[open].range.start,
                    end: self.tokens[index].range.end,
                };
            }
        }
        self.error("unclosed_block", self.tokens[open].range);
        TextRange {
            start: self.tokens[open].range.start,
            end: self.end,
        }
    }
}

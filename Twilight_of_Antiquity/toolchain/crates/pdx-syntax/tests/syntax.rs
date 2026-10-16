use saecvar_pdx_syntax::{AstItem, AstValue, CstItem, MAX_DEPTH, Position, TokenKind, parse};

fn lossless(source: &str) {
    let parsed = parse(source);
    assert_eq!(parsed.reconstruct(), source);
    let mut next = 0;
    for token in &parsed.tokens {
        assert_eq!(token.range.start, next);
        assert!(token.range.end > next);
        assert!(source.is_char_boundary(token.range.start));
        assert!(source.is_char_boundary(token.range.end));
        next = token.range.end;
    }
    assert_eq!(next, source.len());
    for diagnostic in &parsed.diagnostics {
        assert!(
            diagnostic.range.start <= diagnostic.range.end && diagnostic.range.end <= source.len()
        );
        parsed.position(diagnostic.range.start);
        parsed.position(diagnostic.range.end);
    }
}

#[test]
fn synthetic_nested_fixture_preserves_duplicates_and_lists() {
    let source = include_str!("../../../../tests/fixtures/pdx/basic.txt");
    let parsed = parse(source);
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    lossless(source);
    let ast = parsed.ast();
    let AstItem::Property {
        key: "probe",
        value: AstValue::Block(items),
        ..
    } = &ast[1]
    else {
        panic!("expected probe block");
    };
    let keys: Vec<_> = items
        .iter()
        .filter_map(|i| match i {
            AstItem::Property { key, .. } => Some(*key),
            _ => None,
        })
        .collect();
    assert_eq!(&keys[..3], &["name", "key", "key"]);
    let AstItem::Property {
        value: AstValue::Block(list),
        ..
    } = &items[3]
    else {
        panic!("expected list");
    };
    assert_eq!(list.len(), 3);
}

#[test]
fn recovery_keeps_subsequent_properties_and_reports_locations() {
    let source = include_str!("../../../../tests/fixtures/pdx/recovery.txt");
    let parsed = parse(source);
    lossless(source);
    let kinds: Vec<_> = parsed.diagnostics.iter().map(|d| d.kind).collect();
    for expected in [
        "missing_value",
        "unterminated_string",
        "unexpected_closing_brace",
        "unclosed_block",
    ] {
        assert!(kinds.contains(&expected), "{expected}: {kinds:?}");
    }
    let ast = parsed.ast();
    assert!(
        ast.iter()
            .any(|item| matches!(item, AstItem::Property { key: "healthy", .. }))
    );
    assert!(ast.iter().any(|item| matches!(
        item,
        AstItem::Property {
            key: "after_string",
            ..
        }
    )));
    let missing = parsed
        .diagnostics
        .iter()
        .find(|d| d.kind == "missing_value")
        .unwrap();
    assert_eq!(
        parsed.position(missing.range.start),
        Position { line: 2, column: 1 }
    );
}

#[test]
fn bom_crlf_unicode_and_lone_cr_positions() {
    let source = "\u{feff}이름 = 값\r\n다음 = {\r끝 = yes\r}";
    let parsed = parse(source);
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.tokens[0].kind, TokenKind::Bom);
    assert_eq!(parsed.position(3), Position { line: 1, column: 1 });
    assert_eq!(
        parsed.position(source.find('값').unwrap()),
        Position { line: 1, column: 6 }
    );
    assert_eq!(
        parsed.position(source.find('끝').unwrap()),
        Position { line: 3, column: 1 }
    );
    lossless(source);
}

#[test]
fn quoted_comment_braces_and_escapes_are_not_structure() {
    let source = r#"value = "a \" # { } \\ z" # } comment
next = {}"#;
    let parsed = parse(source);
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert_eq!(parsed.items.len(), 2);
    lossless(source);
}

#[test]
fn all_initial_operator_spellings_are_preserved() {
    let source = "a = 1 b == 2 c != 3 d < 4 e <= 5 f > 6 g >= 7 h ?= 8";
    let parsed = parse(source);
    assert!(parsed.diagnostics.is_empty());
    let ops: Vec<_> = parsed
        .tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| t.kind == TokenKind::Operator)
        .map(|(i, _)| parsed.token_text(i))
        .collect();
    assert_eq!(ops, ["=", "==", "!=", "<", "<=", ">", ">=", "?="]);
}

#[test]
fn unsupported_math_is_never_silent_success() {
    let source = include_str!("../../../../tests/fixtures/pdx/unsupported.txt");
    let parsed = parse(source);
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.verification_status() == "UNSUPPORTED_RULE")
    );
    lossless(source);
}

#[test]
fn typed_values_are_opaque_and_explicitly_unsupported() {
    let source = "color = rgb { 1 2 3 } next = yes";
    let parsed = parse(source);
    assert!(
        parsed
            .diagnostics
            .iter()
            .any(|d| d.kind == "unsupported_typed_value")
    );
    let ast = parsed.ast();
    assert!(matches!(
        ast[0],
        AstItem::Property {
            value: AstValue::Opaque("rgb { 1 2 3 }"),
            ..
        }
    ));
    assert!(matches!(ast[1], AstItem::Property { key: "next", .. }));
    lossless(source);
}

#[test]
fn missing_value_before_close_does_not_eat_the_close() {
    let parsed = parse("outer = { bad = } next = yes");
    assert_eq!(parsed.diagnostics.len(), 1);
    assert_eq!(parsed.diagnostics[0].kind, "missing_value");
    assert_eq!(parsed.items.len(), 2);
}

#[test]
fn excessive_nesting_is_bounded_and_recovers_the_tail() {
    let source = format!(
        "{}yes{} tail = yes",
        "{ ".repeat(MAX_DEPTH + 1000),
        " }".repeat(MAX_DEPTH + 1000)
    );
    let parsed = parse(&source);
    assert!(parsed.diagnostics.iter().any(|d| d.kind == "nesting_limit"));
    assert!(matches!(
        parsed.items.last(),
        Some(CstItem::Property { .. })
    ));
    lossless(&source);
    let _ = parsed.ast();
}

#[test]
fn empty_comments_and_each_truncated_prefix_are_lossless() {
    for source in [
        "", "#", "\u{feff}", " ", "a =", "{", "\"", "\"\\", "}", "?", "\0",
    ] {
        lossless(source);
    }
    let source = "\u{feff}國 = { text = \"value\\\"x\" list = { one two } }";
    for (offset, _) in source.char_indices() {
        lossless(&source[..offset]);
    }
}

#[test]
fn deterministic_generated_inputs_terminate_and_round_trip() {
    let alphabet = [
        'a', ' ', '\n', '\r', '{', '}', '"', '\\', '#', '=', '?', '!', '<', '>', '[', ']', '한',
        '\u{feff}', '\0',
    ];
    let mut seed = 0x5aec_a123_u64;
    for iteration in 0..4000 {
        let mut source = String::new();
        for _ in 0..iteration % 128 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            source.push(alphabet[(seed >> 32) as usize % alphabet.len()]);
        }
        lossless(&source);
    }
}

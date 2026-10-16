use saecvar_pdx_syntax::parse;
use std::{fmt::Write, fs, path::Path, process::ExitCode};

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                write!(out, "\\u{:04x}", c as u32).unwrap();
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn run(path: &Path) -> ExitCode {
    if !path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
    {
        eprintln!("unsupported dialect: the M1 parse command accepts .txt text scripts only");
        return ExitCode::from(2);
    }
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("syntax input failed (UTF-8 text required): {error}");
            return ExitCode::from(2);
        }
    };
    let parsed = parse(&source);
    println!(
        "{{\"schema\":\"saecvar.syntax.v0\",\"file\":{},\"dialect\":\"text-script-subset\",\"engine_validation\":\"NOT_RUN\",\"semantic_validation\":\"NOT_IMPLEMENTED\",\"source_bytes\":{},\"tokens\":{},\"root_items\":{},\"diagnostics\":[",
        json_string(&path.to_string_lossy()),
        source.len(),
        parsed.tokens.len(),
        parsed.items.len()
    );
    for (index, diagnostic) in parsed.diagnostics.iter().enumerate() {
        let start = parsed.position(diagnostic.range.start);
        let end = parsed.position(diagnostic.range.end);
        println!(
            "{}{{\"rule_id\":{},\"kind\":{},\"verification_status\":{},\"range\":{{\"start_byte\":{},\"end_byte\":{},\"start\":{{\"line\":{},\"column\":{}}},\"end\":{{\"line\":{},\"column\":{}}}}}}}",
            if index == 0 { "" } else { "," },
            json_string(diagnostic.rule_id),
            json_string(diagnostic.kind),
            json_string(diagnostic.verification_status()),
            diagnostic.range.start,
            diagnostic.range.end,
            start.line,
            start.column,
            end.line,
            end.column
        );
    }
    println!("]}}");
    if parsed.diagnostics.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

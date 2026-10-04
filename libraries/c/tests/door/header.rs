//! The committed header is the export authority, independent of the DLL.
use std::collections::BTreeSet;

fn block_comment(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<(), &'static str> {
    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'/') {
            chars.next();
            return Ok(());
        }
    }
    Err("unterminated header comment")
}

/// Read plain C function declarations; refuse ambiguous or incomplete input.
pub(crate) fn declarations(header: &str) -> Result<Vec<String>, &'static str> {
    let mut code = String::new();
    let mut chars = header.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            block_comment(&mut chars)?;
            code.push(' ');
        } else if ch == '/' && chars.peek() == Some(&'/') {
            chars.by_ref().find(|ch| *ch == '\n');
            code.push('\n');
        } else {
            code.push(ch);
        }
    }
    let code = code
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    let mut names = BTreeSet::new();
    for (at, _) in code.match_indices("thinkthen_") {
        let tail = code.get(at..).ok_or("invalid declaration offset")?;
        let end = tail
            .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
            .unwrap_or(tail.len());
        let name = tail.get(..end).ok_or("invalid name")?;
        let after = tail.get(end..).ok_or("invalid declaration")?.trim_start();
        if !after.starts_with('(') {
            continue;
        }
        let close = after.find(')').ok_or("malformed header declaration")?;
        let arguments = after.get(1..close).ok_or("malformed header arguments")?;
        if arguments.trim().is_empty()
            || arguments.contains(['(', ')', ';', '{'])
            || !after
                .get(close + 1..)
                .ok_or("malformed header declaration")?
                .trim_start()
                .starts_with(';')
        {
            return Err("malformed header declaration");
        }
        let before = code.get(..at).ok_or("invalid declaration prefix")?;
        let prefix = before
            .rsplit([';', '{', '}'])
            .next()
            .unwrap_or_default()
            .trim();
        if prefix.is_empty() || prefix.contains(['(', ')', '=']) || name == "thinkthen_" {
            return Err("malformed header declaration");
        }
        if !names.insert(name.to_owned()) {
            return Err("duplicate header declaration");
        }
    }
    if names.is_empty() {
        return Err("header declares no functions");
    }
    Ok(names.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::declarations;
    #[test]
    fn independent_declarations_and_refusals() {
        assert_eq!(
            declarations(
                "/* thinkthen_hidden(void); */\nvoid thinkthen_z(void);\nconst char *thinkthen_a(int n);"
            ),
            Ok(vec!["thinkthen_a".into(), "thinkthen_z".into()])
        );
        for (input, error) in [
            ("/*", "unterminated header comment"),
            (
                "void thinkthen_a(void); void thinkthen_a(void);",
                "duplicate header declaration",
            ),
            ("void thinkthen_a(void)", "malformed header declaration"),
            ("void thinkthen_a();", "malformed header declaration"),
            ("void thinkthen_a((void));", "malformed header declaration"),
        ] {
            assert_eq!(declarations(input), Err(error));
        }
    }
}

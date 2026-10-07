//! Bounded response storage permission; no header text leaves this parser.

/// Malformed or excessive instructions conservatively forbid storage.
pub(crate) fn permits<'a>(fields: impl IntoIterator<Item = &'a [u8]>) -> bool {
    let mut bytes = 0;
    let mut directives = 0;
    for (at, field) in fields.into_iter().enumerate() {
        bytes += field.len();
        if at >= 8 || bytes > 8192 {
            return false;
        }
        if !valid_field(field, &mut directives) {
            return false;
        }
    }
    true
}

fn valid_field(field: &[u8], directives: &mut usize) -> bool {
    let mut remaining = field;
    loop {
        ows(&mut remaining);
        let Some(name) = token(&mut remaining) else {
            return false;
        };
        *directives += 1;
        if *directives > 64 || name.eq_ignore_ascii_case(b"no-store") {
            return false;
        }
        ows(&mut remaining);
        if take(&mut remaining, b'=') {
            ows(&mut remaining);
            if !value(&mut remaining) {
                return false;
            }
            ows(&mut remaining);
        }
        if remaining.is_empty() {
            break;
        }
        if !take(&mut remaining, b',') {
            return false;
        }
    }
    true
}

fn value(remaining: &mut &[u8]) -> bool {
    if remaining.first() == Some(&b'"') {
        quoted(remaining)
    } else {
        token(remaining).is_some()
    }
}

fn token<'a>(remaining: &mut &'a [u8]) -> Option<&'a [u8]> {
    let count = remaining
        .iter()
        .take_while(|&&byte| token_byte(byte))
        .count();
    let (held, rest) = remaining.split_at_checked(count)?;
    *remaining = rest;
    (!held.is_empty()).then_some(held)
}

fn token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

fn ows(remaining: &mut &[u8]) {
    while matches!(remaining.first(), Some(b' ' | b'\t')) {
        let _taken = remaining.split_off_first();
    }
}

fn take(remaining: &mut &[u8], byte: u8) -> bool {
    if remaining.first() == Some(&byte) {
        let _taken = remaining.split_off_first();
        true
    } else {
        false
    }
}

fn quoted(remaining: &mut &[u8]) -> bool {
    if !take(remaining, b'"') {
        return false;
    }
    while let Some(&byte) = remaining.split_off_first() {
        match byte {
            b'"' => return true,
            b'\\' => {
                if !remaining
                    .split_off_first()
                    .is_some_and(|&next| quoted_byte(next))
                {
                    return false;
                }
            }
            byte if quoted_byte(byte) => {}
            _ => return false,
        }
    }
    false
}

fn quoted_byte(byte: u8) -> bool {
    byte == b'\t' || byte >= b' ' && byte != 127
}

#[cfg(test)]
mod tests {
    use super::permits;

    #[test]
    fn storage_permission_matches_directive_names_and_validates_the_whole_grammar() {
        for (value, expected) in [
            ("max-age=30", true),
            ("private=\"no-store\"", true),
            ("no-store-extra, x=\"a,b\\\"c\"", true),
            (" \tMax-Age \t=\t 30, private", true),
            ("No-StOrE", false),
            ("no-store=whatever", false),
            ("private=\"unterminated", false),
            ("private=\"bad\\", false),
            ("private=\"bad\u{007f}\"", false),
            ("name=value extra", false),
            ("name==value", false),
            ("name=", false),
            ("name,", false),
            (",name", false),
            ("na(me", false),
            ("", false),
            ("name\r\nnext", false),
        ] {
            assert_eq!(permits([value.as_bytes()]), expected, "{value:?}");
        }
        assert!(permits([]));
        assert!(permits([b"max-age=30".as_slice(), b"private"]));
        assert!(!permits([b"max-age=30".as_slice(), b"no-store"]));
    }

    #[test]
    fn fields_bytes_and_directives_each_have_an_inclusive_bound() {
        assert!(permits(std::iter::repeat_n(b"x".as_slice(), 8)));
        assert!(!permits(std::iter::repeat_n(b"x".as_slice(), 9)));
        let longest = "x".repeat(8192);
        assert!(permits([longest.as_bytes()]));
        let excessive = "x".repeat(8193);
        assert!(!permits([excessive.as_bytes()]));
        assert!(!permits([longest.as_bytes(), b"x"]));
        let admitted = std::iter::repeat_n("x", 64).collect::<Vec<_>>().join(",");
        assert!(permits([admitted.as_bytes()]));
        assert!(!permits([admitted.as_bytes(), b"x"]));
    }
}

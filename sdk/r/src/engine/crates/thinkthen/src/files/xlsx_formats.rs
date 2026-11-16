//! Select the numeric number-format section before classifying date or clock tokens.

use super::FileError;
use super::xlsx::invalid;

#[derive(Clone, Copy, Default)]
pub(super) enum Format {
    #[default]
    Stored,
    Date,
    Time,
    DateTime,
    Duration,
}

#[derive(Clone, Default)]
pub(super) struct NumberFormat {
    sections: Vec<Section>,
}

#[derive(Clone)]
struct Section {
    format: Format,
    condition: Option<Condition>,
}

#[derive(Clone)]
enum Condition {
    Less(f64),
    LessEqual(f64),
    Greater(f64),
    GreaterEqual(f64),
    Equal(f64),
    NotEqual(f64),
}

impl Condition {
    fn read(token: &str) -> Result<Option<Self>, FileError> {
        let (operator, value) = token.split_at(
            token
                .find(|c| !matches!(c, '<' | '>' | '='))
                .unwrap_or(token.len()),
        );
        if operator.is_empty() {
            return Ok(None);
        }
        let threshold = value
            .parse::<f64>()
            .map_err(|_| invalid("number format condition has an invalid value"))?;
        if !threshold.is_finite() {
            return Err(invalid("number format condition must have a finite value"));
        }
        Ok(Some(match operator {
            "<" => Self::Less(threshold),
            "<=" => Self::LessEqual(threshold),
            ">" => Self::Greater(threshold),
            ">=" => Self::GreaterEqual(threshold),
            "=" => Self::Equal(threshold),
            "<>" => Self::NotEqual(threshold),
            _ => return Err(invalid("number format condition has an invalid comparison")),
        }))
    }

    fn matches(&self, value: f64) -> bool {
        match *self {
            Self::Less(n) => value < n,
            Self::LessEqual(n) => value <= n,
            Self::Greater(n) => value > n,
            Self::GreaterEqual(n) => value >= n,
            Self::Equal(n) => value == n,
            Self::NotEqual(n) => value != n,
        }
    }
}

impl NumberFormat {
    pub(super) fn builtin(format: Format) -> Self {
        Self {
            sections: vec![Section {
                format,
                condition: None,
            }],
        }
    }

    pub(super) fn read(code: &str) -> Result<Self, FileError> {
        let mut sections = Vec::new();
        let mut current = String::new();
        let mut quoted = false;
        let mut escaped = false;
        let mut bracketed = false;
        for c in code.chars() {
            if !escaped && !quoted && !bracketed && c == ';' {
                if sections.len() == 3 {
                    return Err(invalid("number format has more than four sections"));
                }
                sections.push(classify(&current)?);
                current.clear();
            } else {
                current.push(c);
                if escaped {
                    escaped = false;
                } else if matches!(c, '\\' | '_' | '*') && !quoted {
                    escaped = true;
                } else if c == '"' && !bracketed {
                    quoted = !quoted;
                } else if !quoted {
                    if c == '[' {
                        bracketed = true;
                    }
                    if c == ']' {
                        bracketed = false;
                    }
                }
            }
        }
        if quoted || escaped || bracketed {
            return Err(invalid("number format sections are malformed"));
        }
        sections.push(classify(&current)?);
        if sections
            .iter()
            .skip(2)
            .any(|section| section.condition.is_some())
        {
            return Err(invalid(
                "number format conditions must be in the first two sections",
            ));
        }
        Ok(Self { sections })
    }

    pub(super) fn select(&self, value: f64) -> Format {
        self.sections
            .iter()
            .take(3)
            .enumerate()
            .find(|(index, section)| {
                section.condition.as_ref().map_or_else(
                    || match (self.sections.len(), index) {
                        (1, _) | (_, 2) => true,
                        (2, 0) => value >= 0.0,
                        (2, 1) => true,
                        (_, 0) => value > 0.0,
                        _ => value < 0.0,
                    },
                    |condition| condition.matches(value),
                )
            })
            .map_or(Format::Stored, |(_, section)| section.format)
    }
}

fn classify(code: &str) -> Result<Section, FileError> {
    let mut characters = code.chars().peekable();
    let (mut date, mut time, mut elapsed) = (false, false, false);
    let mut pending_month = false;
    let mut previous = None;
    let mut condition = None;
    while let Some(character) = characters.next() {
        match character.to_ascii_lowercase() {
            '"' => {
                for next in characters.by_ref() {
                    if next == '"' {
                        break;
                    }
                }
            }
            '\\' | '_' | '*' => {
                characters.next();
            }
            '[' => {
                let mut token = String::new();
                for next in characters.by_ref() {
                    if next == ']' {
                        break;
                    }
                    token.push(next.to_ascii_lowercase());
                }
                if let Some(parsed) = Condition::read(&token)?
                    && condition.replace(parsed).is_some()
                {
                    return Err(invalid("number format section has multiple conditions"));
                }
                elapsed |= matches!(token.as_str(), "h" | "hh" | "m" | "mm" | "s" | "ss");
            }
            'a' if characters
                .clone()
                .take(4)
                .collect::<String>()
                .eq_ignore_ascii_case("m/pm") =>
            {
                characters.nth(3);
            }
            token @ ('y' | 'd' | 'h' | 's' | 'm') => {
                let mut count = 1;
                while characters
                    .peek()
                    .is_some_and(|next| next.to_ascii_lowercase() == token)
                {
                    characters.next();
                    count += 1;
                }
                if pending_month {
                    if token == 's' {
                        time = true;
                    } else {
                        date = true;
                    }
                }
                pending_month = false;
                match token {
                    'y' | 'd' => date = true,
                    'h' | 's' => time = true,
                    'm' if count <= 2 && previous == Some('h') => time = true,
                    'm' if count <= 2 => pending_month = true,
                    'm' => date = true,
                    _ => {}
                }
                previous = Some(token);
            }
            _ => {}
        }
    }
    date |= pending_month;
    let format = if elapsed {
        Format::Duration
    } else if date && time {
        Format::DateTime
    } else if date {
        Format::Date
    } else if time {
        Format::Time
    } else {
        Format::Stored
    };
    Ok(Section { format, condition })
}

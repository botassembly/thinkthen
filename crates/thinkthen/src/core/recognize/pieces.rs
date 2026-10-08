//! The step-1 splitter: white space separates pieces, each punctuation or
//! symbol is a piece of its own, and a run of marks joins the piece before it.

use super::categories::{is_joiner, is_mark};

/// One piece's place, in UTF-8 bytes and in Unicode scalar values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Piece {
    pub(crate) byte_start: usize,
    pub(crate) byte_end: usize,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Piece {
    /// Whether the piece starts with punctuation or a symbol.
    pub(crate) fn is_mark(&self, text: &str) -> bool {
        text.get(self.byte_start..)
            .and_then(|rest| rest.chars().next())
            .is_some_and(is_mark)
    }
}

/// Split `text` into pieces, with offsets into the original text.
pub(crate) fn pieces(text: &str) -> Vec<Piece> {
    let mut pieces: Vec<Piece> = Vec::new();
    let mut open: Option<(usize, usize)> = None;
    let mut scalar = 0;
    let mut prefix = true;
    for (byte, character) in text.char_indices() {
        let next = byte + character.len_utf8();
        if prefix
            && matches!(
                character,
                '\u{feff}' | '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{2060}'
            )
        {
            scalar += 1;
            continue;
        }
        prefix = false;
        if character.is_whitespace() || is_mark(character) {
            close(&mut pieces, open.take(), byte, scalar);
            if !character.is_whitespace() {
                pieces.push(Piece {
                    byte_start: byte,
                    byte_end: next,
                    start: scalar,
                    end: scalar + 1,
                });
            }
        } else if open.is_none() {
            match pieces.last_mut() {
                Some(last) if is_joiner(character) && last.byte_end == byte => {
                    last.byte_end = next;
                    last.end = scalar + 1;
                }
                _ => open = Some((byte, scalar)),
            }
        }
        scalar += 1;
    }
    close(&mut pieces, open, text.len(), scalar);
    pieces
}

fn close(pieces: &mut Vec<Piece>, open: Option<(usize, usize)>, byte: usize, scalar: usize) {
    if let Some((byte_start, start)) = open {
        pieces.push(Piece {
            byte_start,
            byte_end: byte,
            start,
            end: scalar,
        });
    }
}

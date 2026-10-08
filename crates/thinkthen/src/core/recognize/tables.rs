//! Ticket 0147's five edge-case tables: pieces, decode, score, edge options
//! and requests. Each row is an input and its exact output.

use super::{
    Asked, NameOdds, Odds, TagRow, decode, edge_options, edge_question, evidence, name_groups,
    pieces, settle, step_one_groups, step_two_questions,
};
use crate::core::Question;

type Places = &'static [(usize, usize)];

/// A row name, a found name's pieces, and the edge option labels it offers.
type Offered = (&'static str, (usize, usize), &'static [&'static str]);

fn places(text: &str) -> Vec<(usize, usize)> {
    pieces(text)
        .iter()
        .map(|piece| (piece.start, piece.end))
        .collect()
}

#[test]
fn pieces_split_at_white_space_and_marks_and_join_combining_runs() {
    let rows: [(&str, Places); 13] = [
        ("George Harrison's", &[(0, 6), (7, 15), (15, 16), (16, 17)]),
        ("U.S.", &[(0, 1), (1, 2), (2, 3), (3, 4)]),
        ("Help!", &[(0, 4), (4, 5)]),
        ("London-based", &[(0, 6), (6, 7), (7, 12)]),
        ("Drake\u{2bc}s", &[(0, 7)]),
        ("e\u{301}", &[(0, 2)]),
        ("e\u{301}\u{302}", &[(0, 3)]),
        ("\u{2665}\u{fe0f}", &[(0, 2)]),
        (".\u{301}", &[(0, 2)]),
        ("\u{1f468}\u{200d}\u{1f469}", &[(0, 2), (2, 3)]),
        ("x \u{301}a", &[(0, 1), (2, 4)]),
        ("\u{1f44d}\u{1f3fd}", &[(0, 1), (1, 2)]),
        ("Hi \"!\" ok", &[(0, 2), (3, 4), (4, 5), (5, 6), (7, 9)]),
    ];
    for (text, expected) in rows {
        assert_eq!(places(text), expected, "{text:?}");
    }
}

const O: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 1.0];

#[test]
fn decode_keeps_the_most_likely_valid_sequence() {
    let mut across = vec![O; 41];
    across[39] = [0.9, 0.0, 0.0, 0.05, 0.05];
    across[40] = [0.0, 0.05, 0.9, 0.0, 0.05];
    let rows: [(&str, Vec<TagRow>, Places); 5] = [
        (
            "a lone BEGIN at the end",
            vec![O, [0.9, 0.0, 0.0, 0.06, 0.04]],
            &[(1, 1)],
        ),
        (
            "BEGIN then OUT",
            vec![[0.8, 0.0, 0.0, 0.2, 0.0], [0.0, 0.0, 0.3, 0.0, 0.7]],
            &[(0, 1)],
        ),
        (
            "equal scores: Ada",
            vec![[0.0, 0.0, 0.0, 0.5, 0.5]],
            &[(0, 0)],
        ),
        ("a name across a request edge", across, &[(39, 40)]),
        (
            "the floor",
            vec![
                [0.99, 0.0, 0.0, 0.0, 0.001],
                [0.0, 0.0, 0.0, 0.0, 0.001],
                [0.0, 0.0, 0.99, 0.0, 0.001],
            ],
            &[(0, 2)],
        ),
    ];
    for (row, tags, expected) in rows {
        assert_eq!(decode(&tags), expected, "{row}");
    }
}

/// One scored name: its tag rows, its decoded stretch, its kind odds (`None`
/// with no kinds), its edge options and odds, and the names it prints.
struct Scored {
    row: &'static str,
    text: &'static str,
    tags: Vec<[f64; 5]>,
    found: Vec<(usize, usize)>,
    kinds: Vec<Option<f64>>,
    edges: Vec<(usize, usize)>,
    printed: &'static [(&'static str, f64)],
}

fn scored(case: &Scored) -> Vec<(String, f64)> {
    let asked: Vec<(Asked, NameOdds)> = case
        .kinds
        .iter()
        .map(|kind| {
            let widened = case.edges.len() > 1;
            let pick = |place: usize| f64::from(u8::from(place == case.edges.len() - 1));
            let edges = widened.then(|| {
                Odds(
                    (0..case.edges.len())
                        .map(|place| (format!("option {place}"), pick(place)))
                        .collect(),
                )
            });
            let kinds = kind.map(|value| {
                Odds(vec![
                    ("person".to_owned(), value),
                    ("none of these".to_owned(), 1.0 - value),
                ])
            });
            let held = Asked {
                kind: kind.is_some(),
                edges: case.edges.clone(),
            };
            (
                held,
                NameOdds {
                    start: 0,
                    end: 0,
                    kinds,
                    edges,
                },
            )
        })
        .collect();
    settle(
        case.text,
        &pieces(case.text),
        &case.tags,
        &case.found,
        &asked,
        0.5,
    )
    .into_iter()
    .map(|name| (name.text, name.strength))
    .collect()
}

#[test]
fn strength_is_p_kind_times_p_span_at_four_places_and_cuts_on_the_printed_value() {
    let single = |value: f64| vec![[0.0, 0.0, 0.0, value, 1.0 - value]];
    let one = |row, tags, kind, printed| Scored {
        row,
        text: "Ada",
        tags,
        found: vec![(0, 0)],
        kinds: vec![kind],
        edges: vec![(0, 0)],
        printed,
    };
    let rows = [
        one("worked", single(0.9), Some(0.8), &[("Ada", 0.72)]),
        Scored {
            row: "three pieces",
            text: "Ada Byron King",
            tags: vec![
                [0.9, 0.0, 0.0, 0.05, 0.05],
                [0.0, 0.8, 0.1, 0.0, 0.1],
                [0.0, 0.1, 0.85, 0.0, 0.05],
            ],
            found: vec![(0, 2)],
            kinds: vec![Some(0.9)],
            edges: vec![(0, 2)],
            printed: &[("Ada Byron King", 0.8927)],
        },
        Scored {
            row: "not the only likely path",
            text: "Ada Lovelace",
            tags: vec![[0.6, 0.0, 0.0, 0.4, 0.0], [0.0, 0.0, 0.6, 0.0, 0.4]],
            found: vec![(0, 1)],
            kinds: vec![Some(0.9)],
            edges: vec![(0, 1)],
            printed: &[("Ada Lovelace", 0.6231)],
        },
        Scored {
            row: "no kinds",
            text: "Ada Lovelace",
            tags: vec![[0.7, 0.0, 0.0, 0.1, 0.2], [0.0, 0.1, 0.8, 0.05, 0.05]],
            found: vec![(0, 1)],
            kinds: vec![None],
            edges: vec![(0, 1)],
            printed: &[("Ada Lovelace", 0.9492)],
        },
        one("at 0.5", single(0.5), Some(1.0), &[("Ada", 0.5)]),
        one("at 0.49", single(0.5), Some(0.98), &[]),
        one("raw 0.49996", single(0.5), Some(0.99992), &[("Ada", 0.5)]),
        one("raw 0.49994", single(0.5), Some(0.99988), &[]),
        Scored {
            row: "widened by the edge pick",
            text: "Help!",
            tags: vec![[0.0, 0.0, 0.0, 0.8, 0.2], [0.0, 0.0, 0.0, 0.1, 0.9]],
            found: vec![(0, 0)],
            kinds: vec![Some(0.9)],
            edges: vec![(0, 0), (0, 1)],
            printed: &[("Help!", 0.72)],
        },
        Scored {
            row: "two names left on one stretch",
            text: "!!",
            tags: vec![[0.0, 0.0, 0.0, 0.9, 0.1], [0.0, 0.0, 0.0, 0.8, 0.2]],
            found: vec![(0, 0), (1, 1)],
            kinds: vec![Some(0.6), Some(0.9)],
            edges: vec![(0, 0), (0, 1)],
            printed: &[("!!", 0.72)],
        },
    ];
    for case in rows {
        let expected: Vec<(String, f64)> = case
            .printed
            .iter()
            .map(|(text, strength)| ((*text).to_owned(), *strength))
            .collect();
        assert_eq!(scored(&case), expected, "{}", case.row);
    }
}

fn option_texts(text: &str, name: (usize, usize)) -> Vec<String> {
    let pieces = pieces(text);
    edge_options(text, &pieces, name)
        .into_iter()
        .map(|(first, last)| text[pieces[first].byte_start..pieces[last].byte_end].to_owned())
        .collect()
}

#[test]
fn edge_options_come_from_touching_marks_alone() {
    let rows: [Offered; 4] = [
        (
            "say \"Acme!\" now",
            (1, 3),
            &["\"Acme!", "\"Acme!\"", "\"Acme", "Acme!"],
        ),
        ("Ada met Bob", (0, 0), &["Ada"]),
        ("Help!", (0, 0), &["Help", "Help!"]),
        ("Hi \"!\" ok", (2, 2), &["!", "!\"", "\"!"]),
    ];
    for (text, name, expected) in rows {
        assert_eq!(option_texts(text, name), expected, "{text}");
    }
    let labelled: [Offered; 4] = [
        ("x ... y", (2, 2), &[".", "..", ".. "]),
        ("Maria\nChen.", (0, 1), &["Maria Chen", "Maria Chen."]),
        ("Maria\tChen!", (0, 1), &["Maria Chen", "Maria Chen!"]),
        ("M\u{1b}ia Chen.", (0, 1), &["M ia Chen", "M ia Chen."]),
    ];
    for (text, name, expected) in labelled {
        let pieces = pieces(text);
        let options = edge_options(text, &pieces, name);
        let Question::Choose {
            options: labels, ..
        } = edge_question(text, &pieces, &options, None).unwrap()
        else {
            panic!("the edge question is a choice");
        };
        assert_eq!(labels.names().collect::<Vec<_>>(), expected, "{text:?}");
    }
    // A stretch of control characters alone would label blank: no edge question.
    let blank: [(&str, (usize, usize)); 4] = [
        ("a \u{1b}! b", (1, 1)),
        ("a \u{0}. b", (1, 1)),
        ("a .\u{7f} b", (1, 2)),
        ("a .\u{7f} b", (2, 2)),
    ];
    for (text, name) in blank {
        let pieces = pieces(text);
        assert!(edge_options(text, &pieces, name).len() > 1, "{text:?}");
        let asked = step_two_questions(text, &pieces, &[name], 0..1, &[], None).unwrap();
        let kept = vec![Asked {
            kind: false,
            edges: vec![name],
        }];
        assert_eq!(asked, (Vec::new(), kept), "{text:?}");
    }
}

#[test]
fn requests_hold_forty_pieces_and_show_six_each_side() {
    let words = |count: usize| vec!["a"; count].join(" ");
    let (forty, forty_one) = (words(40), words(41));
    let (short, long) = (pieces(&forty), pieces(&forty_one));
    assert_eq!(forty.len(), 79);
    assert_eq!(step_one_groups(40), std::slice::from_ref(&(0..40)));
    assert_eq!(evidence(&short, 0, 39), 0..79);
    assert_eq!(step_one_groups(41), [0..40, 40..41]);
    assert_eq!(evidence(&long, 0, 39), 0..81);
    assert_eq!(evidence(&long, 40, 40), 68..81);
    let named = |pieces: &[super::Piece], names: &[(usize, usize)]| {
        name_groups(names)
            .into_iter()
            .map(|group| {
                let first = names[group.start].0;
                let last = names[group.end - 1].1;
                (group, evidence(pieces, first, last))
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(named(&short, &[(2, 2)]), [(0..1, 0..17)]);
    assert_eq!(
        named(&long, &[(2, 2), (40, 40)]),
        [(0..1, 0..17), (1..2, 68..81)]
    );
    assert_eq!(named(&long, &[(39, 40)]), [(0..1, 66..81)]);
}

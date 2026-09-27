//! Ticket 0165's edge rows: what one record's names or edges do between two sides.

use super::{Row, Side, diff};
use crate::core::measure::answer::{self, Identity, Rule, Shown};
use crate::core::measure::items::Matching;
use crate::core::measure::json_lines;
use crate::core::pointer::Pointer;
use crate::core::render::json_line;

/// `MATCH CUT_A CUT_B | A | B | EXPECTED`. `-` reads a side as run, at its run cut of 0.5. A name
/// is `KIND START END STRENGTH`, and an edge `RELATION SOURCE/KIND TARGET/KIND P`, where `wrote`
/// has a direction and `met` has none. EXPECTED lists gained, lost, and changed-kind items, or `refused`.
const ROWS: [&str; 18] = [
    "strict - - | person 0 3 0.9 | person 0 3 0.9 | ",
    "strict - - | person 0 3 0.9 | person 0 3 0.9, work 5 9 0.7 | +work 5 9 0.7",
    "strict - - | person 0 3 0.9 | work 0 3 0.9 | ~person 0 3 0.9 > work 0 3 0.9",
    "overlap - - | place 0 10 0.7 | place 0 18 0.9 | ",
    "overlap - - | work 0 10 0.7 | place 0 18 0.9 | ~work 0 10 0.7 > place 0 18 0.9",
    "strict - - | work 0 10 0.7 | place 0 18 0.9 | +place 0 18 0.9; -work 0 10 0.7",
    "overlap - - | place 0 18 0.9 | place 0 5 0.6, place 6 18 0.8 | +place 0 5 0.6",
    "overlap - - | place 0 18 0.9 | place 0 5 0.7, place 6 18 0.7 | +place 6 18 0.7",
    "strict 0.6 0.6 | person 0 3 0.6 | person 0 3 0.59 | -person 0 3 0.6",
    "strict 0.6 0.6 | person 0 3 0.59 | person 0 3 0.6 | +person 0 3 0.6",
    "strict - - | work 0 3 0.9, person 0 3 0.6 | place 0 3 0.7, event 0 3 0.8 | ~work 0 3 0.9 > event 0 3 0.8; ~person 0 3 0.6 > place 0 3 0.7",
    "strict 0.5 0.8 | person 0 3 0.9, work 5 9 0.6 | person 0 3 0.9, work 5 9 0.6 | -work 5 9 0.6",
    "strict - 0.3 | person 0 3 0.9 | person 0 3 0.9 | refused",
    "strict 0.4:0.6 - | person 0 3 0.9 | person 0 3 0.9 | refused",
    "strict - - |  |  | ",
    "strict - - | wrote Ann/person Hey/song 0.9 | wrote Ann/band Hey/song 0.9 | +wrote Ann/band Hey/song 0.9; -wrote Ann/person Hey/song 0.9",
    "strict - - | met Ann/person Bob/person 0.9 | met Bob/person Ann/person 0.8 | ",
    "strict - - | wrote Ann/person Bob/person 0.9 | wrote Bob/person Ann/person 0.8 | +wrote Bob/person Ann/person 0.8; -wrote Ann/person Bob/person 0.9",
];

/// One name or edge as the command prints it, from its row text.
fn item(text: &str) -> String {
    let end = |held: &str| {
        let (name, kind) = held.split_once('/').unwrap_or_default();
        format!(r#"{{"name":"{name}","kind":"{kind}"}}"#)
    };
    match text.split_whitespace().collect::<Vec<_>>()[..] {
        [relation, source, target, p] if text.contains('/') => format!(
            r#"{{"relation":"{relation}","source":{},"target":{},"probability":{p}}}"#,
            end(source),
            end(target)
        ),
        [kind, start, end, strength] => {
            format!(r#"{{"kind":"{kind}","start":{start},"end":{end},"strength":{strength}}}"#)
        }
        _ => String::new(),
    }
}

/// One saved `--details` line holding a side's items.
fn line(side: &str, edges: bool) -> String {
    let items: Vec<String> = side
        .split(',')
        .map(item)
        .filter(|held| !held.is_empty())
        .collect();
    let (value, question) = match edges {
        true => (
            format!("[{}]", items.join(",")),
            r#""relate","relations":[{"name":"wrote"},{"name":"met","either":true}]"#,
        ),
        false => (
            format!(r#"{{"entities":[{}]}}"#, items.join(",")),
            r#""recognize","kinds":{"person":0,"work":0,"place":0,"event":0}"#,
        ),
    };
    format!(
        r#"{{"input":{{"id":"r"}},"value":{value},"question":{{"verb":{question},"threshold":0.5}}}}"#
    )
}

#[test]
fn each_item_edge_row_gains_loses_or_changes_kind_as_the_ticket_says() {
    let pointer = Pointer::new("/id").expect("a pointer");
    for row in ROWS {
        let [setting, a, b, expected] = row.split(" | ").collect::<Vec<_>>()[..] else {
            panic!("{row}")
        };
        let read = |side| {
            let lines = json_lines(line(side, row.contains('/')).as_bytes()).expect("a line");
            answer::read(&lines, &pointer, Identity::Answer).expect("an answer")
        };
        let (a, b) = (read(a), read(b));
        let rules: Vec<Rule> = setting
            .split(' ')
            .map(|cut| cut.parse().map_or(Rule::AsRun, Rule::Threshold))
            .collect();
        let [_, rule_a, rule_b] = rules[..] else {
            panic!("{row}")
        };
        let [x, y] = [(&a, rule_a), (&b, rule_b)].map(|(answers, rule)| Side { answers, rule });
        let matching =
            [Matching::Strict, Matching::Overlap][usize::from(row.starts_with("overlap"))];
        let shown = [Shown::AsRun, Shown::AsRun];
        let got =
            diff(x, y, None, shown, "runs", Some(matching)).map(|(rows, _)| match rows.first() {
                Some(Row::Items(change)) => {
                    json_line(&(&change.gained, &change.lost, &change.changed_kind))
                }
                _ => Ok("[[],[],[]]".to_owned()),
            });
        let group = |sign| {
            let items = expected
                .split("; ")
                .filter_map(|held| held.strip_prefix(sign));
            let moved = |(from, to)| format!(r#"{{"from":{},"to":{}}}"#, item(from), item(to));
            let items: Vec<String> = items
                .map(|held| held.split_once(" > ").map_or_else(|| item(held), moved))
                .collect();
            items.join(",")
        };
        let want = match expected {
            "refused" => expected.to_owned(),
            _ => format!("[[{}],[{}],[{}]]", group('+'), group('-'), group('~')),
        };
        let got = got.map_or_else(|_| "refused".to_owned(), |got| got.expect("JSON"));
        assert_eq!(got, want, "{row}");
    }
}

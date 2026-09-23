//! The stand-in answers relate by method H where a method-H recording
//! exists (main's ruling of 2026-09-23 in `sdlc/planning/relate-design.md`,
//! and `sdlc/records/surfaces-notes/NOTES-main-parity.md`).
//!
//! Method H asks one yes/no per pair per relation. A one-way relation asks
//! each direction as its own question, so an edge keeps the direction the
//! recording answered. A both-ways relation asks each pair once. The rows
//! come from the relate-methods bake-off by
//! `conformance/tools/build_relate_yes_no.py`.

use thinkthen_contract::{Engine as _, Kind, Relate, Settings};
use thinkthen_standin::BlockingEngine;

fn edges(ask: &Relate, records: &[&str]) -> Vec<(String, u64, u64, f64)> {
    BlockingEngine::from_settings(Settings::default())
        .relate(ask, records)
        .expect("the records are recorded")
        .into_iter()
        .map(|edge| (edge.name, edge.source, edge.target, edge.probability))
        .collect()
}

/// A one-way chain returns each link once, the right way round.
#[test]
fn a_one_way_relation_keeps_the_recorded_direction() {
    let ask = Relate::new()
        .relation("causes", Kind::Any, Kind::Any)
        .expect("a legal rule");
    let records = [
        "Heavy rain falls in the delivery area.",
        "A road on the delivery route is closed.",
        "The package is delivered late.",
        "The customer files a complaint.",
        "The customer is given a refund.",
        "The warehouse installs new shelves.",
        "The office coffee machine is replaced.",
    ];
    let causes = |source, target, probability| ("causes".to_owned(), source, target, probability);
    assert_eq!(
        edges(&ask, &records),
        vec![
            causes(1, 2, 0.76),
            causes(2, 3, 0.91),
            causes(3, 4, 0.84),
            causes(4, 5, 0.76)
        ]
    );
}

/// A both-ways relation returns each true pair once, with the lower record
/// first.
#[test]
fn a_both_ways_relation_returns_each_pair_once() {
    let ask = Relate::new()
        .either("contradicts", Kind::Any)
        .expect("a legal rule");
    let records = [
        "Book economy class for every flight under six hours.",
        "Submit receipts within 30 days of the trip.",
        "Hotel stays are capped at 200 dollars a night.",
        "Employees may book business class on any flight.",
        "Rental cars need a manager's approval.",
        "Receipts may be submitted at any time, with no deadline.",
        "Meals are reimbursed up to 60 dollars a day.",
        "Use the company travel portal for all bookings.",
        "Hotel stays may cost up to 350 dollars a night in any city.",
        "Rental cars can be booked without anyone's approval.",
        "Flights over six hours may be booked in business class.",
    ];
    let contradicts =
        |source, target, probability| ("contradicts".to_owned(), source, target, probability);
    assert_eq!(
        edges(&ask, &records),
        vec![
            contradicts(1, 4, 0.61),
            contradicts(2, 6, 0.91),
            contradicts(3, 9, 0.83),
            contradicts(5, 10, 0.91)
        ]
    );
}

/// The alerts sample answers by method H. A one-way rule keeps each
/// recorded direction, and a both-ways rule on the same records adds its
/// own edge.
#[test]
fn the_alerts_answer_by_method_h() {
    let ask = Relate::new()
        .relation("caused_by", Kind::Any, Kind::Any)
        .expect("a legal rule")
        .either("same_as", Kind::Any)
        .expect("a legal rule");
    let records = [
        "Checkout returns 500 at the payment step.",
        "Card charges are failing for every customer.",
        "The nightly export ran two hours late.",
        "The payments database ran out of disk space.",
    ];
    let edge = |name: &str, source, target, probability| (name.to_owned(), source, target, probability);
    assert_eq!(
        edges(&ask, &records),
        vec![
            edge("same_as", 1, 2, 0.61),
            edge("caused_by", 1, 4, 0.71),
            edge("caused_by", 2, 1, 0.65),
            edge("caused_by", 2, 4, 0.73),
            edge("caused_by", 3, 4, 0.55)
        ]
    );
}

/// One pair holds two relations under method H. Pick-one kept at most one.
#[test]
fn a_founder_pair_holds_both_relations() {
    let ask = Relate::new()
        .relation("founded", Kind::Any, Kind::Any)
        .expect("a legal rule")
        .relation("works_for", Kind::Any, Kind::Any)
        .expect("a legal rule");
    let table: serde_json::Value =
        serde_json::from_str(include_str!("../../conformance/relate-h/sets/founders-24.json")).expect("the set");
    let records: Vec<&str> =
        table["items"].as_array().expect("items").iter().map(|item| item["text"].as_str().expect("text")).collect();
    let got = edges(&ask, &records);
    let first: Vec<_> = got.iter().filter(|edge| edge.1 == 1 && edge.2 == 2).collect();
    assert_eq!(
        first,
        vec![&("founded".to_owned(), 1, 2, 0.91), &("works_for".to_owned(), 1, 2, 0.82)]
    );
    assert_eq!(got.len(), 36, "every recorded yes at the 0.5 bar");
}

/// Ten staff records answer by method H: each person works for the one
/// organization the record names.
#[test]
fn the_staff_records_answer_by_method_h() {
    let ask = Relate::new()
        .relation("works_for", Kind::Any, Kind::Any)
        .expect("a legal rule");
    let table: serde_json::Value =
        serde_json::from_str(include_str!("../../conformance/relate-h/sets/staff-10.json")).expect("the set");
    let records: Vec<&str> =
        table["items"].as_array().expect("items").iter().map(|item| item["text"].as_str().expect("text")).collect();
    let works_for = |source, target, probability| ("works_for".to_owned(), source, target, probability);
    assert_eq!(
        edges(&ask, &records),
        vec![
            works_for(1, 6, 0.99),
            works_for(2, 7, 0.98),
            works_for(3, 8, 0.96),
            works_for(4, 9, 0.95),
            works_for(5, 10, 0.96)
        ]
    );
}

/// A rule asked the other way round from the recording has no recorded
/// answer. The alerts recorded `caused_by` one way and `same_as` both ways.
#[test]
fn a_rule_asked_in_the_other_direction_is_refused() {
    let records = [
        "Checkout returns 500 at the payment step.",
        "Card charges are failing for every customer.",
        "The nightly export ran two hours late.",
        "The payments database ran out of disk space.",
    ];
    let both_ways = Relate::new().either("caused_by", Kind::Any).expect("a legal rule");
    let one_way = Relate::new().relation("same_as", Kind::Any, Kind::Any).expect("a legal rule");
    for (ask, sentence) in [
        (both_ways, "the recording asks caused_by one way"),
        (one_way, "the recording asks same_as both ways"),
    ] {
        let refused = BlockingEngine::from_settings(Settings::default())
            .relate(&ask, &records)
            .expect_err("the recording asked the other direction");
        assert_eq!(refused.kind, thinkthen_contract::ErrorKind::Usage);
        assert!(refused.message.contains(sentence), "{refused}");
    }
}

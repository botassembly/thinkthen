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

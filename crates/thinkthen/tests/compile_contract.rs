//! Compile-pass and compile-fail fixtures for the frozen public contract.
//!
//! One table holds every fixture. Each row becomes one binary of an outside
//! crate that depends on `thinkthen` with default features off, and one
//! `cargo check` compiles them all. A pass row must compile clean. A fail row
//! must fail with its error code and a phrase from the diagnostic, never a
//! line number, so compiler decoration can move without breaking the table.
#![allow(clippy::expect_used, reason = "a failed fixture stops the proof")]

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[path = "../src/test_deadline/child.rs"]
mod child;

/// `None` compiles; `Some((code, phrase))` fails with that code and phrase.
type Expected = Option<(&'static str, &'static str)>;

const PRELUDE: &str = r#"#![allow(dead_code, unused)]
use thinkthen::*;
thinkthen::choices! {
    /// Where a note belongs.
    pub enum Desk { Billing => "billing", Support => "support" }
}
fn engine() -> Engine { Engine::builder().no_cache().build().unwrap() }
fn yes_no() -> Question { Question::decide("asks for a refund").unwrap().cut() }
fn banded() -> BandedQuestion { Question::decide("asks").unwrap().band(0.3, 0.7).unwrap() }
fn desk() -> ChooseQuestion<Desk> {
    Question::choose::<Desk>("which desk").unwrap()
        .option(Desk::Billing, None).unwrap().option(Desk::Support, None).unwrap()
        .build().unwrap()
}
fn desks() -> TagQuestion<Desk> {
    Question::tag::<Desk>("which desks").unwrap()
        .label(Desk::Billing, None).unwrap().label(Desk::Support, None).unwrap()
        .cut().unwrap()
}
fn main() {}
"#;

const TABLE: &[(&str, Expected, &str)] = &[
    (
        "described_choice_macro",
        None,
        r#"
mod downstream {
    thinkthen::choices! {
        pub enum Route {
            Billing => "billing": "Charges and refunds",
            Outage => "outage": { thinkthen::Description::builder().what("A service outage")?.example("Service down")?.build() },
            Other => "other",
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
enum Manual { One }
impl Choice for Manual {
    fn label(&self) -> &'static str { "one" }
    fn labels() -> &'static [&'static str] { &["one"] }
    fn from_label(value: &str) -> Option<Self> { (value == "one").then_some(Self::One) }
}
fn described() -> Result<(), Error> {
    use downstream::Route;
    let _: Option<Description> = Route::Billing.description()?;
    let _: Option<Description> = Manual::One.description()?;
    let _: ChooseQuestion<Route> = Question::choose::<Route>("Where?")?
        .option(Route::Billing, None)?.option(Route::Outage, None)?.option(Route::Other, None)?.build()?;
    let _: TagQuestion<Route> = Question::tag::<Route>("Which?")?
        .label(Route::Billing, None)?.label(Route::Outage, None)?.label(Route::Other, None)?.cut()?;
    let _: Option<Route> = Route::from_label("billing");
    let _: Vec<Route> = vec![Route::Other];
    Ok(())
}
"#,
    ),
    (
        "every_method_and_convenience",
        None,
        r#"
fn methods(e: &Engine, o: CallOptions<'_>) -> Result<(), Error> {
    let (q, b, texts) = (yes_no(), banded(), vec!["a".to_owned(), "b".to_owned()]);
    let _: Call<Answer> = e.decide(&q, "text")?;
    let _: Call<Answer> = e.decide_with(&b, "text", o)?;
    let _: Call<Option<Desk>> = e.choose(&desk(), "text")?;
    let _: Call<Option<Desk>> = e.choose_with(&desk(), "text", o)?;
    let _: Call<f64> = e.score(&Question::score("how urgent").unwrap().level("low", None)?.level("high", None)?.build()?, "t")?;
    let _: Call<Vec<Desk>> = e.tag(&desks(), "text")?;
    let _: Call<Vec<Desk>> = e.tag_with(&desks(), "text", o)?;
    let _: Vec<Result<String, Error>> = e.filter(&q, texts.clone()).collect();
    let _: Vec<Result<&str, Error>> = e.filter_with(&q, ["a", "b"], o).collect();
    let _: Call<Vec<Ranked<&str>>> = e.rank(&Question::rank("most urgent")?, ["a", "b"])?;
    let _: Call<Found<String>> = e.find_with(&Question::find("the refund")?, texts.clone(), o)?;
    let set = QuestionSet::builder().question("refund", q.clone())?.banded("b", b.clone())?
        .choose("desk", desk())?.tag("desks", desks())?.build()?;
    let _: Vec<Result<AnnotatedRecord<&str>, Error>> = e.annotate(&set, ["a"]).collect();
    let rec = Recognize::builder().kind(Kind::new("drug", Some(Description::text("a medicine")?))?)?.build()?;
    let _: Call<Recognized> = e.recognize_with(&rec, "text", o)?;
    let rel = Relate::builder().relation(RelationRule::one_way("treats", "drug", "disease")?)?.build()?;
    let _: Call<Vec<Edge>> = e.relate(&rel, [Entity::new("a", "drug")?, Entity::new("b", "disease")?])?;
    let _: Vec<Result<Row<&str, Answer>, Error>> = e.decide_many(&b, ["a"]).collect();
    let _: Call<Details> = e.details(&desk(), "text")?;
    let _: Counters = e.usage();
    Ok(())
}
fn conveniences(o: CallOptions<'_>) -> Result<(), Error> {
    let (q, token) = (yes_no(), CancelToken::new());
    let o = o.cancel(&token).deadline_after(std::time::Duration::from_secs(1))?;
    let _: &'static Engine = default_engine()?;
    let _: Call<Answer> = decide(&q, "t")?;
    let _: Call<Option<Desk>> = choose(&desk(), "t")?;
    let _: Call<Vec<Desk>> = tag_with(&desks(), "t", o)?;
    let _: Call<f64> = score_with(&q, "t", o)?;
    let _ = filter(&q, ["a"]).count();
    let _ = decide_many_with(&q, ["a"], o).count();
    let _ = rank_with(&q, ["a"], o)?;
    let _ = find(&q, ["a"])?;
    let _ = annotate_with(&QuestionSet::from_json("{}")?, ["a"], o).count();
    let _ = recognize(&Recognize::builder().build()?, "t")?;
    let _ = relate_with(&Relate::builder().build()?, Vec::new(), o)?;
    let _: Call<Details> = details_with(&banded(), "t", o)?;
    let _: Counters = usage()?;
    Ok(())
}
fn descriptions() -> Result<Description, Error> {
    let loaded: LoadedQuestion = Question::from_json("{}")?;
    let _: ChooseQuestion<Desk> = yes_no().into_choose::<Desk>()?;
    Description::builder().what("a refund")?.not_for("a return")?.example("give it back")?
        .field_json("tone", "\"urgent\"")?.build()
}
"#,
    ),
    (
        "a_band_is_not_a_filter",
        Some(("E0308", "expected `&Question`, found `&BandedQuestion`")),
        r#"
fn f(e: &Engine) { let _ = e.filter(&banded(), ["a"]); }
"#,
    ),
    (
        "a_choose_question_does_not_decide",
        Some(("E0277", "DecisionQuestion")),
        r#"
fn f(e: &Engine) { let _ = e.decide(&desk(), "text"); }
"#,
    ),
    (
        "a_tag_question_does_not_choose",
        Some((
            "E0308",
            "expected `&ChooseQuestion<_>`, found `&TagQuestion<Desk>`",
        )),
        r#"
fn f(e: &Engine) { let _ = e.choose(&desks(), "text"); }
"#,
    ),
    (
        "an_unfinished_builder_is_not_a_question",
        Some(("E0277", "DecisionQuestion")),
        r#"
fn f(e: &Engine) { let _ = e.decide(&Question::decide("asks").unwrap(), "text"); }
"#,
    ),
    (
        "the_engine_module_is_private",
        Some(("E0603", "module `engine` is private")),
        r#"
use thinkthen::engine::facade::Engine as Inner;
"#,
    ),
    (
        "the_engine_fields_are_private",
        Some(("E0616", "field `inner` of struct")),
        r#"
fn f(e: &Engine) { let _ = &e.inner; }
"#,
    ),
    (
        "a_duplicate_label_fails",
        Some(("E0080", "choices! labels must differ")),
        r#"
thinkthen::choices! { enum Twice { One => "same", Two => "same" } }
"#,
    ),
];

#[test]
fn every_contract_fixture_compiles_or_fails_as_its_row_says() {
    let crate_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("compile-contract");
    let bins = crate_dir.join("src/bin");
    let _ = fs::remove_dir_all(&bins);
    fs::create_dir_all(&bins).unwrap();
    let library = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = format!(
        "[package]\nname = \"contract-fixtures\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         publish = false\n\n[workspace]\n\n[dependencies]\n\
         thinkthen = {{ path = {:?}, default-features = false }}\n",
        library.display().to_string()
    );
    fs::write(crate_dir.join("Cargo.toml"), manifest).unwrap();
    fs::copy(
        library.join("../../Cargo.lock"),
        crate_dir.join("Cargo.lock"),
    )
    .unwrap();
    for (name, _, body) in TABLE {
        fs::write(bins.join(format!("{name}.rs")), format!("{PRELUDE}{body}")).unwrap();
    }
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = child::command(&cargo, child::CARGO)
        .args([
            "check",
            "--offline",
            "--keep-going",
            "--bins",
            "--message-format=json",
        ])
        .current_dir(&crate_dir)
        .env("CARGO_TARGET_DIR", crate_dir.join("target"))
        .output()
        .expect("cargo runs");
    let mut errors: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-message" || message["message"]["level"] != "error" {
            continue;
        }
        let code = message["message"]["code"]["code"]
            .as_str()
            .unwrap_or_default();
        let rendered = message["message"]["rendered"].as_str().unwrap_or_default();
        let target = message["target"]["name"]
            .as_str()
            .unwrap_or_default()
            .replace('-', "_");
        errors
            .entry(target)
            .or_default()
            .push(format!("{code} {rendered}"));
    }
    let mut wrong = Vec::new();
    for (name, expected, _) in TABLE {
        let found = errors.remove(*name).unwrap_or_default();
        let met = match expected {
            None => found.is_empty(),
            Some((code, phrase)) => found
                .iter()
                .any(|error| error.starts_with(code) && error.contains(phrase)),
        };
        if !met {
            wrong.push(format!("{name}: expected {expected:?}, found {found:#?}"));
        }
    }
    assert!(
        wrong.is_empty(),
        "{wrong:#?}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(errors.is_empty(), "errors outside the table: {errors:#?}");
}

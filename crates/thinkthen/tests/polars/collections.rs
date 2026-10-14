//! Public whole-column consumers with independent saved replies and send counts.
use super::common;
use conformance_backend::{Backend, Canned, Listener};
use thinkthen::polars::prelude::{DataFrame, IntoLazy, NamedFrom, Series};
use thinkthen::{CallOptions, ErrorKind, PolarsEngine, Question, Recognize, Relate};

#[test]
fn filter_and_rank_keep_nullable_duplicate_row_positions() {
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let texts = Series::new(
        "body".into(),
        [Some("strong"), None, Some("weak"), Some("strong")],
    );
    let decide = Question::decide("Relevant?").expect("question").cut();
    let filtered = engine
        .filter_series(&decide, &texts, common::singleton())
        .expect("filter");
    assert_eq!(filtered.value().name().as_str(), "body");
    assert_eq!(
        filtered
            .value()
            .str()
            .expect("text")
            .iter()
            .collect::<Vec<_>>(),
        [Some("strong"), Some("strong")]
    );
    assert_eq!(filtered.facts().requests_sent(), 3);
    let rank = Question::rank("Relevant?").expect("rank question");
    let ranked = engine
        .rank_series(&rank, &texts, common::singleton())
        .expect("rank");
    assert_eq!(
        ranked
            .value()
            .column("index")
            .expect("index")
            .u64()
            .expect("places")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0), Some(3), Some(2)]
    );
    assert_eq!(
        ranked
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("probabilities")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.9), Some(0.9), Some(0.2)]
    );
    // Native rank shares identical pending judgments while retaining both occurrences.
    assert_eq!(ranked.facts().requests_sent(), 2);
    assert_eq!(listener.count(), 5);
    assert_eq!(
        ranked
            .value()
            .column("record")
            .expect("records")
            .str()
            .expect("text")
            .iter()
            .collect::<Vec<_>>(),
        [Some("strong"), Some("strong"), Some("weak")]
    );
    let states = listener
        .requests()
        .iter()
        .map(|request| {
            serde_json::from_slice::<serde_json::Value>(&request.body).expect("body")["state"]
                .clone()
        })
        .collect::<Vec<_>>();
    assert_eq!(states[0], states[2]);
    assert_eq!(states[0], states[3]);
    assert_eq!(states[1], states[4]);
}

#[test]
fn find_and_lazy_find_see_all_candidates_in_the_complete_collection() {
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let texts = Series::new(
        "body".into(),
        [Some("strong"), None, Some("weak"), Some("strong")],
    );
    let find = Question::find("Which?")
        .expect("find question")
        .offering_none()
        .expect("none");
    let found = engine
        .find_series(&find, &texts, common::singleton())
        .expect("find");
    assert_eq!(
        found
            .value()
            .column("index")
            .expect("index")
            .u64()
            .expect("places")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0), Some(2), Some(3), None]
    );
    assert_eq!(
        found
            .value()
            .column("probability")
            .expect("probability")
            .f64()
            .expect("probabilities")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.1), Some(0.8), Some(0.05), Some(0.05)]
    );
    assert_eq!(
        found
            .value()
            .column("selected")
            .expect("selected")
            .bool()
            .expect("booleans")
            .iter()
            .collect::<Vec<_>>(),
        [Some(false), Some(true), Some(false), Some(false)]
    );
    let lazy = DataFrame::new(4, vec![texts.into()]).expect("frame").lazy();
    let materialized = engine
        .find_lazy(&find, lazy, "body", common::singleton())
        .expect("whole logical set");
    assert_eq!(materialized.value(), found.value());
    assert_eq!(listener.count(), 2);
    let bodies = listener.requests();
    let find_body: serde_json::Value = serde_json::from_slice(&bodies[0].body).expect("body");
    let units: serde_json::Value =
        serde_json::from_str(find_body["state"].as_str().expect("whole-set state")).expect("units");
    assert_eq!(units.as_array().expect("all candidates").len(), 3);
    assert_eq!(units[0]["evidence"], "strong");
    assert_eq!(units[1]["evidence"], "weak");
    assert_eq!(units[2]["evidence"], "strong");
}

#[test]
fn recognition_keeps_complete_spans_and_relations_and_relate_keeps_the_entity_set() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).expect("corpus");
    let case = corpus["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .find(|case| case["id"] == "42-recognize-C01-relations")
        .expect("saved case");
    let ask = Recognize::from_json(&case["question"].to_string()).expect("recognize specification");
    let replies = case["exchanges"].as_array().expect("saved exchanges");
    let responses = (0..2)
        .flat_map(|_| {
            replies
                .iter()
                .map(|reply| Canned::ok(&reply["response"].to_string()))
        })
        .collect();
    let saved = Listener::serving(responses).expect("saved reply listener");
    let engine = common::builder(saved.base())
        .no_cache()
        .model("jev-1.13.0")
        .expect("model")
        .build()
        .expect("engine");
    let texts = Series::new(
        "body".into(),
        [case["text"].as_str(), None, case["text"].as_str()],
    );
    let found = engine
        .recognize_series(&ask, &texts, CallOptions::new())
        .expect("recognition");
    assert!(found.value()[1].is_none());
    let first = found.value()[0].as_ref().expect("first");
    assert_eq!(
        first
            .entities()
            .iter()
            .map(|entity| (entity.text(), entity.start(), entity.end(), entity.kind()))
            .collect::<Vec<_>>(),
        [
            ("Maria Chen", 0, 10, "person"),
            ("Northwind Freight", 18, 35, "organization"),
            ("Chicago", 39, 46, "place")
        ]
    );
    assert_eq!(
        first
            .relations()
            .expect("relations")
            .iter()
            .map(|relation| relation.relation())
            .collect::<Vec<_>>(),
        ["works_for"]
    );
    assert_eq!(found.value()[0], found.value()[2]);
    assert_eq!(found.facts().records(), 2);
    assert_eq!(found.facts().requests_sent(), 6);
    assert_eq!(saved.requests().len(), 6);
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#)).expect("relations listener");
    let engine = common::engine(listener.base());
    let ask = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#).expect("relation spec");
    let frame = DataFrame::new(
        3,
        vec![
            Series::new("name".into(), [Some("Ada"), None, Some("Bo")]).into(),
            Series::new("kind".into(), [Some("person"), None, Some("person")]).into(),
        ],
    )
    .expect("entities frame");
    let related = engine
        .relate_frame(&ask, &frame, "name", "kind", CallOptions::new())
        .expect("relate");
    assert_eq!(
        related
            .value()
            .iter()
            .map(|edge| (edge.source().name(), edge.target().name()))
            .collect::<Vec<_>>(),
        [("Ada", "Bo"), ("Bo", "Ada")]
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn empty_invalid_and_cancelled_collections_do_not_send() {
    let backend = Backend::start().expect("backend");
    let engine = common::engine(&format!("{}/generic/v1", backend.origin()));
    let texts = Series::new("body".into(), Vec::<String>::new());
    let rank = Question::rank("Relevant?").expect("rank");
    let ranked = engine
        .rank_series(&rank, &texts, CallOptions::new())
        .expect("empty rank");
    assert_eq!(ranked.value().height(), 0);
    assert_eq!(ranked.facts().requests_sent(), 0);
    let find = Question::find("Which?").expect("find");
    assert_eq!(
        engine
            .find_series(&find, &texts, CallOptions::new())
            .expect_err("empty find")
            .kind(),
        ErrorKind::Usage
    );
    let token = thinkthen::CancelToken::new();
    token.cancel();
    assert_eq!(
        engine
            .rank_series(
                &rank,
                &common::column(&["one", "two"]),
                CallOptions::new().cancel(&token)
            )
            .expect_err("cancel")
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(backend.count(), 0);
    let wrong = Question::choose_labels("Which?")
        .expect("choose")
        .label("a", None)
        .expect("label")
        .label("b", None)
        .expect("label")
        .build()
        .expect("question");
    let text = common::column(&["one", "two"]);
    let numbers = Series::new("numbers".into(), [1_i64, 2]);
    let filter = Question::decide("Relevant?").expect("filter").cut();
    let decide = Question::decide("Relevant?").expect("decide").cut();
    let score = Question::score("How relevant?")
        .and_then(|builder| builder.level("low", None))
        .and_then(|builder| builder.level("high", None))
        .and_then(thinkthen::ScoreBuilder::build)
        .expect("score");
    for error in [
        engine
            .filter_series(&wrong, &text, CallOptions::new())
            .expect_err("wrong filter question"),
        engine
            .rank_series(&decide, &text, CallOptions::new())
            .expect_err("decide rank question"),
        engine
            .rank_series(&score, &text, CallOptions::new())
            .expect_err("score rank question"),
        engine
            .rank_series(&rank, &numbers, CallOptions::new())
            .expect_err("non-text rank"),
        engine
            .find_series(&find, &numbers, CallOptions::new())
            .expect_err("non-text find"),
    ] {
        assert_eq!(error.kind(), ErrorKind::Usage);
    }
    for error in [
        engine
            .filter_series(&filter, &text, CallOptions::new().cancel(&token))
            .expect_err("cancel filter"),
        engine
            .find_series(&find, &text, CallOptions::new().cancel(&token))
            .expect_err("cancel find"),
    ] {
        assert_eq!(error.kind(), ErrorKind::Cancelled);
    }
    remaining_zero_send_cases(&engine, &text, &numbers, &decide, &token);
    assert_eq!(backend.count(), 0);
}

#[expect(
    clippy::expect_used,
    reason = "a failed loopback fixture stops the consumer"
)]
fn remaining_zero_send_cases(
    engine: &thinkthen::Engine,
    text: &Series,
    numbers: &Series,
    decide: &Question,
    token: &thinkthen::CancelToken,
) {
    let recognize =
        Recognize::from_json(r#"{"version":1,"recognize":{"kinds":{"person":"A person name."}}}"#)
            .expect("recognition");
    let relate = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#).expect("relation");
    let entities = |kinds| {
        DataFrame::new(
            2,
            vec![
                Series::new("name".into(), [Some("Ada"), Some("Bo")]).into(),
                Series::new("kind".into(), kinds).into(),
            ],
        )
        .expect("frame")
    };
    let valid = entities([Some("person"), Some("person")]);
    let invalid = entities([Some("person"), None]);
    for error in [
        engine
            .recognize_series(&recognize, numbers, CallOptions::new())
            .expect_err("non-text recognition"),
        engine
            .relate_frame(&relate, &invalid, "name", "kind", CallOptions::new())
            .expect_err("late unmatched null"),
        engine
            .column_with(decide, numbers, thinkthen::PolarsCallOptions::new())
            .expect_err("non-text probability"),
    ] {
        assert_eq!(error.kind(), ErrorKind::Usage);
    }
    for error in [
        engine
            .recognize_series(&recognize, text, CallOptions::new().cancel(token))
            .expect_err("cancel recognition"),
        engine
            .relate_frame(
                &relate,
                &valid,
                "name",
                "kind",
                CallOptions::new().cancel(token),
            )
            .expect_err("cancel relate"),
        engine
            .column_with(
                decide,
                text,
                thinkthen::PolarsCallOptions::new().call(CallOptions::new().cancel(token)),
            )
            .expect_err("cancel probability"),
    ] {
        assert_eq!(error.kind(), ErrorKind::Cancelled);
    }
}

#[expect(
    clippy::expect_used,
    reason = "a failed loopback fixture stops the consumer"
)]
fn collection_listener() -> Listener {
    Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains("u001") {
            Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","probabilities":{"u001":0.1,"u002":0.8,"u003":0.05,"none":0.05}}},"usage":{"input_tokens":10,"output_tokens":2}}"#)
        } else {
            let probability = if String::from_utf8_lossy(body).contains("weak") { 0.2 } else { 0.9 };
            Canned::ok(&format!(r#"{{"model":"jev-latest","answers":{{"q1":{{"type":"noul","noul":{probability}}}}},"usage":{{"input_tokens":10,"output_tokens":2}}}}"#))
        }
    }).expect("listener")
}

#[test]
fn collection_expressions_preserve_logical_sets_and_nullable_original_positions() {
    use thinkthen::PolarsExprOptions;
    use thinkthen::polars::prelude::col;
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let column = Series::new(
        "body".into(),
        [Some("strong"), None, Some("weak"), Some("strong")],
    );
    let frame = DataFrame::new(4, vec![column.into()]).expect("frame");
    let options = PolarsExprOptions::new().batch(thinkthen::BatchSetting::Records(
        std::num::NonZeroUsize::MIN,
    ));
    let rank = Question::rank("Relevant?").expect("rank");
    let ranked = frame
        .clone()
        .lazy()
        .select([engine
            .rank_expr(&rank, col("body"), options.clone())
            .expect("rank expression")])
        .collect()
        .expect("rank collection");
    let ranked = ranked
        .column("body")
        .expect("ranked")
        .struct_()
        .expect("Struct");
    assert_eq!(
        ranked
            .field_by_name("value")
            .expect("indices")
            .u64()
            .expect("UInt64")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0), Some(3), Some(2)]
    );
    assert_eq!(
        ranked
            .field_by_name("probability")
            .expect("probability")
            .f64()
            .expect("Float64")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.9), Some(0.9), Some(0.2)]
    );
    let decide = Question::decide("Relevant?").expect("decide").cut();
    let filtered = frame
        .clone()
        .lazy()
        .select([engine
            .filter_expr(&decide, col("body"), options.clone())
            .expect("filter expression")])
        .collect()
        .expect("filter collection");
    let filtered = filtered
        .column("body")
        .expect("filtered")
        .struct_()
        .expect("Struct");
    assert_eq!(
        filtered
            .field_by_name("value")
            .expect("values")
            .str()
            .expect("String")
            .iter()
            .collect::<Vec<_>>(),
        [Some("strong"), Some("strong")]
    );
}

#[test]
fn find_expression_compares_one_complete_nullable_collection() {
    use thinkthen::PolarsExprOptions;
    use thinkthen::polars::prelude::col;
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let column = Series::new(
        "body".into(),
        [Some("strong"), None, Some("weak"), Some("strong")],
    );
    let frame = DataFrame::new(4, vec![column.into()]).expect("frame");
    let options = PolarsExprOptions::new().batch(thinkthen::BatchSetting::Records(
        std::num::NonZeroUsize::MIN,
    ));
    let before = listener.count();
    let find = Question::find("Which?")
        .expect("find")
        .offering_none()
        .expect("none");
    let found = frame
        .lazy()
        .select([engine
            .find_expr(&find, col("body"), options)
            .expect("find expression")])
        .collect()
        .expect("find collection");
    let found = found
        .column("body")
        .expect("found")
        .struct_()
        .expect("Struct");
    assert_eq!(
        found
            .field_by_name("value")
            .expect("picked")
            .str()
            .expect("String")
            .get(0),
        Some("weak")
    );
    assert_eq!(
        found
            .field_by_name("probability")
            .expect("picked probability")
            .f64()
            .expect("Float64")
            .get(0),
        Some(0.8)
    );
    assert_eq!(
        listener.count() - before,
        1,
        "find compares one complete candidate set"
    );
    let requests = listener.requests();
    let body: serde_json::Value =
        serde_json::from_slice(&requests.last().expect("find request").body).expect("body");
    let candidates: serde_json::Value =
        serde_json::from_str(body["state"].as_str().expect("state")).expect("candidates");
    assert_eq!(candidates.as_array().expect("all candidates").len(), 3);
}

#[test]
fn relation_expression_omits_paired_nulls_and_refuses_a_partial_entity_before_sending() {
    use thinkthen::PolarsExprOptions;
    use thinkthen::polars::prelude::col;
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#)).expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let question = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#).expect("relation");
    let expression = engine
        .relate_expr(
            &question,
            col("name"),
            col("kind"),
            PolarsExprOptions::new(),
        )
        .expect("expression");
    let frame = |kinds: [Option<&str>; 3]| {
        DataFrame::new(
            3,
            vec![
                Series::new("name".into(), [Some("Ada"), None, Some("Bo")]).into(),
                Series::new("kind".into(), kinds).into(),
            ],
        )
        .expect("entities")
    };
    let related = frame([Some("person"), None, Some("person")])
        .lazy()
        .select([expression.clone()])
        .collect()
        .expect("complete set");
    let result = related
        .column("name")
        .expect("result")
        .struct_()
        .expect("Struct");
    assert_eq!(
        result
            .field_by_name("value")
            .expect("edges")
            .list()
            .expect("List(Struct)")
            .get_as_series(0)
            .expect("edges")
            .len(),
        2
    );
    let probabilities = result
        .field_by_name("probability")
        .expect("probability")
        .list()
        .expect("List(Float64)")
        .get_as_series(0)
        .expect("probabilities");
    assert_eq!(
        probabilities
            .f64()
            .expect("Float64")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.9), Some(0.8)]
    );
    assert_eq!(listener.count(), 1);
    assert!(
        frame([Some("person"), Some("person"), Some("person")])
            .lazy()
            .select([expression])
            .collect()
            .expect_err("partial entity")
            .to_string()
            .contains("only one null")
    );
    assert_eq!(listener.count(), 1, "invalid collection sends nothing");
}

#[test]
fn records_column_keeps_rejected_filter_observations_and_refuses_incompatible_originals() {
    use thinkthen::{RecordInput, RequestDefinition, RequestFunction};
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.2}}}"#)).expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let question = Question::decide("Relevant?").expect("question").cut();
    let definition = RequestDefinition::from(question.clone());
    let column = Series::new("body".into(), [Some("strong"), None, Some("weak")]);
    let record = |text: &str| RecordInput {
        original: text.to_owned(),
        context: None,
        options: None,
        seed_spans: None,
        examples: None,
    };
    let result = engine
        .records_column(
            RequestFunction::Filter,
            &definition,
            &column,
            vec![Some(record("strong")), None, Some(record("weak"))],
            CallOptions::new(),
        )
        .expect("records");
    let records = result
        .value()
        .column("records")
        .expect("records")
        .struct_()
        .expect("Struct");
    assert_eq!(records.len(), 3);
    assert_eq!(records.null_count(), 1);
    assert_eq!(
        records
            .field_by_name("index")
            .expect("indices")
            .u64()
            .expect("UInt64")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0), None, Some(2)]
    );
    assert_eq!(
        records
            .field_by_name("value")
            .expect("values")
            .bool()
            .expect("Boolean")
            .iter()
            .collect::<Vec<_>>(),
        [Some(true), None, Some(false)]
    );
    let thinkthen::RequestValue::Filtered(native) = result.results() else {
        panic!("native filter records")
    };
    assert_eq!(
        native.len(),
        2,
        "rejected observations remain canonical results"
    );
    assert!(!native[1].result().value());
    assert_eq!(
        result
            .probability()
            .f64()
            .expect("Float64")
            .iter()
            .collect::<Vec<_>>(),
        [Some(0.9), None, Some(0.2)]
    );
    assert_eq!(
        (result.facts().records(), result.facts().requests_sent()),
        (2, 1)
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn records_column_refuses_changed_originals_or_null_positions_before_sending() {
    use thinkthen::{RecordInput, RequestDefinition, RequestFunction};
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let definition =
        RequestDefinition::from(Question::decide("Relevant?").expect("question").cut());
    let column = Series::new("body".into(), [Some("strong"), None, Some("weak")]);
    let record = |text: &str| RecordInput {
        original: text.to_owned(),
        context: None,
        options: None,
        seed_spans: None,
        examples: None,
    };
    for rows in [
        vec![Some(record("different")), None, Some(record("weak"))],
        vec![Some(record("strong")), Some(record("extra")), None],
    ] {
        let error = engine
            .records_column(
                RequestFunction::Filter,
                &definition,
                &column,
                rows,
                CallOptions::new(),
            )
            .expect_err("incompatible originals");
        assert_eq!(error.kind(), ErrorKind::Usage);
    }
    assert_eq!(
        listener.count(),
        0,
        "input compatibility fails before sending"
    );
}

#[test]
fn records_column_projects_scalar_unions_without_changing_native_records() {
    use thinkthen::{RecordInput, RequestDefinition, RequestFunction};
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let column = common::column(&["strong"]);
    for threshold in [serde_json::json!(0.5), serde_json::json!("0.2:0.8")] {
        let authored = serde_json::json!({"decide":"Relevant?","threshold":threshold});
        let definition =
            RequestDefinition::from_authored_json(&authored.to_string()).expect("definition");
        let result = engine
            .records_column(
                RequestFunction::Decide,
                &definition,
                &column,
                vec![Some(RecordInput {
                    original: "strong".to_owned(),
                    context: None,
                    options: None,
                    seed_spans: None,
                    examples: None,
                })],
                CallOptions::new(),
            )
            .expect("typed record");
        let records = result
            .value()
            .column("records")
            .expect("records")
            .struct_()
            .expect("Struct");
        let projected = records.field_by_name("threshold").expect("threshold");
        let branches = projected.struct_().expect("typed number/string union");
        let number = branches.field_by_name("number").expect("number");
        let text = branches.field_by_name("string").expect("string");
        assert_eq!(number.f64().expect("Float64").get(0), threshold.as_f64());
        assert_eq!(text.str().expect("String").get(0), threshold.as_str());
        let question = records.field_by_name("question").expect("question");
        assert_eq!(
            question
                .struct_()
                .expect("question struct")
                .field_by_name("text")
                .expect("text")
                .str()
                .expect("String")
                .get(0),
            Some("Relevant?")
        );
        let native = serde_json::to_value(result.results()).expect("native records");
        assert_eq!(native[0]["threshold"], threshold);
        assert_eq!(native[0]["question"]["text"], "Relevant?");
    }
    assert_eq!(
        listener.count(),
        2,
        "each record projection uses its one native call"
    );
}

#[test]
fn records_column_refuses_original_fields_missing_from_the_typed_column() {
    use thinkthen::polars::prelude::IntoSeries;
    use thinkthen::{RawRecord, RecordReading, RequestDefinition, RequestFunction};
    let listener = collection_listener();
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let original =
        RawRecord::json(r#"{"kept":"strong","extra":"must remain present"}"#).expect("original");
    let input = RecordReading::new(&["/kept"], None, None)
        .expect("reading")
        .compose(original)
        .expect("record");
    let column = DataFrame::new(1, vec![Series::new("kept".into(), ["strong"]).into()])
        .expect("frame")
        .into_struct("original".into())
        .into_series();
    let definition =
        RequestDefinition::from(Question::decide("Relevant?").expect("question").cut());
    let error = engine
        .records_column(
            RequestFunction::Decide,
            &definition,
            &column,
            vec![Some(input)],
            CallOptions::new(),
        )
        .expect_err("missing original field");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        listener.count(),
        0,
        "no original field is silently dropped before sending"
    );
}

#[test]
fn relation_records_keep_generated_member_question_text_and_whole_set_probabilities() {
    use thinkthen::polars::prelude::IntoSeries;
    use thinkthen::{RawRecord, RecordReading, RequestDefinition, RequestFunction};
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.8}}}"#)).expect("listener");
    let engine = common::builder(listener.base())
        .no_cache()
        .build()
        .expect("engine");
    let question = Relate::from_json(r#"{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person"}]}}"#).expect("relation");
    let reading = RecordReading::new(&[], None, None).expect("reading");
    let inputs = ["Ada", "Bo"]
        .into_iter()
        .map(|name| {
            let original =
                RawRecord::json(&serde_json::json!({"name":name,"kind":"person"}).to_string())
                    .expect("original");
            Some(reading.compose(original).expect("record"))
        })
        .collect();
    let column = DataFrame::new(
        2,
        vec![
            Series::new("name".into(), ["Ada", "Bo"]).into(),
            Series::new("kind".into(), ["person", "person"]).into(),
        ],
    )
    .expect("originals")
    .into_struct("original".into())
    .into_series();
    let call = engine
        .records_column(
            RequestFunction::Relate,
            &RequestDefinition::from(question),
            &column,
            inputs,
            CallOptions::new(),
        )
        .expect("records");
    let records = call
        .value()
        .column("records")
        .expect("records")
        .struct_()
        .expect("Struct");
    assert_eq!(records.len(), 1);
    let answer = records.field_by_name("answer").expect("answer");
    let relations = answer
        .struct_()
        .expect("answer Struct")
        .field_by_name("relations")
        .expect("relations");
    let members = relations
        .list()
        .expect("members List")
        .get_as_series(0)
        .expect("members");
    let questions = members
        .struct_()
        .expect("members Struct")
        .field_by_name("question")
        .expect("questions");
    let text = questions
        .struct_()
        .expect("question Struct")
        .field_by_name("text")
        .expect("text");
    assert_eq!(
        text.str().expect("String").into_iter().collect::<Vec<_>>(),
        [
            Some("Is it true that i1 knows i2?"),
            Some("Is it true that i2 knows i1?")
        ]
    );
    assert_eq!(
        call.probability()
            .list()
            .expect("probability List")
            .get_as_series(0)
            .expect("edge probabilities")
            .f64()
            .expect("Float64")
            .into_iter()
            .collect::<Vec<_>>(),
        [Some(0.9), Some(0.8)]
    );
    assert_eq!(call.facts().requests_sent(), 1);
    assert_eq!(listener.count(), 1);
}

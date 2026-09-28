//! The public annotate set's saved batch tier, observed at a real listener.

use super::*;

#[test]
fn saved_annotate_batch_tiers() {
    let _serial = serial();
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let plain = engine(&base);
    let maximal = Engine::builder()
        .base_url(&base)
        .and_then(|builder| builder.api_key("sk-public-batches"))
        .map(EngineBuilder::no_cache)
        .map(|builder| builder.batch(BatchSetting::Max))
        .and_then(EngineBuilder::build)
        .expect("max engine");
    let saved = QuestionSet::from_json(
        r#"{"version":1,"batch":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    )
    .expect("saved batch one");
    let invalid = QuestionSet::from_json(
        r#"{"version":1,"batch":0,"questions":{"ready":{"decide":"Ready?"}}}"#,
    )
    .expect("invalid saved tier is retained until selected");
    let records = ["one", "two", "three"];
    let run = |engine: &Engine, set: &QuestionSet, options| {
        let mut batch = engine.annotate_with(set, records, options);
        let rows = batch.by_ref().collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            rows.iter().map(|row| *row.input()).collect::<Vec<_>>(),
            records
        );
        assert_eq!(
            rows.iter()
                .map(AnnotatedRecord::value_json)
                .collect::<Vec<_>>(),
            [r#"{"ready":true}"#; 3]
        );
        Ok::<_, Error>((rows, batch.facts().cloned()))
    };

    let (rows, facts) = run(&plain, &saved, CallOptions::new()).expect("saved tier");
    assert_eq!(rows.len(), 3);
    assert_eq!(
        facts.map(|facts| (facts.records(), facts.requests_sent())),
        Some((3, 3))
    );
    assert_eq!(backend.count(), 3);

    let (rows, facts) = run(&plain, &saved, CallOptions::new().batch(BatchSetting::Max))
        .expect("typed max overrides set");
    assert_eq!(rows.len(), 3);
    assert_eq!(
        facts.map(|facts| (facts.records(), facts.requests_sent())),
        Some((3, 1))
    );
    assert_eq!(backend.count(), 4);

    let (rows, facts) =
        run(&maximal, &saved, CallOptions::new()).expect("engine max overrides set");
    assert_eq!(rows.len(), 3);
    assert_eq!(
        facts.map(|facts| (facts.records(), facts.requests_sent())),
        Some((3, 1))
    );
    assert_eq!(backend.count(), 5);

    let error = run(&plain, &invalid, CallOptions::new()).expect_err("selected invalid batch");
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert_eq!(
        error.to_string(),
        "`batch` takes max or a whole number of at least 1"
    );
    assert_eq!(backend.count(), 5);

    for (engine, options) in [
        (&plain, CallOptions::new().batch(BatchSetting::Max)),
        (&maximal, CallOptions::new()),
    ] {
        let (rows, facts) = run(engine, &invalid, options).expect("invalid unselected tier");
        assert_eq!(rows.len(), 3);
        assert_eq!(
            facts.map(|facts| (facts.records(), facts.requests_sent())),
            Some((3, 1))
        );
    }
    assert_eq!(backend.count(), 7);
}

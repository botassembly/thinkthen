//! Configured command pricing at the compiled listener boundary.

use super::*;

fn configuration(name: &str, body: &str) -> std::path::PathBuf {
    let root = crate::facts::fresh_root(name);
    let config = Folder::Config.under(&root);
    fs::create_dir_all(&config).expect("configuration folder");
    fs::write(config.join("config.json"), body).expect("configuration");
    root
}

fn run(root: &Path, listener: &Listener, input: &[u8], extra: &[&str]) -> std::process::Output {
    let args = [
        &[
            "decide",
            QUESTION,
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
        ][..],
        extra,
    ]
    .concat();
    let moved = Folder::Config.variable(root);
    spawn(&args, &[KEY, (moved.0, moved.1.as_str())], input).expect("priced command")
}

#[test]
fn caller_price_config_rounds_once_and_a_retry_missing_usage_omits_cost() {
    let prices = r#"{"schema":"thinkthen.config/1","usd_per_million_input":"0.25","usd_per_million_output":"0.25"}"#;
    let root = configuration("facts-priced-config", prices);
    let reply = r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":1}}"#;
    let good = Listener::serving(vec![Canned::ok(reply)]).expect("listener");
    let complete = run(&root, &good, b"Refund me.", &[]);
    assert_eq!(complete.status.code(), Some(0));
    assert_eq!(good.requests().len(), 1);
    assert_eq!(line(&complete)["estimated_cost_usd"], "0.000001");

    // The blocked usage folder uses a Unix mode.
    #[cfg(unix)]
    {
        // One root holds the prices and a usage folder the writer cannot make.
        let blocked = configuration("facts-priced-usage-unwritable", prices);
        crate::facts::block_usage(&blocked);
        let (config, state) = (
            Folder::Config.variable(&blocked),
            Folder::Usage.variable(&blocked),
        );
        let warned_listener = Listener::answering(move |_| Canned::ok(reply)).expect("listener");
        let warned = spawn(
            &[
                "decide",
                QUESTION,
                "--facts",
                "--no-cache",
                "--url",
                warned_listener.base(),
            ],
            &[
                KEY,
                (config.0, config.1.as_str()),
                (state.0, state.1.as_str()),
            ],
            b"Still a refund.",
        )
        .expect("warned command");
        assert_eq!(warned.status.code(), Some(0));
        assert_eq!(warned_listener.count(), 1);
        assert_eq!(line(&warned)["estimated_cost_usd"], "0.000001");
        assert!(
            String::from_utf8_lossy(&warned.stderr).contains("usage counters could not be updated")
        );
    }

    let retried =
        Listener::serving(vec![Canned::status(503, "busy"), Canned::ok(reply)]).expect("listener");
    let partial = run(&root, &retried, b"Another refund.", &["--max-retries", "1"]);
    assert_eq!(partial.status.code(), Some(0));
    assert_eq!(retried.requests().len(), 2);
    let facts = line(&partial);
    assert_eq!(facts["requests_sent"], 2);
    assert_eq!(
        (
            facts["input_tokens"].as_u64(),
            facts["output_tokens"].as_u64()
        ),
        (Some(1), Some(1))
    );
    assert!(facts.get("estimated_cost_usd").is_none());
}

#[test]
fn caller_price_config_rejects_invalid_pair_without_a_send() {
    let root = configuration(
        "facts-price-refusal",
        r#"{"schema":"thinkthen.config/1","usd_per_million_input":"0.25"}"#,
    );
    let listener = Listener::answering(answered).expect("listener");
    let refused = run(&root, &listener, b"Refund me.", &[]);
    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(listener.count(), 0);
    assert!(
        String::from_utf8_lossy(&refused.stderr)
            .contains("configuration prices require both input and output fields")
    );
}

#[test]
fn complete_replies_with_an_overflowed_sum_hide_tokens_and_cost_then_a_new_run_recovers() {
    let root = configuration(
        "facts-price-overflow",
        r#"{"schema":"thinkthen.config/1","usd_per_million_input":"1","usd_per_million_output":"0"}"#,
    );
    let seen = AtomicUsize::new(0);
    let overflow = Listener::answering(move |_| {
        let input = if seen.fetch_add(1, Ordering::SeqCst) == 0 { u64::MAX } else { 1 };
        Canned::ok(&json!({"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":input,"output_tokens":0}}).to_string())
    }).expect("listener");
    let lines = run(
        &root,
        &overflow,
        b"line 1\nline 2\n",
        &["--lines", "--batch", "1", "--jobs", "1"],
    );
    assert_eq!(lines.status.code(), Some(0));
    assert_eq!(overflow.count(), 2);
    let facts = line(&lines);
    assert_eq!(facts["requests_sent"], 2);
    for absent in ["input_tokens", "output_tokens", "estimated_cost_usd"] {
        assert!(facts.get(absent).is_none(), "{absent}");
    }
    let fresh = Listener::answering(|_| Canned::ok(r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":1,"output_tokens":0}}"#)).expect("listener");
    let recovered = run(&root, &fresh, b"line 3", &[]);
    assert_eq!(recovered.status.code(), Some(0));
    assert_eq!(fresh.count(), 1);
    assert_eq!(line(&recovered)["estimated_cost_usd"], "0.000001");
}

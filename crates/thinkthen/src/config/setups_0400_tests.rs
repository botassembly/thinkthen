//! Configuration edge and canonical setup selection contract for ticket 0400.
use super::Config;
use crate::core::named;
use serde_json::{Value, json};

fn parse(entries: Value) -> Config {
    Config::parse(
        json!({"schema":"thinkthen.config/1","backends":entries})
            .to_string()
            .as_bytes(),
    )
    .expect("valid setups")
}

#[test]
fn every_nonempty_builtin_subset_keeps_transport_and_selects_atomic_settings() {
    for name in ["typesafe", "liquid", "ollama", "perplexity", "openrouter"] {
        for bits in 1..8 {
            let mut fields = json!({});
            if bits & 1 != 0 {
                fields["requests_per_minute"] = json!(600);
            }
            if bits & 2 != 0 {
                fields["usd_per_million_input"] = json!("2");
                fields["usd_per_million_output"] = json!("0");
            }
            if bits & 4 != 0 {
                fields["profile"] = json!({"schema":"thinkthen.backend-profile/1","name":"small","max_questions":1});
            }
            let config = parse(json!({name:fields}));
            let selected = named::choose(&[(Some(name), None)], config.named()).unwrap();
            let backend = selected.backend(None, "unused").unwrap();
            let plain = named::choose(&[(Some(name), None)], &[])
                .unwrap()
                .backend(None, "unused")
                .unwrap();
            assert_eq!(backend.clone().with_per_minute(None), plain);
            let (prices, profile) = selected.setup(&backend);
            assert_eq!(
                prices.and_then(|pair| pair.estimate(9, 3)).as_deref(),
                (bits & 2 != 0).then_some("0.000018")
            );
            assert_eq!(
                profile.map(|p| p.max_questions),
                (bits & 4 != 0).then_some(Some(1))
            );
        }
    }
}

#[test]
fn unnamed_matches_final_posting_url_only_and_named_survives_overrides() {
    let profile = json!({"schema":"thinkthen.backend-profile/1","name":"small","max_questions":1});
    let config = parse(
        json!({"typesafe":{"usd_per_million_input":"2","usd_per_million_output":"0"}, "perplexity":{"profile":profile}, "alias":{"url":"http://127.0.0.1:8080/v1","key_env":"K","model":"m","profile":profile}}),
    );
    for (name, url, price, limits) in [
        (None, None, true, false),
        (None, Some("https://api.typesafe.ai/v1/"), true, false),
        (None, Some("https://api.typesafe.ai/other"), false, false),
        (None, Some("https://api.perplexity.ai/v1"), false, false),
        (None, Some("http://127.0.0.1:8080/v1"), false, false),
        (
            Some("perplexity"),
            Some("http://127.0.0.1:8080/alternate"),
            false,
            true,
        ),
        (Some("alias"), Some("http://127.0.0.1:9090/v1"), false, true),
    ] {
        let choice =
            named::choose(&[(name, url), (Some("perplexity"), None)], config.named()).unwrap();
        // A missing top tier deliberately selects the lower tier. Default matching has no tiers.
        let choice = if name.is_none() && url.is_none() {
            named::choose(&[], config.named()).unwrap()
        } else {
            choice
        };
        let backend = choice.backend(Some("override"), "unused").unwrap();
        let setup = choice.setup(&backend);
        assert_eq!(setup.0.is_some(), price, "{name:?} {url:?}");
        assert_eq!(setup.1.is_some(), limits, "{name:?} {url:?}");
    }
}

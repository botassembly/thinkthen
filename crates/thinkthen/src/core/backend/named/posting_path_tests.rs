//! Ticket 0399: built-in path grammar and exact posting identity for rates.
use super::{Named, choose, valid_path};
use crate::core::Backend;
use crate::core::adapters::built_in::backends::BUILT_INS;
use std::num::NonZeroU32;

#[test]
fn builtin_paths_satisfy_the_configuration_grammar() {
    for entry in BUILT_INS {
        assert!(valid_path(entry.path), "{}", entry.name);
    }
}

#[test]
fn a_rated_builtin_matches_its_own_posting_path_and_never_the_unnamed_path() {
    let rated = Named::built_in("perplexity")
        .expect("built-in")
        .with_per_minute(NonZeroU32::new(60));
    let configured = [rated];
    let named = choose(&[(Some("perplexity"), None)], &configured)
        .expect("choice")
        .backend(None, "m")
        .expect("backend");
    assert_eq!(
        named.url().as_str(),
        "https://api.perplexity.ai/v1/decisions"
    );
    assert_eq!(named.per_minute().map(u32::from), Some(60));
    assert!(!named.is_built_in());
    let unnamed = choose(&[(None, Some("https://api.perplexity.ai/v1"))], &configured)
        .expect("choice")
        .backend(None, "m")
        .expect("backend");
    assert_eq!(
        unnamed.url(),
        Backend::resolve(Some("https://api.perplexity.ai/v1"), None, "m")
            .expect("unnamed default")
            .url()
    );
    assert_eq!(unnamed.per_minute(), None);
    let default = Backend::resolve(None, None, "m").expect("default");
    assert!(default.is_built_in());
    let other_path = Backend::resolve_path(None, None, "m", "decisions").expect("path");
    assert!(!other_path.is_built_in());
}

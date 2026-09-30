//! Ticket 0318: a shared host caches only in a named folder, and refuses a
//! named folder that another user owns or others can write.

use super::*;

/// Ask the same question twice through a seeded shared-host engine and
/// report how the second answer arrived, or the build refusal.
pub(super) fn run_twice(argument: &str) -> Vec<String> {
    let mut builder = EngineBuilder::from_env().expect("a seed").shared_host();
    if let Some(folder) = argument.strip_prefix("cache_at=") {
        builder = builder.cache_at(folder).expect("a folder");
    } else if let Some(folder) = argument.strip_prefix("replay=") {
        builder = builder.replay(folder).expect("a folder");
    }
    match builder.build() {
        Ok(engine) => vec![ask(&engine), ask(&engine)],
        Err(error) => vec![format!("{:?}: {error}", error.kind())],
    }
}

#[cfg(unix)]
fn with_mode(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt as _;
    fs::create_dir_all(path).expect("folder");
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("mode");
}

const REFUSED: &str = "Usage: the answer folder belongs to another user or others can write it, so they could choose its answers; make it this process user's own with mode 0700, or name another folder";

#[cfg(unix)]
#[test]
fn a_shared_host_caches_only_in_a_private_named_folder() {
    let listener = listener();
    let base = listener.base().to_owned();
    let home = folder("shared-host");
    let xdg = home.join("xdg");
    let named = home.join("named");
    let open = home.join("open");
    let group = home.join("group");
    with_mode(&xdg, 0o700);
    with_mode(&open, 0o777);
    with_mode(&group, 0o770);
    let path = |p: &Path| p.to_str().expect("utf-8").to_owned();
    let (xdg_s, named_s, open_s, group_s) = (path(&xdg), path(&named), path(&open), path(&group));
    let cache_at_open = format!("cache_at={open_s}");
    let replay_open = format!("replay={open_s}");
    // Each row: the case, the named cache variable, the argument, and
    // whether the second ask came from a cache.
    let rows: [(&str, Option<&str>, &str, Option<bool>); 6] = [
        ("platform default stays off", None, "", Some(false)),
        (
            "THINKTHEN_CACHE names a private folder",
            Some(&named_s),
            "",
            Some(true),
        ),
        (
            "THINKTHEN_CACHE names a group-writable folder",
            Some(&group_s),
            "",
            Some(true),
        ),
        (
            "THINKTHEN_CACHE names an open folder",
            Some(&open_s),
            "",
            None,
        ),
        ("cache_at names an open folder", None, &cache_at_open, None),
        ("replay names an open folder", None, &replay_open, None),
    ];
    for (name, cache, argument, cached) in rows {
        let before = listener.count();
        let mut environment = vec![
            ("XDG_CACHE_HOME", xdg_s.as_str()),
            ("THINKTHEN_BASE_URL", base.as_str()),
            ("THINKTHEN_API_KEY", "sk-test"),
            (ARGUMENT, argument),
        ];
        if let Some(cache) = cache {
            environment.push(("THINKTHEN_CACHE", cache));
        }
        let lines = in_child("shared-host", &environment);
        let sends = listener.count() - before;
        match cached {
            None => {
                assert_eq!(lines, REFUSED, "{name}");
                assert_eq!(sends, 0, "{name}: a refused build sends nothing");
            }
            Some(cached) => {
                let second = lines.lines().nth(1).expect("two asks");
                assert!(
                    second.starts_with(&format!("sent {} cached {cached} ", u8::from(!cached))),
                    "{name}: {lines}"
                );
                assert_eq!(sends, if cached { 1 } else { 2 }, "{name}");
            }
        }
    }
    assert_eq!(entries(&xdg), 0, "the platform folder stays empty");
}

#[cfg(unix)]
#[test]
fn an_ordinary_engine_keeps_an_open_named_folder() {
    let open = folder("ordinary-open");
    with_mode(&open, 0o777);
    let built = Engine::builder()
        .api_key("sk-test")
        .and_then(|builder| builder.cache_at(&open))
        .and_then(EngineBuilder::build);
    assert!(built.is_ok(), "only a shared host refuses the folder");
}

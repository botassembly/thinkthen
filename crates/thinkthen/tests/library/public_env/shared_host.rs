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
    let home = folder("shared-host");
    let at = |name: &str, mode: u32| {
        let path = home.join(name);
        with_mode(&path, mode);
        path.to_str().expect("utf-8").to_owned()
    };
    let (xdg, named, group, open) = (
        at("xdg", 0o700),
        at("named", 0o700),
        at("group", 0o770),
        at("open", 0o777),
    );
    let (cache_at, replay) = (format!("cache_at={open}"), format!("replay={open}"));
    // The case, THINKTHEN_CACHE (empty is unset), the argument, and the
    // sends for two equal asks; `None` is a refused build.
    let rows = [
        ("the platform default stays off", "", "", Some(2)),
        ("a private named folder caches", named.as_str(), "", Some(1)),
        (
            "a group-writable named folder caches",
            group.as_str(),
            "",
            Some(1),
        ),
        (
            "an open THINKTHEN_CACHE is refused",
            open.as_str(),
            "",
            None,
        ),
        ("an open cache_at is refused", "", cache_at.as_str(), None),
        ("an open replay is refused", "", replay.as_str(), None),
    ];
    for (name, cache, argument, sends) in rows {
        let before = listener.count();
        let lines = in_child(
            "shared-host",
            &[
                ("XDG_CACHE_HOME", &xdg),
                ("THINKTHEN_BASE_URL", listener.base()),
                ("THINKTHEN_API_KEY", "sk-test"),
                ("THINKTHEN_CACHE", cache),
                (ARGUMENT, argument),
            ],
        );
        assert_eq!(
            listener.count() - before,
            sends.unwrap_or(0),
            "{name}: {lines}"
        );
        assert_eq!(sends.is_none(), lines == REFUSED, "{name}: {lines}");
    }
    let names: Vec<_> = fs::read_dir(&xdg)
        .expect("the platform folder")
        .map(|entry| entry.expect("an entry").file_name())
        .collect();
    assert_eq!(
        names,
        ["thinkthen-usage"],
        "the platform folder holds only the usage totals"
    );
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

#[cfg(unix)]
#[test]
fn a_shared_host_creates_a_missing_named_folder_private_at_build() {
    use std::os::unix::fs::PermissionsExt as _;
    // Each row: the case, the mode the test gives the folder first (none
    // leaves it missing), and whether the build succeeds. The build keeps
    // a given mode and creates a missing folder with mode 0700.
    let rows: [(&str, Option<u32>, bool); 2] = [
        ("a missing folder is created private", None, true),
        ("an other-writable folder is refused", Some(0o777), false),
    ];
    for (name, given, builds) in rows {
        let folder = folder("shared-host-build").join("answers");
        if let Some(mode) = given {
            with_mode(&folder, mode);
        }
        let built = Engine::builder()
            .api_key("sk-test")
            .and_then(|builder| builder.cache_at(&folder))
            .map(EngineBuilder::shared_host)
            .and_then(EngineBuilder::build);
        assert_eq!(built.is_ok(), builds, "{name}");
        let mode = fs::metadata(&folder)
            .expect("a folder")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, given.unwrap_or(0o700), "{name}");
    }
}

use super::*;

#[cfg(feature = "cli")]
#[test]
fn cli_rank_sets_admit_every_member_declaration_before_lookup_or_send() {
    let root = Folder::new().unwrap();
    let set = root.path().join("rank.json");
    fs::write(&set, r#"{"version":1,"questions":{"first":{"decide":"Ready?"},"second":{"decide":"Useful?","item_schema":{"type":"object","properties":{}}}}}"#).unwrap();
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let input = root.path().join("items.txt");
    fs::write(&input, "alpha\nbeta\n").unwrap();
    let cache = root.path().join("saved");
    fs::create_dir(&cache).unwrap();
    // An empty replay returns a miss if declaration admission reaches lookup.
    fs::write(cache.join("thinkthen.jsonl"), "").unwrap();
    for mode in ["--no-cache", "--replay"] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
        command.clear_environment();
        child_environment(&mut command, root.path());
        command.args([
            "rank",
            &format!("@{}", set.display()),
            "--batch",
            "2",
            "--url",
            listener.base(),
            "--model",
            "fixed",
            "--max-retries",
            "0",
        ]);
        if mode == "--replay" {
            command.arg(mode).arg(&cache);
        } else {
            command.arg(mode);
        }
        let output = command
            .env("THINKTHEN_API_KEY", "rank-private")
            .stdin(fs::File::open(&input).unwrap())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert_eq!(
            String::from_utf8(output.stderr).unwrap(),
            format!(
                "thinkthen: the item does not match item_schema\nthinkthen: stopped at record 1; 0 records finished{}, and nothing was printed because an order needs every record\n",
                if mode == "--replay" {
                    ", 0 records from a recording"
                } else {
                    ""
                }
            )
        );
        assert_eq!(listener.count(), 0);
        assert!(!cache.join("thinkthen.sqlite").exists());
        assert_eq!(
            fs::read_to_string(cache.join("thinkthen.jsonl")).unwrap(),
            ""
        );
    }
}

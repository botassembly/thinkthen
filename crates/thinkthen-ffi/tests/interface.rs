#![cfg(test)]
//! A compiled C program gets each answer in one call and keeps only kept records.
mod support;
use loopback::{KEY, Loopback, Reply};
use serde_json::{Value, json};

fn program(mode: &str, server: Option<&Loopback>) -> String {
    let mut command = support::command("thinkthen-interface");
    command.arg(mode);
    // The program opens its client with no settings, so the environment picks the backend.
    match server {
        Some(server) => command
            .env("THINKTHEN_BACKEND", "systemone")
            .env("THINKTHEN_BASE_URL", server.base())
            .env("THINKTHEN_API_KEY", KEY),
        None => command.env("THINKTHEN_BACKEND", "stub"),
    };
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn case(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../conformance/typed")
        .join(format!("{name}.json"));
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn decide_answers_in_one_call_and_refuses_blank_text_on_the_stub() {
    let output = program("decide", None);
    let lines: Vec<_> = output.lines().collect();
    assert!(lines[0] == "decide rc=0 answer=1" || lines[0] == "decide rc=0 answer=0");
    assert_eq!(lines[1], "blank rc=-2005 code=2005 answer=1");
    assert_eq!(lines[2], "invalid rc=-1012 code=1012 answer=1");
}

#[test]
fn decide_writes_the_backend_answer_and_never_sends_blank_text() {
    let server = Loopback::noul(0.9).unwrap();
    assert_eq!(
        program("decide", Some(&server)),
        "decide rc=0 answer=1\nblank rc=-2005 code=2005 answer=1\ninvalid rc=-1012 code=1012 answer=1\n"
    );
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn decide_band_follows_each_bar_and_sends_nothing_it_refuses() {
    let server = Loopback::noul(0.5).unwrap();
    assert_eq!(
        program("band", Some(&server)),
        "band 0.6 0.8 rc=0 answer=0\n\
         band 0.4 0.8 rc=4 answer=1\n\
         band 0.4 0.8 rc=4 answer=0\n\
         band 0.2 0.4 rc=0 answer=1\n\
         band 0.8 0.2 rc=-1001 answer=1\n\
         client rc=-1017 code=1017 answer=1\n\
         question rc=-1017 code=1017 answer=1\n\
         text rc=-2005 code=2005 answer=1\n\
         answer rc=-1017 code=1017\n\
         wording rc=-1012 code=1012 answer=1\n\
         invalid rc=-2004 code=2004 answer=1\n\
         blank rc=-2005 code=2005 answer=1\n"
    );
    assert_eq!(server.requests().len(), 4);
}

#[test]
fn choose_writes_the_chosen_index_and_leaves_it_alone_when_not_sure() {
    let server = Loopback::case(&json!({"replies":[
        {"contains":"first note","distribution":{"billing":0.1,"shipping":0.8,"other":0.1}},
        {"contains":"second note","distribution":{"billing":0.4,"shipping":0.3,"other":0.3}}
    ]}))
    .unwrap();
    assert_eq!(
        program("choose", Some(&server)),
        "choose rc=0 team=1\nunsure rc=4 team=99\n"
    );
}

#[test]
fn tag_marks_each_label_that_applies() {
    let server = Loopback::case(&json!({"replies":[
        {"contains":"label \"billing\"","probability":0.7},
        {"contains":"label \"refund\"","probability":0.2},
        {"contains":"label \"shipping\"","probability":0.9}
    ]}))
    .unwrap();
    assert_eq!(program("tag", Some(&server)), "tag rc=0 applies=101\n");
}

#[test]
fn score_writes_the_weighted_score() {
    let server = Loopback::case(&case("score-weighted")).unwrap();
    assert_eq!(program("score", Some(&server)), "score rc=0 urgency=0.80\n");
}

#[test]
fn find_points_into_the_callers_own_text_in_bytes() {
    let server =
        Loopback::case(&json!({"replies":[{"distribution":{"u1":0.0,"u2":1.0,"none":0.0}}]}))
            .unwrap();
    assert_eq!(
        program("find", Some(&server)),
        "find rc=0 present=1 start=5 end=8 inside=1 text=Ada\n"
    );
}

#[test]
fn recognize_and_relate_return_values_read_by_the_existing_accessors() {
    let server = Loopback::case(&case("recognize-name")).unwrap();
    assert_eq!(
        program("recognize", Some(&server)),
        "recognize rc=0 count=1 text=Ada kind=person\n"
    );
    let server = Loopback::case(&case("relate-either")).unwrap();
    assert_eq!(
        program("relate", Some(&server)),
        "relate rc=0 count=1 source=Ada target=Acme\n"
    );
}

#[test]
fn annotate_answers_are_found_by_name() {
    let server = Loopback::case(&json!({"replies":[
        {"criterion":"billing","distribution":{"billing":0.8,"shipping":0.2}},
        {"probability":0.1}
    ]}))
    .unwrap();
    assert_eq!(
        program("annotate", Some(&server)),
        "annotate rc=0 topic=billing unknown=-1003 missing=1\n"
    );
}

#[test]
fn filter_gives_kept_records_before_its_aggregate_and_failed_rows() {
    let server = Loopback::case(&case("filter-rows")).unwrap();
    assert_eq!(
        program("filter", Some(&server)),
        "rc=0 position=1 text=refund\nfailed rc=-2002\nrc=-2002 position=4 text=\nrc=-2005 position=5 text=   \n"
    );
}

#[test]
fn answer_meanings_and_descriptions_reach_the_backend_and_unknown_names_are_refused() {
    let server = Loopback::case(&json!({"replies":[
        {"contains":"A refund","distribution":{"billing":0.9,"shipping":0.1}},
        {"probability":0.9}
    ]}))
    .unwrap();
    assert_eq!(
        program("meanings", Some(&server)),
        "unknown rc=-1002\nchoose yes rc=-1002\nready rc=0 answer=1\nteam rc=0 answer=\"billing\"\n"
    );
    let sent: String = server
        .requests()
        .iter()
        .map(|request| request.json().unwrap().to_string())
        .collect();
    for meaning in ["ready to ship", "still failing", "charges and refunds"] {
        assert!(sent.contains(meaning), "{meaning} was not sent");
    }
}

#[test]
fn client_settings_keep_environment_addresses_with_their_provider() {
    for backend in ["systemone", "openai"] {
        for explicit_backend in [false, true] {
            for explicit_url in [false, true] {
                let system = Loopback::noul(0.9).unwrap();
                let openai = Loopback::start(|_| {
                    Reply::json(
                        200,
                        &json!({"answers":[{"type":"predicate","probability":0.9}]}),
                    )
                })
                .unwrap();
                let (server, other) = if backend == "systemone" {
                    (&system, &openai)
                } else {
                    (&openai, &system)
                };
                let mut settings = json!({});
                if explicit_backend {
                    settings["backend"] = backend.into();
                }
                if explicit_url {
                    settings["url"] = server.base().into();
                }
                let output = support::command("thinkthen-interface")
                    .args(["decide", &settings.to_string()])
                    .env(
                        "THINKTHEN_BACKEND",
                        if explicit_backend { "unknown" } else { backend },
                    )
                    .env("THINKTHEN_API_KEY", "synthetic-system")
                    .env("OPENAI_API_KEY", "synthetic-openai")
                    .env(
                        "THINKTHEN_URL",
                        if explicit_url {
                            other.base()
                        } else {
                            system.base()
                        },
                    )
                    .env(
                        "OPENAI_BASE_URL",
                        if explicit_url {
                            other.base()
                        } else {
                            openai.base()
                        },
                    )
                    .output()
                    .unwrap();
                assert_eq!(other.count(), 0);
                assert!(output.status.success());
                assert!(output.stdout.starts_with(b"decide rc=0 answer=1\n"));
                assert!(output.stderr.is_empty());
                let requests = server.requests();
                assert_eq!(requests.len(), 1);
                let key = if backend == "systemone" {
                    "synthetic-system"
                } else {
                    "synthetic-openai"
                };
                assert!(
                    requests[0].header("authorization") == Some(format!("Bearer {key}").as_str())
                );
            }
        }
    }
}

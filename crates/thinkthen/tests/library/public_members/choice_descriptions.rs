//! Typed choice descriptions at the public request boundary.

use super::*;

thinkthen::choices! {
    enum DescribedTeam {
        Billing => "billing": "Money owed.",
        Outage => "outage": { thinkthen::Description::builder().what("Service unavailable.")?.example("Cannot sign in.")?.build() },
        Other => "other",
    }
}

thinkthen::choices! {
    enum BadTeam { Billing => "billing": "  ", Outage => "outage" }
}
#[test]
fn typed_choice_descriptions_keep_old_identity_and_explicit_precedence() {
    let listener = Listener::answering(|body| {
        if String::from_utf8_lossy(body).contains(r#""type":"choice""#)
            && String::from_utf8_lossy(body).contains(r#""other":null"#)
        {
            Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"billing","probabilities":{"billing":0.9,"outage":0.1,"other":0.0}}}}"#)
        } else if String::from_utf8_lossy(body).contains(r#""type":"choice""#) {
            Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"choice","choice":"billing","probabilities":{"billing":0.9,"outage":0.1}}}}"#)
        } else {
            Canned::ok(r#"{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1},"q3":{"type":"noul","noul":0.1}}}"#)
        }
    })
    .expect("listener");
    let engine = engine(listener.base());
    old_bare_identity(&engine, &listener);
    described_choose_identity(&engine, &listener);
    described_tag_and_null_identity(&engine, &listener);
    invalid_metadata_and_unknown_map(&listener);
}

fn captured_pair(listener: &Listener) -> (Vec<u8>, Vec<u8>) {
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let mut requests = requests.into_iter();
    let first = requests.next().expect("first request").body;
    let second = requests.next().expect("second request").body;
    (first, second)
}

fn old_bare_identity(engine: &Engine, listener: &Listener) {
    let old = Question::choose::<Team>("Which team?")
        .and_then(|builder| builder.option(Team::Billing, None))
        .and_then(|builder| builder.option(Team::Outage, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("old bare choice");
    let details = engine
        .details(&old, "An invoice dispute.")
        .expect("details");
    assert_eq!(listener.count(), 1);
    let request = listener.requests().pop().expect("captured request");
    assert_eq!(
        String::from_utf8(request.body).expect("UTF-8 body"),
        r#"{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"The text is \"An invoice dispute.\". Which team?","criteria":{"billing":null,"outage":null}}}}"#
    );
    assert_eq!(
        details.value().question_sha256(),
        "768f400eee4e2fc2f8d63204d7c3ca381a58fb70d43b7b51441b44d566fbb216"
    );
}

fn described_choose_identity(engine: &Engine, listener: &Listener) {
    let typed = Question::choose::<DescribedTeam>("Which team?")
        .and_then(|builder| builder.option(DescribedTeam::Billing, None))
        .and_then(|builder| builder.option(DescribedTeam::Outage, None))
        .and_then(|builder| builder.option(DescribedTeam::Other, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("described choice");
    let loaded_choice = loaded(r#"{"choose":"Which team?","options":{"billing":"Money owed.","outage":{"what":"Service unavailable.","examples":["Cannot sign in."]},"other":null}}"#)
        .into_choose::<DescribedTeam>()
        .expect("bound described map");
    let typed_details = engine
        .details(&typed, "An invoice dispute.")
        .expect("typed details");
    let loaded_details = engine
        .details(&loaded_choice, "An invoice dispute.")
        .expect("map details");
    let (typed_body, map_body) = captured_pair(listener);
    assert_eq!(typed_body, map_body);
    assert!(String::from_utf8_lossy(&typed_body).contains(r#""criteria":{"billing":"Money owed.","outage":{"what":"Service unavailable.","examples":["Cannot sign in."]},"other":null}"#));
    assert_eq!(
        typed_details.value().question_sha256(),
        loaded_details.value().question_sha256()
    );
    assert_eq!(
        engine
            .choose(&typed, "An invoice dispute.")
            .expect("typed choice")
            .into_value(),
        Some(DescribedTeam::Billing)
    );
    assert_eq!(listener.requests().len(), 1);

    let explicit = thinkthen::Description::text("A payment question.").expect("override");
    let overridden = Question::choose::<DescribedTeam>("Which team?")
        .and_then(|builder| builder.option(DescribedTeam::Billing, Some(explicit)))
        .and_then(|builder| builder.option(DescribedTeam::Outage, None))
        .and_then(|builder| builder.option(DescribedTeam::Other, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("per-label override");
    let mapped = loaded(r#"{"choose":"Which team?","options":{"billing":"A payment question.","outage":{"what":"Service unavailable.","examples":["Cannot sign in."]},"other":null}}"#)
        .into_choose::<DescribedTeam>()
        .expect("bound override map");
    let override_details = engine
        .details(&overridden, "An invoice dispute.")
        .expect("override details");
    let map_details = engine
        .details(&mapped, "An invoice dispute.")
        .expect("map details");
    let (typed_body, map_body) = captured_pair(listener);
    assert_eq!(typed_body, map_body);
    assert_eq!(
        override_details.value().question_sha256(),
        map_details.value().question_sha256()
    );
}

fn described_tag_and_null_identity(engine: &Engine, listener: &Listener) {
    let tagged = Question::tag::<DescribedTeam>("Which teams?")
        .and_then(|builder| builder.label(DescribedTeam::Billing, None))
        .and_then(|builder| builder.label(DescribedTeam::Outage, None))
        .and_then(|builder| builder.label(DescribedTeam::Other, None))
        .and_then(thinkthen::TagBuilder::cut)
        .expect("described tag");
    let tagged_map = loaded(r#"{"tag":"Which teams?","labels":{"billing":"Money owed.","outage":{"what":"Service unavailable.","examples":["Cannot sign in."]},"other":null}}"#)
        .into_tag::<DescribedTeam>()
        .expect("bound tag map");
    let tag_details = engine
        .details(&tagged, "An invoice dispute.")
        .expect("tag details");
    let map_tag_details = engine
        .details(&tagged_map, "An invoice dispute.")
        .expect("map tag details");
    let (typed_body, map_body) = captured_pair(listener);
    assert_eq!(typed_body, map_body);
    assert_eq!(
        tag_details.value().question_sha256(),
        map_tag_details.value().question_sha256()
    );
    assert_eq!(
        engine
            .tag(&tagged, "An invoice dispute.")
            .expect("typed tag")
            .into_value(),
        vec![DescribedTeam::Billing]
    );
    assert_eq!(listener.requests().len(), 1);

    let null_map =
        loaded(r#"{"choose":"Which team?","options":{"billing":null,"outage":null,"other":null}}"#)
            .into_choose::<DescribedTeam>()
            .expect("null map binding");
    let bare_list = loaded(r#"{"choose":"Which team?","options":["billing","outage","other"]}"#)
        .into_choose::<DescribedTeam>()
        .expect("bare list binding");
    let null_details = engine
        .details(&null_map, "An invoice dispute.")
        .expect("null map details");
    let list_details = engine
        .details(&bare_list, "An invoice dispute.")
        .expect("bare list details");
    let (null_body, list_body) = captured_pair(listener);
    assert_eq!(null_body, list_body);
    assert!(
        String::from_utf8_lossy(&null_body)
            .contains(r#""criteria":{"billing":null,"outage":null,"other":null}"#)
    );
    assert_eq!(
        null_details.value().question_sha256(),
        list_details.value().question_sha256()
    );
}

fn invalid_metadata_and_unknown_map(listener: &Listener) {
    let before = listener.count();
    let invalid = Question::choose::<BadTeam>("Which team?")
        .and_then(|builder| builder.option(BadTeam::Billing, None))
        .expect_err("selected blank metadata");
    assert_eq!(invalid.kind(), ErrorKind::Usage);
    assert_eq!(
        invalid.to_string(),
        "a description is text, not white space"
    );
    let valid = thinkthen::Description::text("A payment question.").expect("override");
    Question::choose::<BadTeam>("Which team?")
        .and_then(|builder| builder.option(BadTeam::Billing, Some(valid)))
        .and_then(|builder| builder.option(BadTeam::Outage, None))
        .and_then(thinkthen::ChooseBuilder::build)
        .expect("unselected blank metadata is bypassed");
    let unknown = loaded(
        r#"{"choose":"Which team?","options":{"billing":null,"outage":null,"unknown":null}}"#,
    )
    .into_choose::<DescribedTeam>()
    .expect_err("unknown map label");
    assert_eq!(unknown.kind(), ErrorKind::Usage);
    assert_eq!(
        unknown.to_string(),
        "the question's labels are not the choice's labels in order"
    );
    assert_eq!(listener.count(), before);
}

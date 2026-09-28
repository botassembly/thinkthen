//! Pure request packing for one compatible annotate group slice.

use super::{Batch, BatchError, BatchRecord, Batcher, Setting};
use crate::core::{Backend, BackendProfile, Question};

/// One input record's named logical questions and their decoded places.
pub(crate) struct GroupMember {
    pub(crate) outcomes: Vec<usize>,
}

/// One open group slice, using the ordinary batcher's cuts and byte counts.
pub(crate) struct GroupBatcher(Batcher);

impl GroupBatcher {
    /// Prepare one compatible, ordered question slice. Each push selects one record.
    pub(crate) fn new(
        backend: Backend,
        profile: Option<BackendProfile>,
        questions: Vec<Question>,
        setting: Setting,
    ) -> Result<Self, BatchError> {
        if questions.is_empty() {
            return Err(BatchError::Defect("an annotate group asks nothing"));
        }
        Batcher::new_slice(backend, profile, questions, setting, None, true).map(Self)
    }

    /// Add one selected record and retain any requests closed by this push.
    pub(crate) fn push(
        &mut self,
        record: BatchRecord,
        closed: &mut Vec<Batch>,
    ) -> Result<(), BatchError> {
        self.0.push_slice(record, self.0.question.clone(), closed)
    }

    /// Close the final valid prefix at input end or local refusal.
    pub(crate) fn finish(&mut self) -> Result<Option<Batch>, BatchError> {
        self.0.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Plan;
    use crate::core::adapters::built_in;
    use crate::core::question::Labels;
    use crate::core::recording::Exchange;
    use crate::core::text::{Evidence, QuestionText};

    fn question(text: &str) -> Question {
        Question::Decide {
            text: QuestionText::new(text).expect("question"),
            yes: None,
            no: None,
        }
    }

    fn record(text: &str) -> BatchRecord {
        let evidence = Evidence::new(text).expect("evidence");
        BatchRecord {
            value: evidence.as_json(),
            evidence,
        }
    }

    #[test]
    fn two_records_keep_each_named_logical_place_and_singleton_bytes() {
        let backend =
            Backend::resolve(Some("http://127.0.0.1:9"), None, "jev-latest").expect("backend");
        let questions = vec![question("First?"), question("Second?")];
        let mut packed = GroupBatcher::new(backend.clone(), None, questions.clone(), Setting::Max)
            .expect("group");
        let mut closed = Vec::new();
        packed.push(record("alpha"), &mut closed).expect("alpha");
        packed.push(record("beta"), &mut closed).expect("beta");
        assert!(closed.is_empty());
        let packed = packed.finish().expect("finish").expect("packed");
        let expected = r#"{"state":{"records":["alpha","beta"]},"model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". First?"},"q2":{"type":"noul","instructions":"The text is \"alpha\". Second?"},"q3":{"type":"noul","instructions":"The text is \"beta\". First?"},"q4":{"type":"noul","instructions":"The text is \"beta\". Second?"}}}"#;
        assert_eq!(packed.body, expected.as_bytes());
        assert_eq!(
            packed.digest,
            Exchange::new(backend.url(), expected.as_bytes()).digest()
        );
        let members = packed.group_members.expect("group members");
        assert_eq!(
            members
                .iter()
                .map(|one| one.outcomes.as_slice())
                .collect::<Vec<_>>(),
            [&[0, 1][..], &[2, 3][..]]
        );

        let mut singleton = GroupBatcher::new(
            backend.clone(),
            None,
            questions.clone(),
            Setting::Records(std::num::NonZeroUsize::MIN),
        )
        .expect("group");
        let mut closed = Vec::new();
        singleton.push(record("alpha"), &mut closed).expect("alpha");
        let [alone] = closed.as_slice() else {
            panic!("one closed group")
        };
        let plan = Plan::new(
            Evidence::new("alpha").expect("evidence"),
            backend.model().clone(),
            questions,
        )
        .expect("original plan");
        assert_eq!(alone.body, built_in::encode(&plan).expect("original body"));
    }

    #[test]
    fn copies_share_a_complete_slice_and_tag_wires_count_against_the_profile() {
        let backend =
            Backend::resolve(Some("http://127.0.0.1:9"), None, "jev-latest").expect("backend");
        let tag = Question::Tag {
            text: QuestionText::new("Topics?").expect("question"),
            labels: Labels::tags(["a", "b", "c"].map(|label| (label.to_owned(), None)).into())
                .expect("labels"),
        };
        let questions = vec![question("First?"), tag];
        let mut duplicates =
            GroupBatcher::new(backend.clone(), None, questions.clone(), Setting::Max)
                .expect("group");
        let mut closed = Vec::new();
        duplicates
            .push(record("alpha"), &mut closed)
            .expect("first");
        duplicates.push(record("alpha"), &mut closed).expect("copy");
        let duplicate = duplicates.finish().expect("finish").expect("batch");
        assert_eq!(duplicate.plan.questions().len(), 2);
        assert_eq!(duplicate.plan.wire_question_count(), 4);
        assert_eq!(
            duplicate
                .group_members
                .expect("members")
                .iter()
                .map(|one| one.outcomes.as_slice())
                .collect::<Vec<_>>(),
            [&[0, 1][..], &[0, 1][..]]
        );

        let profile = BackendProfile::parse(
            r#"{"schema":"thinkthen.backend-profile/1","name":"test","max_questions":7}"#,
        )
        .expect("profile");
        let mut limited =
            GroupBatcher::new(backend, Some(profile), questions, Setting::Max).expect("group");
        let mut closed = Vec::new();
        limited.push(record("alpha"), &mut closed).expect("first");
        limited.push(record("beta"), &mut closed).expect("second");
        closed.extend(limited.finish().expect("finish"));
        assert_eq!(closed.len(), 2, "two slices need eight expanded wires");
        assert!(
            closed
                .iter()
                .all(|batch| batch.plan.wire_question_count() == 4)
        );
    }
}

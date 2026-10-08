//! Find keeps the complete-set boundary while bounding caller-side source admission.
use super::*;
use thinkthen::{CancelToken, RecordInput};
#[test]
fn cancelled_find_stops_before_pulling_a_suffix_in_complete_and_released_calls() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Which?").unwrap();
    for complete in [false, true] {
        let token = CancelToken::new();
        let pulls = AtomicUsize::new(0);
        let units = std::iter::from_fn(|| {
            let at = pulls.fetch_add(1, Ordering::Relaxed);
            if at == 3 {
                return None;
            }
            token.cancel();
            Some(Ok("Plain."))
        });
        let options = CallOptions::new().cancel(&token);
        let error = if complete {
            engine
                .try_find_complete_with(&question, units, options)
                .map(|_| ())
                .unwrap_err()
        } else {
            engine
                .try_find_with(&question, units, options)
                .map(|_| ())
                .unwrap_err()
        };
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(
            pulls.load(Ordering::Relaxed),
            1,
            "cancelled admission pulled the suffix"
        );
        assert!(error.facts().is_none());
        assert_eq!(listener.count(), 0);
    }
}
#[test]
fn pre_cancelled_find_never_pulls_units_in_complete_or_released_calls() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Which?").unwrap();
    for complete in [false, true] {
        let token = CancelToken::new();
        token.cancel();
        let pulls = AtomicUsize::new(0);
        let units = (0..3).map(|_| {
            pulls.fetch_add(1, Ordering::Relaxed);
            Ok("Plain.")
        });
        let options = CallOptions::new().cancel(&token);
        let error = if complete {
            engine
                .try_find_complete_with(&question, units, options)
                .map(|_| ())
                .unwrap_err()
        } else {
            engine
                .try_find_with(&question, units, options)
                .map(|_| ())
                .unwrap_err()
        };
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(pulls.load(Ordering::Relaxed), 0);
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}
#[test]
fn composed_find_admission_stops_at_one_excess_candidate_without_dropping_the_count_error() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = engine(&listener);
    for none in [false, true] {
        let question = if none {
            Question::find("Which?").unwrap().offering_none().unwrap()
        } else {
            Question::find("Which?").unwrap()
        };
        let pulls = AtomicUsize::new(0);
        let units = (0..1_000).map(|_| {
            pulls.fetch_add(1, Ordering::Relaxed);
            Ok(RecordInput {
                examples: None,
                original: "Plain.",
                context: None,
                options: None,
            })
        });
        let error = engine
            .try_find_records_complete_with(&question, units, CallOptions::new())
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.detail().message(),
            if none {
                "a find question offering none takes 2 to 254 units"
            } else {
                "find takes 2 to 255 units"
            }
        );
        assert_eq!(pulls.load(Ordering::Relaxed), if none { 255 } else { 256 });
        assert!(error.facts().is_none());
    }
    assert_eq!(listener.count(), 0);
}

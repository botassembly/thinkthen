//! Stays: queue lock/wake ordering and bounded prefetch need private state inspection.
use super::*;
use crate::public::readiness::Wake;
use crate::{RequestFraming, RequestImage, RequestOriginal};
use std::{sync::{Arc, mpsc}, time::Duration};

fn descriptor(text: &str) -> RequestSessionDescriptor {
    RequestSessionDescriptor { item: super::super::RequestItem {
        original: Some(RequestOriginal::Text { text: text.to_owned() }), images: Vec::new(),
        context: None, options: None, seed_spans: None, examples: None,
    }, location: None }
}
fn push(queue: &Queue, text: &str) {
    assert!(matches!(queue.push(descriptor(text)), RequestSessionPush::Accepted));
}
#[test]
fn headers_reschedule_buffered_work_and_staging_retains_only_one_logical_record() {
    let queue = Arc::new(Queue::default());
    queue.configure_input(RequestFraming::Csv);
    let weak = Arc::downgrade(&queue);
    let for_wake = weak.clone();
    let (events, notified) = mpsc::channel();
    let wake = Wake::default();
    wake.register(Box::new(move || {
        if let Some(queue) = for_wake.upgrade() {
            assert!(queue.state.try_lock().is_ok(), "wake runs after releasing queue state");
            assert!(queue.logical.try_lock().is_ok(), "wake runs after releasing logical input");
        }
        events.send(()).unwrap();
    }));
    queue.register_input_wake(Some(wake));
    push(&queue, "note\n");
    notified.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(!queue.logical_ready(CallOptions::new()));
    push(&queue, "first\n");
    notified.recv_timeout(Duration::from_secs(5)).unwrap();
    // This is the header-consumption race's already-buffered-work branch:
    // rescheduling must notify even when the last push notification is gone.
    queue.reschedule_input();
    notified.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(queue.logical_ready(CallOptions::new()));
    push(&queue, "second\n");
    notified.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(queue.logical_ready(CallOptions::new()));
    assert!(queue.state.lock().unwrap().input.is_some(), "one staged row does not drain the raw cell");
    let first = queue.next_decoded(CallOptions::new()).unwrap().unwrap();
    let Some(RequestOriginal::Json { value }) = first.item.original else { panic!("decoded table row") };
    assert_eq!(value.content().unwrap().member("note").unwrap().text(), Some("first"));
    assert!(queue.logical_ready(CallOptions::new()));
    let _second = queue.next_decoded(CallOptions::new()).unwrap().unwrap();
    assert!(!queue.logical_ready(CallOptions::new()));
    assert_eq!(notified.try_recv(), Err(mpsc::TryRecvError::Empty), "empty readiness does not spin");
    queue.finish(None).unwrap();
    notified.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(queue.logical_ready(CallOptions::new()));
    assert!(queue.next_decoded(CallOptions::new()).is_none());
    queue.register_input_wake(None);
    drop(queue);
    assert!(weak.upgrade().is_none(), "decoder and wake retain no queue cycle");
}

#[test]
fn staged_errors_are_deferred_and_staging_never_opens_attachments() {
    let queue = Queue::default();
    queue.configure_input(RequestFraming::Jsonl);
    push(&queue, "{\"private-marker\":");
    assert!(queue.logical_ready(CallOptions::new()));
    let error = queue.next_decoded(CallOptions::new()).unwrap().unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-marker"));
    assert!(queue.next_decoded(CallOptions::new()).is_none());
    let queue = Queue::default();
    queue.configure_input(RequestFraming::Document);
    let mut item = descriptor("first");
    item.item.images.push(RequestImage::File { path: "absent-staged-image".into(), media: None });
    assert!(matches!(queue.push(item), RequestSessionPush::Accepted));
    assert!(queue.logical_ready(CallOptions::new()));
    assert_eq!(queue.next_decoded(CallOptions::new()).unwrap().unwrap().item.images.len(), 1);
}

#[test]
fn cancel_and_receiver_drop_notify_without_holding_queue_locks() {
    for drop_receiver in [false, true] {
        let queue = Arc::new(Queue::default());
        queue.configure_input(RequestFraming::Csv);
        let weak = Arc::downgrade(&queue);
        let (events, notified) = mpsc::channel();
        let wake = Wake::default();
        wake.register(Box::new(move || {
            let queue = weak.upgrade().unwrap();
            assert!(queue.state.try_lock().is_ok());
            assert!(queue.logical.try_lock().is_ok());
            events.send(()).unwrap();
        }));
        queue.register_input_wake(Some(wake));
        queue.stop(drop_receiver);
        notified.recv_timeout(Duration::from_secs(5)).unwrap();
        assert!(queue.logical_ready(CallOptions::new().cancel(&queue.cancel)));
        assert_eq!(queue.next_decoded(CallOptions::new().cancel(&queue.cancel)).unwrap().unwrap_err().kind(), crate::ErrorKind::Cancelled);
        queue.register_input_wake(None);
    }
}

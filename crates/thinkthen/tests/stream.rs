#![cfg(test)]
//! Stream consumers receive pushed rows and completion through the public trait.
#![cfg(feature = "stream")]
use thinkthen::{BackendName, Record, Settings, questions, results::Status};
#[test]
fn a_registered_stream_waker_observes_published_rows_and_completion() {
    use futures_core::Stream;
    use std::{
        pin::Pin,
        sync::{Arc, Mutex, mpsc},
        task::{Context, Poll, Wake, Waker},
    };
    #[derive(Debug)]
    struct Signal(Mutex<mpsc::Sender<()>>);
    impl Wake for Signal {
        fn wake(self: Arc<Self>) {
            let _ = self.0.lock().unwrap().send(());
        }
    }
    let (send, receive) = mpsc::channel();
    let waker = Waker::from(Arc::new(Signal(Mutex::new(send))));
    let mut cx = Context::from_waker(&waker);
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let call = engine
        .open(&questions::Decide::new("Ready?").unwrap())
        .unwrap();
    let sender = call.sender();
    let mut rows = call.rows();
    assert!(Pin::new(&mut rows).poll_next(&mut cx).is_pending());
    sender.send(Record::missing()).unwrap();
    sender.close();
    let mut seen = 0;
    loop {
        match Pin::new(&mut rows).poll_next(&mut cx) {
            Poll::Ready(Some(row)) => {
                assert_eq!(row.unwrap().status(), Status::Failed);
                seen += 1;
            }
            Poll::Ready(None) => break,
            Poll::Pending => {
                receive.recv().unwrap();
            }
        }
    }
    assert_eq!(seen, 1);
}

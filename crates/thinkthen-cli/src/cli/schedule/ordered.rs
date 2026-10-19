//! Legacy test fixtures feed the native ordered runner through one channel bridge.
use crate::engine::{Cancel, error::Error};
pub(crate) use crate::public::complete::recognize::ordered::{Input, Outcome, Row};
use std::sync::mpsc::{Receiver, Sender, channel};

pub(crate) struct Port<T, R, E>(Sender<Input<T, E>>, std::marker::PhantomData<fn() -> R>);
impl<T, R, E> Port<T, R, E> {
    pub(crate) fn send(&self, input: Input<T, E>) -> Result<(), ()> {
        self.0.send(input).map_err(|_| ())
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "existing fixtures retain their typed callbacks"
)]
pub(crate) fn run<T: Send + 'static, R: Send + 'static, E: Send + 'static>(
    jobs: usize,
    cancel: &Cancel,
    start_reader: impl FnOnce(Receiver<()>, Port<T, R, E>),
    answer: &(impl Fn(T) -> Result<Row<R>, E> + Sync),
    emit: impl FnMut(R) -> Result<bool, E>,
    stopped: &(impl Fn(Error) -> E + Sync),
    ended: fn() -> E,
) -> Result<Outcome<E>, E> {
    let (asked, requests) = channel();
    let (sent, received) = channel();
    let _keep_input_open = sent.clone();
    start_reader(requests, Port(sent, std::marker::PhantomData));
    let mut outstanding = false;
    crate::public::complete::recognize::ordered::run(
        jobs,
        cancel,
        None,
        || {
            if !outstanding {
                if asked.send(()).is_err() {
                    return Some(Input::Failed(ended()));
                }
                outstanding = true;
            }
            match received.try_recv() {
                Ok(input) => {
                    outstanding = false;
                    Some(input)
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Input::Failed(ended())),
            }
        },
        answer,
        emit,
        stopped,
        ended,
    )
}

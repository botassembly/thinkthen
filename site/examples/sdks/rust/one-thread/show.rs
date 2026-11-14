use std::sync::mpsc::TrySendError;
use thinkthen::{Decide, Engine, Record};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let call = engine.open(&Decide::new("Does the customer request a refund?")?)?;
    let sender = call.sender();
    let mut refund_checks = call.rows();
    let messages = ["Please refund my order.", "Where is my parcel?"];
    'feed: for message in messages {
        let mut record = Record::text(message);
        loop {
            match sender.try_send(record) {
                Ok(()) => break,
                Err(TrySendError::Full(back)) => {
                    record = back;
                    if let Some(refund_check) = refund_checks.next() {
                        println!("{}", refund_check?.into_answer()?);
                    }
                }
                Err(TrySendError::Disconnected(_)) => break 'feed,
            }
        }
    }
    sender.close();
    for refund_check in refund_checks {
        println!("{}", refund_check?.into_answer()?);
    }
    Ok(())
}

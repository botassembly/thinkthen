//! The held arm on the backend binary: counts, rounds and releases.

use std::thread;
use std::time::Duration;

use super::*;

#[test]
fn a_held_reply_waits_for_a_release_line() -> Tested {
    let mut backend = start()?;
    let answer = posting(backend.3, HELD);
    assert_eq!(ask(&mut backend, "wait 1")?, "wait 1");
    still_held(&answer);
    send(&mut backend, "release")?;
    answered(&answer)?;
    assert_eq!(finish(backend)?, ["1"]);
    Ok(())
}

/// Eight held requests are all counted while every reply is held, so the
/// backend serves them at once. The stress case below times the same claim
/// on the delay arm (ticket 0352).
#[test]
fn eight_held_requests_are_counted_while_every_reply_is_held() -> Tested {
    let mut backend = start()?;
    let answers: Vec<_> = (0..8).map(|_| posting(backend.3, HELD)).collect();
    assert_eq!(ask(&mut backend, "wait 8")?, "wait 8");
    if let Some(last) = answers.last() {
        still_held(last);
    }
    send(&mut backend, "release")?;
    for answer in &answers {
        answered(answer)?;
    }
    assert_eq!(finish(backend)?, ["8"]);
    Ok(())
}

#[test]
fn four_rounds_on_one_backend_each_let_go_only_the_reply_held_then() -> Tested {
    let mut backend = start()?;
    for round in 1..=4 {
        let answer = posting(backend.3, HELD);
        assert_eq!(
            ask(&mut backend, &format!("wait {round}"))?,
            format!("wait {round}")
        );
        still_held(&answer);
        send(&mut backend, "round")?;
        answered(&answer)?;
    }
    assert_eq!(last(backend)?, "4");
    Ok(())
}

#[test]
fn a_release_stays_open_for_later_held_replies() -> Tested {
    let mut backend = start()?;
    send(&mut backend, "release")?;
    answered(&posting(backend.3, HELD))?;
    assert_eq!(last(backend)?, "1");
    Ok(())
}

#[test]
fn a_wait_line_answers_once_the_count_reaches_it() -> Tested {
    let mut backend = start()?;
    send(&mut backend, "wait 1")?;
    thread::sleep(Duration::from_millis(200));
    let _held = posting(backend.3, HELD);
    assert_eq!(backend.2.recv_timeout(LINE)?, "wait 1");
    assert_eq!(last(backend)?, "1");
    Ok(())
}

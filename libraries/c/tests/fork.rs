//! Ian's item 6 for the C door: a fork after the first call answers its
//! own call.
//!
//! The parent warms the door with one call, forks, and the child makes
//! its own call through the JSON door on the null backend under a 10 s
//! alarm. The alarm makes a hang a failure — signal death, not a hang —
//! and the child's exit code carries whether it answered.
//!
//! Run through `./check.sh`, which sets `THINKTHEN_NULL=1`.

use std::ffi::CString;

use thinkthen::thinkthen_engine;

const ALARM_SECONDS: u32 = 10;
const REQUEST: &str = r#"{"decide": "Does the customer ask for a refund?", "evidence": "I want a refund for order 9"}"#;

unsafe fn call(engine: *const thinkthen_engine, text: &str) -> bool {
    unsafe {
        let request = CString::new(text).expect("no NUL");
        let pointer = thinkthen::thinkthen_call(engine, request.as_ptr());
        assert!(!pointer.is_null(), "the call answered a string");
        let reply = std::ffi::CStr::from_ptr(pointer)
            .to_string_lossy()
            .into_owned();
        thinkthen::thinkthen_free_string(pointer);
        let value: serde_json::Value = serde_json::from_str(&reply).expect("the reply is JSON");
        value["answer"] == serde_json::json!(true)
    }
}

#[test]
fn a_fork_after_the_first_call_answers_in_the_child() {
    unsafe {
        let engine = thinkthen::thinkthen_engine_new();

        // The parent's first call: whatever the engine holds is held
        // before the fork.
        assert!(call(engine, REQUEST), "the parent's first call answers");

        let pid = libc::fork();
        assert!(pid >= 0, "fork");
        if pid == 0 {
            // The child. A hang is killed by the alarm and read as a
            // signal death by the parent.
            libc::alarm(ALARM_SECONDS);
            let answered = call(engine, REQUEST);
            thinkthen::thinkthen_engine_free(engine);
            libc::_exit(if answered { 0 } else { 2 });
        }

        let mut status = 0;
        let waited = libc::waitpid(pid, &mut status, 0);
        assert_eq!(waited, pid, "the parent reaped the child");
        assert!(
            libc::WIFEXITED(status),
            "the child died to a signal — a hang under the {ALARM_SECONDS} s alarm"
        );
        assert_eq!(
            libc::WEXITSTATUS(status),
            0,
            "the child answered its own call through the door"
        );
    }
}

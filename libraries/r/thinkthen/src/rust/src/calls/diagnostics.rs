//! A caught panic on the R worker stays out of every output.

use std::io::Write as _;

use super::{Crossed, on_worker};
use thinkthen::CancelToken;

const CHILD: &str = "THINKTHEN_TEST_R_PANIC_CHILD";
const STRING: &str = "r-owned-string-payload-marker";
const DROP: &str = "r-owned-drop-payload-marker";

struct Exploding;

impl Drop for Exploding {
    fn drop(&mut self) {
        panic!("{DROP}");
    }
}

#[test]
fn a_caught_panic_stays_out_of_r_diagnostics() {
    if std::env::var_os(CHILD).is_some() {
        child();
    } else {
        let output = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .env_clear()
            .args([
                "--exact",
                "calls::diagnostics::a_caught_panic_stays_out_of_r_diagnostics",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .output()
            .expect("isolated R proof");
        assert!(output.status.success());
        for stream in [&output.stdout, &output.stderr] {
            let text = String::from_utf8_lossy(stream);
            assert!(!text.contains(STRING), "{text}");
            assert!(!text.contains(DROP), "{text}");
        }
        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "host-thread-marker\n"
        );
    }
}

fn child() {
    std::panic::set_hook(Box::new(|info| {
        let text = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str))
            .unwrap_or(if info.payload().is::<Exploding>() {
                DROP
            } else {
                "other panic"
            });
        let _ = std::io::stderr().write_all(format!("{text}\n").as_bytes());
    }));
    for failure in [
        on_worker(CancelToken::new(), &|| false, || -> Crossed<i32> {
            panic!("{STRING}")
        }),
        on_worker(CancelToken::new(), &|| false, || -> Crossed<i32> {
            std::panic::panic_any(Exploding)
        }),
    ] {
        assert_eq!(failure, Err(crate::defect("the call panicked")));
    }
    assert_eq!(on_worker(CancelToken::new(), &|| false, || Ok(7)), Ok(7));
    let _ = std::thread::spawn(|| panic!("host-thread-marker")).join();
}

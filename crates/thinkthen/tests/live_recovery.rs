//! Recovery consistency proofs for the guarded live supervisor.

#[cfg(test)]
#[path = "live_support/mod.rs"]
#[allow(
    dead_code,
    reason = "the shared fixture exposes helpers used by sibling suites"
)]
mod live_support;

use live_support::*;
use std::fs;
use std::process::{Command, Stdio};

#[test]
fn inconsistent_boot_ids_cannot_clear_a_matching_live_wrapper() {
    let (main, _linked) = temporary_repository("live-inconsistent-boots");
    let hooks = main.join("hooks");
    fs::create_dir(&hooks).unwrap();
    let mut crashed = live(&main)
        .current_dir(&main)
        .env("THINKTHEN_LIVE_TEST_DIR", &hooks)
        .env("THINKTHEN_LIVE_TEST_POINT", "after-replace")
        .args(["--max-tokens", "1", "quick.sh"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let _crashed_cleanup = ProcessCleanup::new(crashed.id());
    wait_for(&hooks.join("after-replace.ready"));
    signal(crashed.id(), "-KILL");
    let _status = crashed.wait().unwrap();

    let mut live_wrapper = Command::new("/bin/sleep").arg("30").spawn().unwrap();
    let _live_cleanup = ProcessCleanup::new(live_wrapper.id());
    let state = main.join(".git/thinkthen-live/state.json");
    let mutate = "import json,sys;p=sys.argv[1];n=int(sys.argv[2]);s=json.load(open(p));raw=open('/proc/%d/stat'%n).read();start=int(raw[raw.rfind(')')+2:].split()[19]);s['pending']['wrapper'].update(pid=n,start_ticks=start);s['pending']['gate']['boot_id']='00000000-0000-0000-0000-000000000000';open(p,'w').write(json.dumps(s,separators=(',',':'),sort_keys=True)+'\\n')";
    assert!(
        Command::new("/usr/bin/python3")
            .args(["-c", mutate])
            .arg(&state)
            .arg(live_wrapper.id().to_string())
            .status()
            .unwrap()
            .success()
    );
    let before = authority_state(&main);
    let refused = recover(&main);
    assert_eq!(refused.status.code(), Some(1), "{refused:?}");
    assert_eq!(authority_state(&main), before);
    live_wrapper.kill().unwrap();
    live_wrapper.wait().unwrap();
}

#![cfg(test)]
//! Typed response-store admission and compatible native usage snapshots.
#![allow(
    unsafe_code,
    reason = "exercise sized native configuration and copied usage layouts"
)]
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use serde_json::json;
use std::{mem::size_of, ptr};
use thinkthen_ffi::*;
fn view(value: &str) -> TextView {
    TextView {
        data: value.as_ptr(),
        len: value.len(),
    }
}
#[test]
fn typed_store_configuration_and_v2_metrics_preserve_the_original_usage_layout() {
    let server = loopback::Loopback::start(|_| {
        loopback::Reply::json(200, &json!({"answers":{"q1":{"type":"noul","noul":0.9}}}))
    })
    .unwrap();
    let path = std::env::temp_dir().join(format!(
        "thinkthen-native-responses-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let path = path.to_str().unwrap();
    // SAFETY: every borrowed configuration input outlives the setter; each owned
    // handle and sized output stays live until its matching cleanup.
    unsafe {
        let mut question = ptr::null_mut();
        assert_eq!(
            thinkthen_question_new(1, b"Ready?".as_ptr(), 6, &mut question, ptr::null_mut()),
            0
        );
        for mode in [THINKTHEN_RESPONSE_RECORD, THINKTHEN_RESPONSE_REPLAY] {
            let mut settings = ptr::null_mut();
            assert_eq!(thinkthen_settings_new(&mut settings, ptr::null_mut()), 0);
            assert_eq!(
                thinkthen_settings_set_backend(
                    settings,
                    thinkthen::BackendName::SystemOne.abi(),
                    ptr::null_mut()
                ),
                0
            );
            assert_eq!(
                thinkthen_settings_set_url(
                    settings,
                    server.base().as_ptr(),
                    server.base().len(),
                    ptr::null_mut()
                ),
                0
            );
            if mode == THINKTHEN_RESPONSE_RECORD {
                assert_eq!(
                    thinkthen_settings_set_api_key(
                        settings,
                        loopback::KEY.as_ptr(),
                        loopback::KEY.len(),
                        ptr::null_mut()
                    ),
                    0
                );
            }
            let mut config = HostResponseStoreConfig {
                struct_size: size_of::<HostResponseStoreConfig>() as u32,
                path: view(path),
                mode,
                namespace_name: view("native"),
                model_generation: view("revision-1"),
                max_bytes: ptr::null(),
                max_entries: ptr::null(),
                max_age: ptr::null(),
                busy_timeout_ms: ptr::null(),
            };
            config.struct_size -= 1;
            assert_eq!(
                thinkthen_settings_set_response_store(settings, &config, ptr::null_mut()),
                -thinkthen::Code::UsageAbi.abi()
            );
            config.struct_size += 1;
            assert_eq!(
                thinkthen_settings_set_response_store(settings, &config, ptr::null_mut()),
                0
            );
            if mode == THINKTHEN_RESPONSE_REPLAY {
                assert_eq!(
                    thinkthen_settings_set_response_store(settings, ptr::null(), ptr::null_mut()),
                    0
                );
                // Stub clients reject response stores. Successful construction
                // therefore proves clearing the option without consulting keys.
                assert_eq!(
                    thinkthen_settings_set_backend(
                        settings,
                        thinkthen::BackendName::Stub.abi(),
                        ptr::null_mut()
                    ),
                    0
                );
                let mut disabled = ptr::null_mut();
                assert_eq!(
                    thinkthen_client_open(settings, &mut disabled, ptr::null_mut()),
                    0
                );
                thinkthen_client_free(disabled);
                assert_eq!(
                    thinkthen_settings_set_backend(
                        settings,
                        thinkthen::BackendName::SystemOne.abi(),
                        ptr::null_mut()
                    ),
                    0
                );
                assert_eq!(
                    thinkthen_settings_set_response_store(settings, &config, ptr::null_mut()),
                    0
                );
            }
            let zero = 0;
            config.max_entries = &zero;
            assert_eq!(
                thinkthen_settings_set_response_store(settings, &config, ptr::null_mut()),
                -thinkthen::Code::UsageSettings.abi()
            );
            let mut client = ptr::null_mut();
            assert_eq!(
                thinkthen_client_open(settings, &mut client, ptr::null_mut()),
                0
            );
            thinkthen_settings_free(settings);
            let mut row: HostRow = std::mem::zeroed();
            row.struct_size = size_of::<HostRow>() as u32;
            assert_eq!(
                thinkthen_run_one(
                    client,
                    question,
                    b"evidence".as_ptr(),
                    8,
                    0,
                    &mut row,
                    ptr::null_mut()
                ),
                0
            );
            thinkthen_row_free(&mut row);
            let mut original = HostUsage {
                struct_size: size_of::<HostUsage>() as u32,
                ..HostUsage::default()
            };
            assert_eq!(
                thinkthen_client_usage_values(client, &mut original, ptr::null_mut()),
                0
            );
            let mut full = HostUsageV2 {
                struct_size: size_of::<HostUsageV2>() as u32,
                ..HostUsageV2::default()
            };
            for getter in [
                thinkthen_client_usage_v2_values,
                thinkthen_client_usage_v2_values_settled,
            ] {
                assert_eq!(
                    getter(client, ptr::null_mut(), ptr::null_mut()),
                    -thinkthen::Code::UsageAbi.abi()
                );
                full.struct_size -= 1;
                full.cache_hits = 99;
                assert_eq!(
                    getter(client, &mut full, ptr::null_mut()),
                    -thinkthen::Code::UsageAbi.abi()
                );
                assert_eq!(full.cache_hits, 99);
                full.struct_size += 1;
            }
            assert_eq!(
                thinkthen_client_usage_v2_values_settled(client, &mut full, ptr::null_mut()),
                0
            );
            assert_eq!(full.usage.struct_size, size_of::<HostUsage>() as u32);
            assert_eq!(full.usage.attempts, original.attempts);
            assert_eq!(
                full.replay_hits,
                u64::from(mode == THINKTHEN_RESPONSE_REPLAY)
            );
            assert_eq!(full.cache_hits, 0);
            thinkthen_client_free(client);
            assert_eq!(
                full.replay_hits,
                u64::from(mode == THINKTHEN_RESPONSE_REPLAY)
            );
        }
        thinkthen_question_free(question);
    }
    assert_eq!(server.count(), 1);
}
#[test]
fn sql_refuses_explicit_storage_before_environment_resolution() {
    for value in ["null", "{}"] {
        let error = thinkthen::native::sql_engine_unrestricted(
            &format!("{{\"response_store\":{value}}}"),
            |_| panic!("forbidden SQL settings resolved environment"),
        )
        .unwrap_err();
        assert_eq!(error.code(), thinkthen::Code::UsageSettings);
    }
}

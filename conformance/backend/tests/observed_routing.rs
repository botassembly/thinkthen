//! Header-free observations reject successful replies at the wrong posting path.

use std::collections::BTreeMap;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use conformance_backend::{Backend, Paths};
use serde_json::{Value, json};

const BODY: &str = r#"{"state":"refund","model":"pplx-decider-v1-27b","questions":{"q1":{"type":"noul","description":"attention?"}}}"#;

fn send(
    backend: &Backend,
    method: &str,
    path: &str,
    authorization: &str,
) -> Result<String, Box<dyn Error>> {
    let address = backend.origin().strip_prefix("http://").ok_or("http")?;
    let mut socket = TcpStream::connect(address)?;
    write!(
        socket,
        "{method} {path} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n{authorization}Content-Length: {}\r\n\r\n{BODY}",
        BODY.len()
    )?;
    socket.shutdown(Shutdown::Write)?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    let mut response = String::new();
    socket.read_to_string(&mut response)?;
    Ok(response)
}

fn compare(actual: Paths, expected: Paths) -> Result<(), &'static str> {
    if actual == expected && !actual.overflow {
        Ok(())
    } else {
        Err("posting_path_mismatch")
    }
}

#[test]
fn observations_count_exact_targets_without_retaining_headers() -> Result<(), Box<dyn Error>> {
    let backend = Backend::with_markers(BTreeMap::from([(
        "provider".into(),
        "fake-provider".into(),
    )]))?;
    let other = Backend::start()?;
    assert_eq!(backend.paths(), Paths::default());
    for target in [
        "/generic/v1/systemone",
        "/generic/v1/decisions",
        "/generic/v1/judgements/v2/decide",
    ] {
        assert!(
            send(
                &backend,
                "POST",
                target,
                "Authorization: Bearer fake-provider\r\n"
            )?
            .starts_with("HTTP/1.1 200")
        );
    }
    for target in [
        "/generic/v1/systemone/extra",
        "/generic/v1/systemone?x=1",
        "/generic/v1/%73ystemone",
        "/generic/v1/Systemone",
    ] {
        send(
            &backend,
            "POST",
            target,
            "Authorization: Bearer unknown\r\n",
        )?;
    }
    send(&backend, "GET", "/generic/v1/systemone", "")?;
    send(
        &backend,
        "POST",
        "/generic/v1/systemone",
        "Authorization: Bearer fake-provider\r\nAuthorization: Bearer fake-provider\r\n",
    )?;
    let expected = Paths {
        generic_systemone: 2,
        generic_decisions: 1,
        generic_custom: 1,
        other: 4,
        non_post: 1,
        ..Default::default()
    };
    assert_eq!(compare(backend.paths(), expected.clone()), Ok(()));
    assert_eq!(backend.paths(), expected);
    assert_eq!(backend.count(), 9);
    assert_eq!(other.paths(), Paths::default());
    let bearer: Value = serde_json::from_str(&backend.bearers())?;
    assert_eq!(
        bearer,
        json!({"markers":{"provider":3},"absent":1,"unknown":5,"overflow":false})
    );
    assert!(!format!("{backend:?}").contains("fake-provider"));
    assert_eq!(
        serde_json::from_str::<Value>(&backend.capture())?,
        json!({"bodies":[]})
    );
    Ok(())
}

#[test]
fn a_successful_perplexity_body_at_systemone_fails_the_path_expectation()
-> Result<(), Box<dyn Error>> {
    let backend = Backend::with_markers(BTreeMap::from([(
        "provider".into(),
        "fake-provider".into(),
    )]))?;
    let response = send(
        &backend,
        "POST",
        "/arm/full/capture/v1/systemone",
        "Authorization: Bearer fake-provider\r\n",
    )?;
    assert!(response.starts_with("HTTP/1.1 200"));
    assert_eq!(backend.count(), 1);
    let capture: Value = serde_json::from_str(&backend.capture())?;
    assert_eq!(capture, json!({"bodies":[BODY]}));
    let bearer: Value = serde_json::from_str(&backend.bearers())?;
    assert_eq!(
        bearer,
        json!({"markers":{"provider":1},"absent":0,"unknown":0,"overflow":false})
    );
    assert_eq!(
        compare(
            backend.paths(),
            Paths {
                capture_decisions: 1,
                ..Default::default()
            }
        ),
        Err("posting_path_mismatch")
    );
    assert_eq!(
        compare(
            Paths {
                overflow: true,
                ..Default::default()
            },
            Paths {
                overflow: true,
                ..Default::default()
            }
        ),
        Err("posting_path_mismatch")
    );
    Ok(())
}

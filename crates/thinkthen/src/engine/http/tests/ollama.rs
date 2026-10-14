//! The refusal advice uses the posting address while the resolver owns its socket.

use conformance_backend::{Canned, Listener};
use std::net::SocketAddr;
use std::time::Duration;
use ureq::unversioned::resolver::{ResolvedSocketAddrs, Resolver};
use ureq::unversioned::transport::NextTimeout;

#[derive(Debug)]
struct OwnedAddress(SocketAddr);

impl Resolver for OwnedAddress {
    fn resolve(
        &self,
        uri: &ureq::http::Uri,
        _config: &ureq::config::Config,
        _timeout: NextTimeout,
    ) -> Result<ResolvedSocketAddrs, ureq::Error> {
        assert_eq!(uri.host(), Some("127.0.0.1"));
        assert_eq!(uri.port_u16(), Some(11434));
        let mut addresses = self.empty();
        addresses.push(self.0);
        Ok(addresses)
    }
}

#[test]
fn an_ollama_shaped_address_suggests_the_named_backend_without_changing_the_request() {
    use crate::core::{Backend, Evidence, ModelName, Plan, Question, QuestionText};
    use crate::engine::http::{Client, Exchange, Key};

    let message = "Description objects are unsupported; use text descriptions.";
    let body = serde_json::json!({"error":{"message":message}}).to_string();
    let socket = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = socket.local_addr().unwrap();
    let listener = Listener::serving_on(socket, vec![Canned::status(400, &body)]).unwrap();
    let backend = Backend::resolve(Some("http://127.0.0.1:11434/v1"), None, "local-1").unwrap();
    let plan = Plan::authored(
        Evidence::new("private evidence marker").unwrap(),
        ModelName::new("local-1").unwrap(),
        vec![Question::Decide {
            text: QuestionText::new("asks for a refund").unwrap(),
            yes: None,
            no: None,
        }],
    )
    .unwrap();
    let request_body = backend.api_type().encode(&plan).unwrap();
    let mut client = Client::new(
        Duration::from_secs(5),
        false,
        &crate::engine::limits::process().widths,
    );
    client.agent = ureq::Agent::with_parts(
        ureq::Agent::config_builder()
            .http_status_as_error(false)
            .proxy(None)
            .build(),
        super::super::send::OpeningConnector::default(),
        OwnedAddress(address),
    );
    let key = Key::of("sk-test-value");
    let exchange = Exchange {
        url: backend.url().as_str(),
        body: &request_body,
        key: &key,
        max_retries: 0,
        retry_wait: Duration::from_millis(1),
    };
    let error = client
        .post_observed(&exchange, &crate::engine::Cancel::default(), || ())
        .err()
        .unwrap();
    let (code, sentence) = crate::cli::failure::sentence(&error.into());
    assert_eq!(code, 4);
    assert_eq!(
        sentence,
        format!(
            "the backend answered with status 400: the backend refused the request; this address looks like Ollama; try --backend ollama; provider message: {message}"
        )
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let request: serde_json::Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert!(
        request["questions"].is_object(),
        "plain URL retains its request shape"
    );
}

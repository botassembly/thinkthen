//! Print the recording digest one request is filed under (0053).
//!
//! The production rule: the adapter name, the resolved URL, and the exact
//! request bytes through `recording::Exchange::digest`. Run with a clean
//! environment so the built-in default URL applies, exactly as the
//! conformance file's canonical `backend_url` does:
//!
//! ```text
//! env -u ENGINE_BASE_URL cargo run --release --example exchange_digest -- \
//!   '{"decide":"Does the writer ask for a refund?","threshold":0.9}' \
//!   'I want my money back'
//! ```

use thinkthen_contract::Question;
use thinkthen_standin::request_digest;

fn main() {
    let mut args = std::env::args().skip(1);
    let question = args.next().expect("the question JSON");
    let evidence = args.next().expect("the evidence text");
    let question = Question::from_json(&question).expect("the question builds");
    let digest = request_digest(&question, None, &evidence).expect("the request digests");
    println!("{digest}");
}

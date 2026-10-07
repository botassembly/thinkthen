//! Bounded body-only capture for opted-in loopback fixture requests.
/// Only three requests from an opted-in relation case or full-answer arm are retained.
const CAPTURE_BODIES: usize = 3;
pub(crate) const CAPTURE_BYTES: usize = 96_000;

#[derive(Debug, Default)]
pub(crate) struct Capture {
    bodies: Vec<Vec<u8>>,
    overflow: bool,
}

impl Capture {
    pub(crate) fn push(&mut self, body: &[u8], limit: usize) {
        if self.bodies.len() == CAPTURE_BODIES || body.len() > limit {
            self.overflow = true;
        } else {
            self.bodies.push(body.to_vec());
        }
    }

    pub(crate) fn json(&self) -> String {
        if self.overflow {
            return serde_json::json!({"error": "capture overflow"}).to_string();
        }
        let bodies = self
            .bodies
            .iter()
            .map(|body| std::str::from_utf8(body))
            .collect::<Result<Vec<_>, _>>();
        match bodies {
            Ok(bodies) => serde_json::json!({"bodies": bodies}).to_string(),
            Err(_) => serde_json::json!({"error": "capture is not UTF-8"}).to_string(),
        }
    }
}

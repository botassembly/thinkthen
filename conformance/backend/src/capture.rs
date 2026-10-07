//! Bounded body-only capture for opted-in loopback fixture requests.
/// At most sixteen requests from an opted-in relation case or full-answer arm are retained.
const CAPTURE_BODIES: usize = 16;
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

#[cfg(test)]
mod tests {
    use super::{CAPTURE_BYTES, Capture};

    #[test]
    fn sixteen_bodies_are_retained_and_the_seventeenth_marks_overflow() {
        let mut capture = Capture::default();
        for _ in 0..16 {
            capture.push(b"{}", CAPTURE_BYTES);
        }
        let expected = serde_json::json!({"bodies":vec!["{}"; 16]}).to_string();
        assert_eq!(capture.json(), expected);
        capture.push(b"{}", CAPTURE_BYTES);
        assert_eq!(capture.json(), r#"{"error":"capture overflow"}"#);
    }

    #[test]
    fn body_byte_bound_still_accepts_its_endpoint_and_refuses_one_more() {
        let mut exact = Capture::default();
        exact.push(&vec![b'x'; CAPTURE_BYTES], CAPTURE_BYTES);
        assert_eq!(
            exact.json(),
            serde_json::json!({"bodies":["x".repeat(CAPTURE_BYTES)]}).to_string()
        );
        let mut excess = Capture::default();
        excess.push(&vec![b'x'; CAPTURE_BYTES + 1], CAPTURE_BYTES);
        assert_eq!(excess.json(), r#"{"error":"capture overflow"}"#);
    }
}

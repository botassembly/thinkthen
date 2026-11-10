//! Serve validated ask answers before assigning transport slots to misses.
use super::{Engine, Split};
use crate::core::{AskAnswer, Request, RequestAnswers};
use crate::{Code, Error};

impl Engine {
    pub(super) fn send(
        &self,
        request: Request<'_>,
        url: &str,
        split: Split,
    ) -> Result<Vec<RequestAnswers>, Error> {
        let Some(store) = &self.store else {
            return self.send_live(request, url, split);
        };
        if self.cancel.is_cancelled() {
            return Err(crate::retry::cancelled());
        }
        let keys = self.keys(&request, url, store);
        let held = store.lookup(&keys, &self.cancel)?;
        let mut hit_positions = Vec::new();
        let mut hit_answers = Vec::new();
        let mut misses = Vec::new();
        for (index, ((ask, _), held)) in request.materialized().zip(held).enumerate() {
            if let Some(bytes) = held {
                hit_answers.push(AskAnswer::restore(ask, &bytes)?);
                hit_positions.push(index);
            } else {
                misses.push(index);
            }
        }
        self.account.reused(hit_answers.len(), store.replay())?;
        let mut answers = Vec::new();
        if !hit_positions.is_empty() {
            answers.push(request.select(&hit_positions).saved(hit_answers));
        }
        if !misses.is_empty() {
            answers.extend(self.send_live(request.select(&misses), url, split)?);
        }
        Ok(answers)
    }

    fn keys(
        &self,
        request: &Request<'_>,
        url: &str,
        store: &crate::response_store::Store,
    ) -> Vec<[u8; 32]> {
        request
            .materialized()
            .map(|(ask, question)| {
                crate::response_store::identity(
                    store.config(),
                    self.backend.name(),
                    url,
                    self.backend.model(),
                    request.wire(),
                    ask.variant(),
                    request.state(),
                    question,
                )
            })
            .collect()
    }

    pub(super) fn retain(
        &self,
        request: Request<'_>,
        url: &str,
        body: Vec<u8>,
    ) -> Result<Vec<RequestAnswers>, Error> {
        let Some(store) = &self.store else {
            return Ok(vec![request.answered(Ok(body))]);
        };
        // Malformed or partial backend answers preserve existing row failures and
        // never become reusable, including any otherwise valid sibling answer.
        let Ok(answers) = request.validated(&body) else {
            return Ok(vec![request.answered(Ok(body))]);
        };
        let request_body = request.body();
        let metadata = crate::response_store::metadata(&self.backend, store.config())?;
        // Responses are untrusted: a provider that echoes our credential must not
        // turn exact-body provenance into credential retention.
        if let Some(key) = self.backend.remote().and_then(|remote| remote.key.as_ref()) {
            for bytes in [&request_body, &body, &metadata] {
                if crate::response_store::contains_credential(bytes, key.as_str())? {
                    return Err(crate::response_store::error(Code::LocalResponseStore));
                }
            }
        }
        let persisted = self
            .keys(&request, url, store)
            .into_iter()
            .zip(&answers)
            .map(|(key, answer)| answer.stored().map(|bytes| (key, bytes)))
            .collect::<Result<Vec<_>, _>>()?;
        store.append(&metadata, &request_body, &body, &persisted, &self.cancel)?;
        Ok(vec![request.saved(answers)])
    }
}

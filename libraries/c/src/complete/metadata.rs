//! Actual facts, transport attempts and aligned response provenance.
use crate::current::Storage;
use crate::failures::Failure;
use crate::ffi::carriers::{
    AttemptV1, AttemptsV1, BatchV1, BatchWarningV1, FactsV1, MetaV1, ObservationIdentitiesV1,
    ObservationIdentityDataV1, ObservationIdentityV1, OptionalAttemptsV1, OptionalBatchV1,
    OptionalBatchWarningV1, OptionalDiscriminatorV1, OptionalDoubleV1, OptionalFactsV1,
    OptionalProfileWarningV1, OptionalSizeV1, OptionalStoppedV1, OptionalU16V1, OptionalU64V1,
    OptionalUsageV1, ProfileWarningV1, QuestionSourceV1, QuestionSourcesV1, StoppedV1, UsageV1,
};
use crate::ffi::values as abi;
use thinkthen::{
    AttemptObservation, BatchSetting, Facts, Observation, Origin, QuestionSource, ResultMetadata,
    StopCause, Stopped,
};
pub(super) fn size(value: Option<usize>) -> OptionalSizeV1 {
    OptionalSizeV1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0),
    }
}
pub(super) fn double(value: Option<f64>) -> OptionalDoubleV1 {
    OptionalDoubleV1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0.0),
    }
}
fn u64_view(value: Option<u64>) -> OptionalU64V1 {
    OptionalU64V1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0),
    }
}
fn u16_view(value: Option<u16>) -> OptionalU16V1 {
    OptionalU16V1 {
        present: i32::from(value.is_some()),
        value: value.unwrap_or(0),
    }
}
pub(super) const fn origin(value: Origin) -> u32 {
    match value {
        Origin::Live => abi::THINKTHEN_ORIGIN_LIVE_V1,
        Origin::Cache => abi::THINKTHEN_ORIGIN_CACHE_V1,
        Origin::Replay => abi::THINKTHEN_ORIGIN_REPLAY_V1,
        Origin::Proxy => abi::THINKTHEN_ORIGIN_PROXY_V1,
        Origin::Memory => abi::THINKTHEN_ORIGIN_MEMORY_V1,
    }
}
fn batch(value: BatchSetting) -> BatchV1 {
    match value {
        BatchSetting::Max => BatchV1 {
            kind: abi::THINKTHEN_BATCH_MAX_V1,
            records: 0,
        },
        BatchSetting::Records(count) => BatchV1 {
            kind: abi::THINKTHEN_BATCH_RECORDS_V1,
            records: count.get(),
        },
    }
}
impl Storage {
    pub(super) fn attempts(&mut self, values: Option<&[AttemptObservation]>) -> OptionalAttemptsV1 {
        let Some(values) = values else {
            return OptionalAttemptsV1::default();
        };
        let values = values
            .iter()
            .map(|a| AttemptV1 {
                ordinal: a.ordinal(),
                request_sha256: self.string(a.request_sha256()),
                wall_ms: a.wall_ms(),
                outcome: match a.outcome() {
                    thinkthen::AttemptOutcome::Ok => abi::THINKTHEN_ATTEMPT_OK_V1,
                    thinkthen::AttemptOutcome::Status => abi::THINKTHEN_ATTEMPT_STATUS_V1,
                    thinkthen::AttemptOutcome::Transport => abi::THINKTHEN_ATTEMPT_TRANSPORT_V1,
                },
                sdk_request_id: self.string(a.sdk_request_id().as_str()),
                status: u16_view(a.status()),
                server_ms: u64_view(a.server_ms()),
                request_id: self.optional_string(a.request_id()),
            })
            .collect();
        let (data, len) = self.array(values);
        OptionalAttemptsV1 {
            present: 1,
            value: AttemptsV1 { data, len },
        }
    }
    pub(super) fn sources(&mut self, values: &[QuestionSource]) -> QuestionSourcesV1 {
        let values = values
            .iter()
            .map(|v| QuestionSourceV1 {
                origin: origin(v.origin()),
                answered_by: self.string(v.answered_by()),
            })
            .collect();
        let (data, len) = self.array(values);
        QuestionSourcesV1 { data, len }
    }
    pub(super) fn identities(&mut self, values: &[Observation]) -> ObservationIdentitiesV1 {
        let values = values
            .iter()
            .map(|v| {
                let mut data = ObservationIdentityDataV1::default();
                let kind = match v {
                    Observation::Answered { observation_id } => {
                        data.observation_id = self.string(observation_id.as_str());
                        abi::THINKTHEN_ID_OBSERVATION_V1
                    }
                    Observation::Failed { failure_id } => {
                        data.failure_id = self.string(failure_id.as_str());
                        abi::THINKTHEN_ID_FAILURE_V1
                    }
                };
                ObservationIdentityV1 { kind, data }
            })
            .collect();
        let (data, len) = self.array(values);
        ObservationIdentitiesV1 { data, len }
    }
    pub(super) fn usage(&mut self, value: Option<thinkthen::ReportedUsage>) -> OptionalUsageV1 {
        // This canonical v1 layout represents complete dimensions only; partial
        // dimensions remain unknown rather than synthesizing a zero count.
        match value.and_then(|v| Some((v.input_tokens()?, v.output_tokens()?))) {
            Some((input_tokens, output_tokens)) => OptionalUsageV1 {
                present: 1,
                value: UsageV1 {
                    input_tokens,
                    output_tokens,
                },
            },
            None => OptionalUsageV1::default(),
        }
    }
    pub(super) fn metadata(&mut self, value: ResultMetadata<'_>) -> MetaV1 {
        let identity = value.identity();
        MetaV1 {
            tool: self.string(value.tool()),
            question_sha256: self.optional_string(value.question_sha256()),
            questions_sha256: self.optional_string(value.questions_sha256()),
            url: self.string(value.url()),
            model: self.string(value.model()),
            usage: self.usage(value.usage()),
            requests_sent: value.requests_sent(),
            cached: i32::from(value.cached()),
            requests: self.strings(value.requests()),
            failed_questions: value.failed_questions(),
            profile_warning: value
                .profile_warning()
                .map(|w| OptionalProfileWarningV1 {
                    present: 1,
                    value: ProfileWarningV1 {
                        tuned_for: self.string(w.tuned_for()),
                        running: self.string(w.running()),
                    },
                })
                .unwrap_or_default(),
            batch_setting: value
                .batch_setting()
                .map(|b| OptionalBatchV1 {
                    present: 1,
                    value: batch(b),
                })
                .unwrap_or_default(),
            batch_warning: value
                .batch_warning()
                .and_then(|w| {
                    Some(BatchWarningV1 {
                        tuned_for: batch(w.tuned_for()?),
                        running: batch(w.running()?),
                    })
                })
                .map(|value| OptionalBatchWarningV1 { present: 1, value })
                .unwrap_or_default(),
            context_sha256: self.optional_string(value.context_sha256()),
            attempts: self.attempts(value.attempts()),
            origin: OptionalDiscriminatorV1 {
                present: i32::from(identity.origin().is_some()),
                value: identity.origin().map_or(0, origin),
            },
            question_sources: self.sources(identity.question_sources()),
            observations: self.identities(identity.observations()),
            answered_by: self.optional_string(identity.answered_by()),
        }
    }
}
pub(super) fn facts(s: &mut Storage, f: &Facts) -> Result<OptionalFactsV1, Failure> {
    let complete = f
        .complete()
        .ok_or_else(|| Failure::defect("one complete invocation lost its native call identity"))?;
    Ok(OptionalFactsV1 {
        present: 1,
        value: FactsV1 {
            call_id: s.string(complete.call_id().as_str()),
            cache_answers: f.cache_answers(),
            estimated_cost_usd: s.optional_string(f.estimated_cost_usd()),
            input_tokens: u64_view(f.input_tokens()),
            model: s.optional_string(f.model()),
            output_tokens: u64_view(f.output_tokens()),
            records: f.records(),
            requests_sent: f.requests_sent(),
            seconds: f.seconds(),
            command_ms: OptionalU64V1::default(),
        },
    })
}
pub(super) fn stopped(value: Stopped) -> OptionalStoppedV1 {
    OptionalStoppedV1 {
        present: 1,
        value: StoppedV1 {
            at: size(value.at()),
            cause: match value.cause() {
                StopCause::Usage => abi::THINKTHEN_STOP_USAGE_V1,
                StopCause::Local => abi::THINKTHEN_STOP_LOCAL_V1,
                StopCause::NoKey => abi::THINKTHEN_STOP_NO_KEY_V1,
                StopCause::Transport => abi::THINKTHEN_STOP_TRANSPORT_V1,
                StopCause::Status => abi::THINKTHEN_STOP_STATUS_V1,
                StopCause::TooLarge => abi::THINKTHEN_STOP_TOO_LARGE_V1,
                StopCause::Reply => abi::THINKTHEN_STOP_REPLY_V1,
                StopCause::Backend => abi::THINKTHEN_STOP_BACKEND_V1,
                StopCause::Cancelled => abi::THINKTHEN_STOP_CANCELLED_V1,
                StopCause::Defect => abi::THINKTHEN_STOP_DEFECT_V1,
                StopCause::Deadline => abi::THINKTHEN_STOP_DEADLINE_V1,
            },
            status: u16_view(value.status()),
            retryable: i32::from(value.retryable()),
        },
    }
}

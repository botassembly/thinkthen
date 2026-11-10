//! Allocation-free usage projections for native callers.
#![allow(
    unsafe_code,
    reason = "borrow live native handles and a sized caller-owned output"
)]
use crate::exports::{THINKTHEN_OK, operation};
use crate::{ClientHandle, HostError};
use thinkthen::{Code, TokenUsage, Usage, native};

/// Observed tokens; an unknown reported total denotes arithmetic overflow.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct HostTokenUsage {
    /// Known token subtotal, meaningful only when reported_known is true.
    pub reported: u64,
    /// Finished attempts lacking a reported count for this dimension.
    pub unknown_attempts: u64,
    /// Whether the subtotal fits in reported; missing attempts are counted separately.
    pub reported_known: bool,
}
impl From<TokenUsage> for HostTokenUsage {
    fn from(tokens: TokenUsage) -> Self {
        Self {
            reported: tokens.reported.unwrap_or(0),
            unknown_attempts: tokens.unknown_attempts,
            reported_known: tokens.reported.is_some(),
        }
    }
}
/// Independently copied cumulative usage. Initialize struct_size before a getter.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct HostUsage {
    /// Set to sizeof(thinkthen_usage).
    pub struct_size: u32,
    /// Actual send attempts.
    pub attempts: u64,
    /// Fully received bounded response bodies.
    pub responses: u64,
    /// Attempts retrying an HTTP status.
    pub status_retries: u64,
    /// Attempts retrying a connection.
    pub connection_retries: u64,
    /// Encoded request bytes including retries.
    pub request_body_bytes: u64,
    /// Engine calls that have not settled, including paused input.
    pub active_calls: u64,
    /// Transport attempts that have not finished.
    pub active_attempts: u64,
    /// Observed input tokens.
    pub input_tokens: HostTokenUsage,
    /// Observed output tokens.
    pub output_tokens: HostTokenUsage,
}
impl From<Usage> for HostUsage {
    fn from(usage: Usage) -> Self {
        Self {
            struct_size: std::mem::size_of::<Self>() as u32,
            attempts: usage.attempts,
            responses: usage.responses,
            status_retries: usage.status_retries,
            connection_retries: usage.connection_retries,
            request_body_bytes: usage.request_body_bytes,
            active_calls: usage.active_calls,
            active_attempts: usage.active_attempts,
            input_tokens: usage.input_tokens.into(),
            output_tokens: usage.output_tokens.into(),
        }
    }
}
/// Copy client usage into a sized typed output, with no allocation or waiting.
/// # Safety
/// client is live and cannot be freed concurrently. out is writable, initialized
/// with struct_size = sizeof(thinkthen_usage). error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_usage_values(
    client: *const ClientHandle,
    out: *mut HostUsage,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends its live client and sized writable output. */
    unsafe {
        operation(error, || {
            let client = client
                .as_ref()
                .ok_or_else(|| native::argument_error(Code::UsageAbi))?;
            if out.is_null() || (*out).struct_size < std::mem::size_of::<HostUsage>() as u32 {
                return Err(native::argument_error(Code::UsageAbi));
            }
            out.write(client.0.usage().into());
            Ok(THINKTHEN_OK)
        })
    }
}
/// Wait for owned calls to settle and copy usage into a sized typed output.
/// First stop admitting calls and cancel/free owned unfinished calls.
/// # Safety
/// client is live; no thread opens new calls or frees it during this function.
/// out is writable with struct_size = sizeof(thinkthen_usage); error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_usage_values_settled(
    client: *const ClientHandle,
    out: *mut HostUsage,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends its live client and sized writable output. */
    unsafe {
        operation(error, || {
            let client = client
                .as_ref()
                .ok_or_else(|| native::argument_error(Code::UsageAbi))?;
            if out.is_null() || (*out).struct_size < std::mem::size_of::<HostUsage>() as u32 {
                return Err(native::argument_error(Code::UsageAbi));
            }
            out.write(client.0.settled_usage().into());
            Ok(THINKTHEN_OK)
        })
    }
}

/// Full copied usage including local hit metrics. The embedded usage layout stays unchanged.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct HostUsageV2 {
    /// Initialize to sizeof(thinkthen_usage_v2).
    pub struct_size: u32,
    /// Original counters; the getter also fills this nested struct_size.
    pub usage: HostUsage,
    /// Eligible cached asks reused, not rows.
    pub cache_hits: u64,
    /// Recorded asks replayed, not rows.
    pub replay_hits: u64,
}
impl From<Usage> for HostUsageV2 {
    fn from(usage: Usage) -> Self {
        Self {
            struct_size: std::mem::size_of::<Self>() as u32,
            usage: usage.into(),
            cache_hits: usage.cache_hits,
            replay_hits: usage.replay_hits,
        }
    }
}
unsafe fn output_v2(
    client: *const ClientHandle,
    out: *mut HostUsageV2,
    error: *mut HostError,
    snapshot: fn(&thinkthen::Engine) -> Usage,
) -> i32 {
    /* SAFETY: wrappers lend their documented live handles and sized output. */
    unsafe {
        operation(error, || {
            let client = client
                .as_ref()
                .ok_or_else(|| native::argument_error(Code::UsageAbi))?;
            if out.is_null() || (*out).struct_size < std::mem::size_of::<HostUsageV2>() as u32 {
                return Err(native::argument_error(Code::UsageAbi));
            }
            out.write(snapshot(&client.0).into());
            Ok(THINKTHEN_OK)
        })
    }
}
/// Copy all usage and store metrics from one atomic account snapshot without waiting.
/// # Safety
/// client is live; out is writable with its top-level struct_size initialized;
/// error is NULL or initialized. No thread frees client until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_usage_v2_values(
    client: *const ClientHandle,
    out: *mut HostUsageV2,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends its live client and sized writable output. */
    unsafe { output_v2(client, out, error, thinkthen::Engine::usage) }
}
/// Wait for owned calls to settle, then copy all usage and store metrics atomically.
/// # Safety
/// Stop admission and cancel/free unfinished owned calls first. client remains live,
/// out is writable with top-level struct_size initialized; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_usage_v2_values_settled(
    client: *const ClientHandle,
    out: *mut HostUsageV2,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends its live client and sized writable output. */
    unsafe { output_v2(client, out, error, thinkthen::Engine::settled_usage) }
}

//! Bounded newline framing and typed JSON-RPC envelopes at the transport edge.

use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

pub(super) const VERSION: &str = "2025-11-25";
pub(super) const MAX_MESSAGE: usize = 16 * 1024 * 1024;

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(untagged)]
pub(super) enum Id {
    Text(String),
    Signed(i64),
    Unsigned(u64),
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Id(<withheld>)")
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Message {
    pub(super) jsonrpc: String,
    #[serde(default)]
    pub(super) id: Option<Id>,
    pub(super) method: String,
    #[serde(default)]
    pub(super) params: Option<Box<RawValue>>,
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("request", &self.id.is_some())
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub(super) struct Fault {
    pub(super) code: i32,
    pub(super) message: &'static str,
}

impl Fault {
    pub(super) const PARSE: Self = Self {
        code: -32700,
        message: "invalid JSON",
    };
    pub(super) const REQUEST: Self = Self {
        code: -32600,
        message: "invalid JSON-RPC request",
    };
    pub(super) const PARAMS: Self = Self {
        code: -32602,
        message: "invalid method parameters",
    };
    pub(super) const METHOD: Self = Self {
        code: -32601,
        message: "method not found",
    };
    pub(super) const STATE: Self = Self {
        code: -32000,
        message: "initialize and notify initialized before calling tools",
    };
    pub(super) const BUSY: Self = Self {
        code: -32001,
        message: "one tool call is already active",
    };
    pub(super) const SIZE: Self = Self {
        code: -32600,
        message: "MCP message exceeds 16 MiB",
    };
}

pub(super) fn parse(bytes: &[u8]) -> Result<Message, (Option<Id>, Fault)> {
    // The first parse distinguishes bad JSON from an invalid envelope. A null
    // id is never a notification; MCP request IDs are strings or integers.
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| (None, Fault::PARSE))?;
    if !value.is_object() {
        return Err((None, Fault::REQUEST));
    }
    // Deserialize the ID independently so envelope faults retain correlation.
    // Unlike a Value lookup, this refuses duplicate IDs instead of picking one.
    #[derive(Deserialize)]
    struct Identity {
        #[serde(default)]
        id: Option<Id>,
    }
    let id = serde_json::from_slice::<Identity>(bytes)
        .ok()
        .and_then(|identity| identity.id);
    if value.get("id").is_some_and(serde_json::Value::is_null) {
        return Err((id, Fault::REQUEST));
    }
    let message: Message =
        serde_json::from_slice(bytes).map_err(|_| (id.clone(), Fault::REQUEST))?;
    if message.jsonrpc != "2.0"
        || message.method.len() > 128
        || message
            .id
            .as_ref()
            .is_some_and(|id| matches!(id, Id::Text(s) if s.len() > 256))
    {
        return Err((id, Fault::REQUEST));
    }
    Ok(message)
}

/// Never retain more than the byte limit. An excess closes the session rather
/// than draining an attacker-controlled unbounded line. Final lines need LF.
type FramedLine = Option<Result<Vec<u8>, Fault>>;

pub(super) fn read_line(reader: &mut impl BufRead) -> io::Result<FramedLine> {
    let mut line = Vec::new();
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            return Ok(if line.is_empty() {
                None
            } else {
                Some(Err(Fault::PARSE))
            });
        }
        let end = buffer.iter().position(|&byte| byte == b'\n');
        let count = end.map_or(buffer.len(), |at| at + 1);
        if line.len().saturating_add(count) > MAX_MESSAGE {
            return Ok(Some(Err(Fault::SIZE)));
        }
        line.extend_from_slice(
            buffer
                .get(..count)
                .ok_or_else(|| io::Error::other("invalid reader buffer"))?,
        );
        reader.consume(count);
        if end.is_some() {
            return Ok(Some(Ok(line)));
        }
    }
}

#[derive(Deserialize)]
pub(super) struct Initialize {
    #[serde(rename = "protocolVersion")]
    pub(super) version: String,
    #[serde(rename = "capabilities")]
    _capabilities: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "clientInfo")]
    pub(super) client: ClientInfo,
}

#[derive(Deserialize)]
pub(super) struct ClientInfo {
    pub(super) name: String,
    pub(super) version: String,
}

#[derive(Deserialize)]
pub(super) struct Cancellation {
    #[serde(rename = "requestId")]
    pub(super) id: Id,
    // The optional reason is deliberately not retained or logged.
}

pub(super) fn params<T: serde::de::DeserializeOwned>(message: &Message) -> Result<T, Fault> {
    serde_json::from_str(message.params.as_ref().map_or("{}", |raw| raw.get()))
        .map_err(|_| Fault::PARAMS)
}

pub(super) fn empty_params(message: &Message) -> Result<(), Fault> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Empty {
        #[serde(rename = "_meta", default)]
        _meta: Option<serde_json::Map<String, serde_json::Value>>,
    }
    params::<Empty>(message).map(|_| ())
}

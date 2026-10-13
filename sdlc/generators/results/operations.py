"""Native operation contracts: ABI arguments, ownership, diagnostics and publication.

Semantic decoding, admission, allocation and waiting remain Rust helpers. No
operation record contains executable wrapper bodies.
"""
from dataclasses import dataclass


@dataclass(frozen=True)
class Argument:
    name: str
    abi: str
    ownership: str = 'input'
    count: str | None = None
    lifetime: str = 'call'


@dataclass(frozen=True)
class Operation:
    module: str
    name: str
    documentation: str
    arguments: tuple[Argument, ...]
    native: str
    inputs: tuple[str, ...]
    outputs: tuple[str, ...]
    diagnostic: str
    result: str = 'std::ffi::c_int'
    fallback: str | None = None
    native_unsafe: bool = False


OPERATIONS = (
    Operation('session', 'thinkthen_session_wait',
        "/// Block for requested readiness without consuming a packet or joining provider\n/// work. timeout_ms is the maximum condition wait in milliseconds; zero checks\n/// readiness immediately. RESULT ignores input room; INPUT also watches intake;\n/// SETTLEMENT ignores cancel to drain final packets after cancellation. Output\n/// readiness takes priority over cancel. Success preserves thread-local errors;\n/// failures preserve out_ready and set only the calling thread's diagnostic.\n/// # Safety\n/// session is live for the whole wait, including concurrent cancel/read/push.\n/// Shared operations are synchronized internally. out_ready is writable and\n/// nonnull. Free only after every concurrent operation, including wait, returns.\n",
        (
            Argument('session', '*const SessionHandle', 'borrow'),
            Argument('interest', 'u32', 'input'),
            Argument('timeout_ms', 'u32', 'input'),
            Argument('out_ready', '*mut u32', 'output'),
        ), 'wait_ready', ('session', 'interest', 'timeout_ms'), ('out_ready',), 'session_call'),
    Operation('session', 'thinkthen_session_try_read',
        '/// Transfer one owned packet without waiting for native work.\n/// RESULT transfers an owner; PENDING and END write NULL to out. Validate both\n/// output slots before popping. A result survives session_free and engine_free.\n/// # Safety\n/// session is live. status and out are writable, nonnull, distinct addresses.\n/// Alignment, readable/writable ranges and partial overlaps are caller duties.\n',
        (
            Argument('session', '*mut SessionHandle', 'borrow'),
            Argument('status', '*mut u32', 'output'),
            Argument('out', '*mut *mut SessionResultHandle', 'transfer', lifetime='result_free'),
        ), 'read_packet', ('session', '@None'), ('status', 'out'), 'session_call'),
    Operation('session', 'thinkthen_session_next_packet',
        "/// Block and transfer one packet without polling or joining provider work.\n/// interest is WAIT_RESULT or WAIT_SETTLEMENT. RESULT wakes promptly on cancel\n/// with ECANCELLED; output already queued wins. SETTLEMENT drains later terminal\n/// facts after cancel. RESULT and END match try_read, including packet ownership.\n/// Errors preserve both output slots and affect only this thread's session error.\n/// # Safety\n/// The try_read slot obligations apply. Keep session live throughout the wait;\n/// concurrent cancel/push/finish are synchronized. Free after every call returns.\n",
        (
            Argument('session', '*mut SessionHandle', 'borrow'),
            Argument('interest', 'u32', 'input'),
            Argument('status', '*mut u32', 'output'),
            Argument('out', '*mut *mut SessionResultHandle', 'transfer', lifetime='result_free'),
        ), 'read_packet', ('session', '@Some(interest)'), ('status', 'out'), 'session_call'),
    Operation('session', 'thinkthen_session_result_json',
        '/// Borrow canonical immutable packet JSON, including an owned trailing NUL.\n/// The length excludes that terminator. Bytes stay valid until result_free,\n/// even after session_free and engine_free. Concurrent access is synchronized.\n/// Immediate failure preserves both outputs and records the session diagnostic.\n/// # Safety\n/// result is live throughout the call. out and out_len are nonnull, writable,\n/// distinct addresses; alignment and partial overlaps are caller duties.\n',
        (
            Argument('result', '*const SessionResultHandle', 'borrow'),
            Argument('out', '*mut *const c_char', 'borrowed-output', lifetime='result_free'),
            Argument('out_len', '*mut usize', 'output'),
        ), 'packet_json', ('result',), ('out', 'out_len'), 'session_call'),
    Operation('session', 'thinkthen_session_result_view',
        '/// Borrow the complete immutable typed packet graph. Nested views retain every\n/// known field and unknown extension, and survive engine/session destruction.\n/// Repeated and concurrent access returns the same view. Only result_free ends\n/// all view lifetimes. Failure leaves out unchanged.\n/// # Safety\n/// result stays live during access and while using any borrowed view; out is\n/// nonnull, aligned writable pointer storage. Free only after all borrowers finish.\n',
        (
            Argument('result', '*const SessionResultHandle', 'borrow'),
            Argument('out', '*mut *const crate::session::views_generated::thinkthen_complete_session_packet_v1', 'borrowed-output', lifetime='result_free'),
        ), 'packet_view', ('result',), ('out',), 'session_call'),
    Operation('session', 'thinkthen_engine_usage_persistence_v1',
        "/// Observe this engine's live usage persistence without waiting for a writer.\n/// On success assign both outputs; Failed is an observation, not an operation\n/// error. Advice is a static counted UTF-8 view, or NULL with zero length.\n/// Success preserves the calling-thread session diagnostic.\n/// # Safety\n/// engine is live for the call. Both outputs are nonnull, aligned, writable,\n/// nonoverlapping slots. Destruction waits for concurrent callers to return.\n",
        (
            Argument('engine', '*const EngineHandle', 'borrow'),
            Argument('out_state', '*mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1', 'output'),
            Argument('out_advice', '*mut crate::session::views::thinkthen_complete_utf8_v1', 'borrowed-output', lifetime='library_unload'),
        ), 'usage_status', ('engine', '@thinkthen::Engine::usage_persistence'), ('out_state', 'out_advice'), 'session_call'),
    Operation('session', 'thinkthen_engine_finish_usage_status_v1',
        "/// Finish this engine's current usage deltas and observe their persistence.\n/// Only usage-lock acquisition has a deadline; other filesystem work can take\n/// longer. Written covers current deltas only, not future calls or other engines.\n/// Failed carries static native safe advice readable after engine destruction\n/// while the library remains loaded. Every nonzero return preserves outputs.\n/// # Safety\n/// The engine and output obligations of usage_persistence_v1 apply.\n",
        (
            Argument('engine', '*const EngineHandle', 'borrow'),
            Argument('out_state', '*mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1', 'output'),
            Argument('out_advice', '*mut crate::session::views::thinkthen_complete_utf8_v1', 'borrowed-output', lifetime='library_unload'),
        ), 'usage_status', ('engine', '@thinkthen::Engine::finish_usage_status'), ('out_state', 'out_advice'), 'session_call'),
    Operation('typed', 'thinkthen_session_try_push_typed',
        '/// Push a counted typed feed descriptor. Rust owns framing and admission.\n/// No input pointer survives return; status is published only on success.\n/// # Safety\n/// session is live, status is writable and nonnull, and descriptor satisfies\n/// the finite aligned carrier-tree obligations of the typed constructor.\n',
        (
            Argument('session', '*const SessionHandle', 'borrow'),
            Argument('descriptor', '*const carriers::thinkthen_request_session_descriptor_v1', 'input'),
            Argument('status', '*mut u32', 'output'),
        ), 'push_typed', ('session', 'descriptor', '@false'), ('status',), 'session_native_call', native_unsafe=True),
    Operation('typed', 'thinkthen_session_push_typed',
        '/// Wait for input room and transfer a typed descriptor without consuming output.\n/// Closed intake retains nothing and never inspects the descriptor tree.\n/// No input pointer survives return; status is published only on success.\n/// # Safety\n/// session is live, status is writable and nonnull, and descriptor satisfies\n/// the finite aligned carrier-tree obligations of the typed constructor.\n',
        (
            Argument('session', '*const SessionHandle', 'borrow'),
            Argument('descriptor', '*const carriers::thinkthen_request_session_descriptor_v1', 'input'),
            Argument('status', '*mut u32', 'output'),
        ), 'push_typed', ('session', 'descriptor', '@true'), ('status',), 'session_native_call', native_unsafe=True),
    Operation('engine', 'thinkthen_error_message', "/// The calling thread's last message on this engine; never null.\n///\n/// # Safety\n///\n/// `engine` is null or a live engine.\n/// The message for the last failure the calling thread recorded on this\n/// engine. Its borrowed pointer stays valid until that thread records its\n/// next failure on this engine, the engine is freed, or the thread exits.\n/// Another thread's calls never replace it. Never free the pointer.\n/// A deadline's message names the limit and its value. Never NULL: before\n/// any failure it names that nothing failed yet. With a null engine it is\n/// the calling thread's last failed build's message in a distinct slot, valid\n/// until that thread's next thinkthen_engine_new or thinkthen_engine_new_with\n/// call, or its exit. Otherwise it names that no engine came.\n",
        (Argument('engine', '*const EngineHandle', 'nullable-borrow'),),
        'message', ('engine',), (), 'engine_guard', '*const c_char', 'NO_MESSAGE.as_ptr()'),
    Operation('engine', 'thinkthen_error_facts_json', "/// Borrow the last failure's call facts on this thread and engine, or null.\n///\n/// # Safety\n///\n/// `engine` is null or a live engine.\n/// Borrow the calling thread's last failed call's facts on this engine, or NULL\n/// when no failure with started-call facts exists. The pointer has the same\n/// lifetime as thinkthen_error_message and must not be freed.\n",
        (Argument('engine', '*const EngineHandle', 'nullable-borrow'),),
        'facts', ('engine',), (), 'engine_guard', '*const c_char', 'std::ptr::null()'),
    Operation('engine', 'thinkthen_error_retryable', "/// 1 when the calling thread's last failure here could pass later.\n///\n/// # Safety\n///\n/// `engine` is null or a live engine.\n/// Whether the same call could pass later: 1 for a backend status the\n/// engine retries, such as busy or failing; 0 for a transport failure,\n/// which may already have reached the backend, for a refused key, and for\n/// every kind but the backend kind. Zero when nothing failed. With a null\n/// engine it follows the calling thread's last failed build, else zero.\n",
        (Argument('engine', '*const EngineHandle', 'nullable-borrow'),),
        'retryable', ('engine',), (), 'engine_guard', 'std::ffi::c_int', '0'),
    Operation('engine', 'thinkthen_error_code', "/// The calling thread's last code here; with a null engine, its last failed\n/// build's code, else the usage code.\n///\n/// # Safety\n///\n/// `engine` is null or a live engine.\n/// The code of the calling thread's last failure on this engine: the value\n/// the failing call returned, THINKTHEN_OK when nothing failed yet. Success\n/// does not clear it, so read it when a call fails. With a null engine it\n/// is the calling thread's last failed build's code, else THINKTHEN_EUSAGE,\n/// because no engine holds a failure.\n",
        (Argument('engine', '*const EngineHandle', 'nullable-borrow'),),
        'code', ('engine',), (), 'engine_guard', 'std::ffi::c_int', 'DEFECT'),

    Operation('session', 'thinkthen_session_try_push', '/// Admit one owned descriptor through the shared native decoder without waiting.\n/// ACCEPTED retains decoded data; FULL and CLOSED retain nothing. Check native\n/// capacity before decoding or allocating retained content. A concurrent closure\n/// returns CLOSED without publishing. On OK status always receives a value.\n/// # Safety\n/// session is live, descriptor_json has descriptor_len readable UTF-8 bytes\n/// (NULL requires zero), and status is nonnull and writable.\n',
        (
            Argument('session', '*mut SessionHandle', 'borrow', None),
            Argument('descriptor_json', '*const c_char', 'input', 'descriptor_len'),
            Argument('descriptor_len', 'usize', 'input', None),
            Argument('status', '*mut u32', 'output', None),
        ), 'try_push_json', ('session', 'descriptor_json', 'descriptor_len'), ('status',), 'session_call', native_unsafe=True),
    Operation('session', 'thinkthen_session_push', '/// Wait for input room and transfer one descriptor without consuming output.\n/// Pending output cannot wake this producer. Closure or cancellation returns\n/// CLOSED without decoding the payload. Failures preserve status and publish\n/// only the calling thread diagnostic. No input bytes survive return.\n/// # Safety\n/// session stays live throughout the wait. descriptor_json has descriptor_len\n/// readable bytes (NULL requires zero); status is nonnull writable storage.\n',
        (
            Argument('session', '*mut SessionHandle', 'borrow', None),
            Argument('descriptor_json', '*const c_char', 'input', 'descriptor_len'),
            Argument('descriptor_len', 'usize', 'input', None),
            Argument('status', '*mut u32', 'output', None),
        ), 'push_json', ('session', 'descriptor_json', 'descriptor_len'), ('status',), 'session_native_call', native_unsafe=True),
    Operation('session', 'thinkthen_session_finish', '/// Fix input EOF or a canonical closed reader failure. NULL with zero means EOF;\n/// other inputs decode kind io, utf8, invalid_input or producer and optional location.\n/// No input bytes survive return. A changed finish refuses without changing intake.\n/// # Safety\n/// session is live and failure_json has failure_len readable UTF-8 bytes.\n',
        (
            Argument('session', '*mut SessionHandle', 'borrow', None),
            Argument('failure_json', '*const c_char', 'input', 'failure_len'),
            Argument('failure_len', 'usize', 'input', None),
        ), 'finish_json', ('session', 'failure_json', 'failure_len'), (), 'session_call', native_unsafe=True),

)

# Native status projections preserve their established public integer values.
STATUSES = (('THINKTHEN_SESSION_ACCEPTED_V1', 0, 'Push transferred one descriptor to the session.'), ('THINKTHEN_SESSION_FULL_V1', 1, 'Push retained nothing; retry the same descriptor.'), ('THINKTHEN_SESSION_CLOSED_V1', 2, 'Intake closed; stop advancing the producer.'), ('THINKTHEN_SESSION_RESULT_V1', 0, 'Read transferred one independent packet owner.'), ('THINKTHEN_SESSION_PENDING_V1', 1, 'Work has not settled and no packet is ready.'), ('THINKTHEN_SESSION_END_V1', 2, 'Terminal was read and no more output will arrive.'), ('THINKTHEN_SESSION_WAIT_RESULT_V1', 0, 'Wait only for results, end or cancel; input room cannot wake this wait.'), ('THINKTHEN_SESSION_WAIT_INPUT_V1', 1, 'Wait for result, input room/closure, end or cancel.'), ('THINKTHEN_SESSION_WAIT_SETTLEMENT_V1', 2, 'Drain output after cancel without waking repeatedly for cancellation.'), ('THINKTHEN_SESSION_READY_RESULT_V1', 0, 'A result or terminal packet is ready; no packet has been consumed.'), ('THINKTHEN_SESSION_READY_INPUT_V1', 1, 'Caller-supplied input has room.'), ('THINKTHEN_SESSION_READY_CLOSED_V1', 2, 'Caller-supplied input has closed.'), ('THINKTHEN_SESSION_READY_END_V1', 3, 'Every output packet has been consumed.'), ('THINKTHEN_SESSION_READY_CANCELLED_V1', 4, 'Intake was cancelled; settlement can still be pending.'), ('THINKTHEN_SESSION_READY_TIMEOUT_V1', 5, 'The caller-given timeout passed.'))
PROJECTIONS = (
    ('push_status', 'thinkthen::RequestSessionPushStatus', (
        ('Accepted', 'THINKTHEN_SESSION_ACCEPTED_V1'),
        ('Full', 'THINKTHEN_SESSION_FULL_V1'),
        ('Closed', 'THINKTHEN_SESSION_CLOSED_V1'))),
    ('ready_status', 'thinkthen::RequestSessionReady', (
        ('Result', 'THINKTHEN_SESSION_READY_RESULT_V1'),
        ('Input', 'THINKTHEN_SESSION_READY_INPUT_V1'),
        ('Closed', 'THINKTHEN_SESSION_READY_CLOSED_V1'),
        ('End', 'THINKTHEN_SESSION_READY_END_V1'),
        ('Cancelled', 'THINKTHEN_SESSION_READY_CANCELLED_V1'),
        ('Timeout', 'THINKTHEN_SESSION_READY_TIMEOUT_V1'))),
)

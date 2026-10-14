<?php
declare(strict_types=1);
namespace ThinkThen\Internal;

function requireBlockingContext(): void
{
    if (\Fiber::getCurrent() !== null) throw new \ThinkThen\UsageFailure('blocking wait is unavailable inside a fiber; use poll');
}

namespace ThinkThen;
use ThinkThen\Internal\Requests as Input;

/** Own a native session and retain its client until the operation closes. */
final class Operation
{
    private ?\FFI\CData $session = null;
    private ?\Iterator $producer = null;
    private ?string $pending = null;
    private bool $advance = false;
    private bool $started = false;
    private ?\Throwable $producerError = null;
    private bool $single;
    private array $rows = [];
    private array $packets = [];
    private ?\ThinkThen\Results\Node $terminal = null;
    private function __construct(private ?Client $client, private readonly \FFI $ffi, \FFI\CData $engine,
        private readonly string $function, string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input,
        \ThinkThen\Inputs\Options $options, private readonly ?Cancellation $token)
    {
        $this->single = !($input instanceof Files || $input instanceof \Traversable || (is_array($input) && array_is_list($input)));
        if ($input instanceof \Traversable) $this->producer = $input instanceof \Iterator ? $input : $input->getIterator();
        $request = new \ThinkThen\Internal\Request($ffi, $function, $question, $input, $options);
        $buffers = [];
        $surface = $request->utf8('php', $buffers);
        $owner = $ffi->new('struct thinkthen_session *');
        $this->check($ffi->thinkthen_session_new_named_with_surface($engine, \FFI::addr($request->value), $surface, \FFI::addr($owner)));
        $this->session = $owner;
    }
    private function check(int $code): void
    {
        if ($code) \ThinkThen\Internal\ErrorFactory::raise($this->ffi, $code, true);
    }
    public function poll(): ?\ThinkThen\Results\Node
    {
        if ($this->session === null) throw new UsageFailure('operation is closed');
        if ($this->token?->cancelled) $this->ffi->thinkthen_session_cancel($this->session);
        $packet = $this->readPacket();
        if ($this->terminal === null && $this->producer !== null) $this->feed();
        return $packet;
    }
    private function readPacket(bool $blocking = false): ?\ThinkThen\Results\Node
    {
        $status = $this->ffi->new('uint32_t');
        $owner = $this->ffi->new('struct thinkthen_session_result *');
        $code = $blocking
            ? $this->ffi->thinkthen_session_next_packet($this->session, \ThinkThen\Internal\Session\THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, \FFI::addr($status), \FFI::addr($owner))
            : $this->ffi->thinkthen_session_try_read($this->session, \FFI::addr($status), \FFI::addr($owner));
        $this->check($code);
        if ($status->cdata === \ThinkThen\Internal\Session\THINKTHEN_SESSION_END_V1 && $this->terminal === null) throw new DefectFailure('native session ended without terminal facts');
        if ($status->cdata !== \ThinkThen\Internal\Session\THINKTHEN_SESSION_RESULT_V1) return null;
        try {
            $view = $this->ffi->new('const thinkthen_complete_session_packet_v1 *');
            $this->check($this->ffi->thinkthen_session_result_view($owner, \FFI::addr($view)));
            $packet = \ThinkThen\Internal\Decoder::native($view);
            $this->packets[] = $packet;
            if ($packet->kind === 'row') $this->rows[] = $packet->value;
            elseif ($packet->kind === 'aggregate') array_push($this->rows, ...(is_array($packet->value) ? $packet->value : [$packet->value]));
            elseif ($packet->kind === 'terminal') $this->terminal = $packet;
            return $packet;
        } finally { $this->ffi->thinkthen_session_result_free($owner); }
    }
    /** Wait for native readiness without consuming a packet; poll stays nonblocking. */
    public function wait(): void
    {
        \ThinkThen\Internal\requireBlockingContext();
        if ($this->session === null) throw new UsageFailure('operation is closed');
        $ready = $this->ffi->new('uint32_t');
        $interest = $this->producer !== null ? \ThinkThen\Internal\Session\THINKTHEN_SESSION_WAIT_INPUT_V1 : \ThinkThen\Internal\Session\THINKTHEN_SESSION_WAIT_SETTLEMENT_V1;
        // Tokens and async signals need the VM to observe host cancellation.
        $cooperative = $this->token !== null || (function_exists('pcntl_async_signals') && pcntl_async_signals());
        $this->check($this->ffi->thinkthen_session_wait($this->session, $interest, $cooperative ? 50 : 0xffffffff, \FFI::addr($ready)));
    }
    public function result(): \ThinkThen\Results\Result
    {
        try {
            \ThinkThen\Internal\requireBlockingContext();
            while ($this->terminal === null) {
                // Host generators, tokens and async signal handlers must regain the VM.
                $blocking = $this->producer === null && $this->token === null
                    && (!function_exists('pcntl_async_signals') || !pcntl_async_signals());
                if ($blocking) $this->readPacket(true);
                else {
                    $this->poll();
                    if ($this->terminal === null) $this->wait();
                }
            }
            if ($this->terminal->has('failure')) {
                $failure = $this->terminal->failure;
                $detail = $failure->error;
                $class = \ThinkThen\Internal\ErrorFactory::classFor($detail->kind);
                throw new $class($detail->message, $failure, $this->rows, $this->terminal, $this->packets, $this->producerError);
            }
            return \ThinkThen\Internal\ResultBuilder::complete($this->function, $this->single, $this->rows, $this->terminal, $this->packets);
        } finally { $this->close(); }
    }
    private function feed(): void
    {
        if ($this->pending === null) {
            try {
                if (!$this->started) { $this->producer->rewind(); $this->started = true; }
                if ($this->advance) $this->producer->next();
                $this->advance = false;
                if (!$this->producer->valid()) {
                    $finished = true;
                } else {
                    $finished = false;
                    $this->pending = \ThinkThen\Internal\Encoder::encode(new Input\SessionDescriptor(new Input\Item(\ThinkThen\Internal\descriptor($this->producer->current()))));
                }
            } catch (\Throwable $error) {
                $this->producerError = $error;
                $failure = \ThinkThen\Internal\Encoder::encode(new Input\ReaderFailureProducer());
                $this->check($this->ffi->thinkthen_session_finish($this->session, $failure, strlen($failure)));
                $this->producer = null;
                return;
            }
            if ($finished) {
                $this->check($this->ffi->thinkthen_session_finish($this->session, null, 0));
                $this->producer = null;
                return;
            }
        }
        $status = $this->ffi->new('uint32_t');
        $this->check($this->ffi->thinkthen_session_try_push($this->session, $this->pending, strlen($this->pending), \FFI::addr($status)));
        if ($status->cdata === \ThinkThen\Internal\Session\THINKTHEN_SESSION_ACCEPTED_V1) { $this->pending = null; $this->advance = true; }
        elseif ($status->cdata === \ThinkThen\Internal\Session\THINKTHEN_SESSION_CLOSED_V1) { $this->pending = null; $this->producer = null; }
    }
    public function cancel(): void
    {
        if ($this->session !== null) $this->ffi->thinkthen_session_cancel($this->session);
    }
    public function close(): void
    {
        if ($this->session !== null) {
            $this->ffi->thinkthen_session_cancel($this->session);
            $this->ffi->thinkthen_session_free($this->session);
            $this->session = null;
        }
        $this->producer = null;
        $this->pending = null;
        $this->client = null;
    }
    public function __destruct() { $this->close(); }
    public function __debugInfo(): array { return ['type' => self::class]; }
}

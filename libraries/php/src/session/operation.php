<?php
declare(strict_types=1);
namespace ThinkThen;

/** Own a native session. A client can close without depending on its batches. */
final class Operation
{
    private ?\FFI\CData $session = null;
    private ?\Iterator $producer = null;
    private ?string $pending = null;
    private bool $advance = false;
    private bool $started = false;
    private array $rows = [];
    private array $packets = [];
    private ?\ThinkThen\Results\Node $terminal = null;
    public function __construct(private readonly \FFI $ffi, \FFI\CData $engine,
        string $function, string|array|\stdClass $question, mixed $input,
        array $options, private readonly ?Cancellation $token)
    {
        if ($input instanceof FileInput) {
            $source = ['kind' => 'source', 'source' => ['paths' => $input->paths, 'reading' => (object)$input->reading, 'media' => 'text']];
        } elseif ($input instanceof \Traversable) {
            $this->producer = $input instanceof \Iterator ? $input : $input->getIterator();
            $source = ['kind' => 'feed', 'name' => 'php'];
        } else {
            $records = is_array($input) && array_is_list($input) ? $input : [$input];
            $source = ['kind' => match ($function) { 'find' => 'units', 'relate' => 'entities', default => 'records' },
                'items' => array_map(Client::descriptor(...), $records)];
        }
        $request = Client::json(['schema' => \ThinkThen\Session\REQUEST_VERSION,
            'call' => ['function' => $function,
                'question' => is_string($question) ? ['kind' => 'text', 'text' => $question] : ['kind' => 'definition', 'value' => $question],
                'input' => $source, 'options' => (object)$options]]);
        $owner = $ffi->new('struct thinkthen_session *');
        $this->check($ffi->thinkthen_session_new_with_surface($engine, $request, strlen($request), 'php', 3, \FFI::addr($owner)));
        $this->session = $owner;
    }
    private function check(int $code): void
    {
        if ($code) Client::throwImmediate($code, Client::text($this->ffi->thinkthen_session_error_message()));
    }
    public function poll(): ?\ThinkThen\Results\Node
    {
        if ($this->session === null) throw new UsageFailure('operation is closed');
        if ($this->token?->cancelled) $this->ffi->thinkthen_session_cancel($this->session);
        $status = $this->ffi->new('uint32_t');
        $owner = $this->ffi->new('struct thinkthen_session_result *');
        $this->check($this->ffi->thinkthen_session_try_read($this->session, \FFI::addr($status), \FFI::addr($owner)));
        if ($status->cdata === \ThinkThen\Session\THINKTHEN_SESSION_END_V1 && $this->terminal === null) throw new DefectFailure('native session ended without terminal facts');
        if ($status->cdata !== \ThinkThen\Session\THINKTHEN_SESSION_RESULT_V1) return null;
        try {
            $text = $this->ffi->new('const char *');
            $length = $this->ffi->new('size_t');
            $this->check($this->ffi->thinkthen_session_result_json($owner, \FFI::addr($text), \FFI::addr($length)));
            $packet = \ThinkThen\Results\Decoder::packet(\FFI::string($text, $length->cdata));
            $this->packets[] = $packet;
            if ($packet->kind === 'row') $this->rows[] = $packet->value;
            elseif ($packet->kind === 'aggregate') $this->rows = is_array($packet->value) ? $packet->value : [$packet->value];
            elseif ($packet->kind === 'terminal') $this->terminal = $packet;
            return $packet;
        } finally { $this->ffi->thinkthen_session_result_free($owner); }
    }
    public function result(): \ThinkThen\Results\Completed
    {
        try {
            while ($this->terminal === null) {
                $this->poll();
                if ($this->terminal !== null) break;
                if ($this->producer !== null) $this->feed();
                usleep(1000);
            }
            if ($this->terminal->has('failure')) {
                $failure = $this->terminal->failure;
                $detail = $failure->error;
                $class = match ($detail->kind) {
                    'usage' => UsageFailure::class, 'backend' => BackendFailure::class, 'deadline' => DeadlineFailure::class,
                    'local' => LocalFailure::class, 'cancelled' => CancelledFailure::class, default => DefectFailure::class,
                };
                throw new $class($detail->message, $failure, $this->rows, $this->terminal, $this->packets);
            }
            return new \ThinkThen\Results\Completed($this->rows, $this->terminal, $this->packets);
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
                    $this->pending = Client::json(['item' => Client::descriptor($this->producer->current())]);
                }
            } catch (\Throwable) {
                $failure = '{"kind":"invalid_input"}';
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
        if ($status->cdata === \ThinkThen\Session\THINKTHEN_SESSION_ACCEPTED_V1) { $this->pending = null; $this->advance = true; }
        elseif ($status->cdata === \ThinkThen\Session\THINKTHEN_SESSION_CLOSED_V1) { $this->pending = null; $this->producer = null; }
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
    }
    public function __destruct() { $this->close(); }
    public function __debugInfo(): array { return ['type' => self::class]; }
}

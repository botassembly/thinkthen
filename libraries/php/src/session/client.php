<?php
declare(strict_types=1);
namespace ThinkThen;

/** Native sessions own admission, scheduling, cache and every reading rule. */
final class Client
{
    private \FFI $ffi;
    private ?\FFI\CData $engine = null;
    private \WeakMap $operations;
    public function __construct(array $settings = [], ?string $library = null)
    {
        $this->operations = new \WeakMap();
        $library ??= dirname(__DIR__, 2).'/native/libthinkthen.so';
        try {
            $declarations = @file_get_contents(__DIR__.'/ffi_generated.h');
            if ($declarations === false) throw new \RuntimeException('native declarations are missing');
            $this->ffi = \FFI::cdef($declarations, $library);
        } catch (\Throwable $error) { throw new LocalFailure('native library could not be loaded', previous: $error); }
        $engine = $this->ffi->thinkthen_engine_new_with(self::json((object)$settings));
        if ($engine === null || \FFI::isNull($engine)) {
            $code = $this->ffi->thinkthen_error_code(null);
            self::throwImmediate($code, self::text($this->ffi->thinkthen_error_message(null)));
        }
        $this->engine = $engine;
    }
    public static function text(mixed $value): string { return is_string($value) ? $value : \FFI::string($value); }
    public static function json(mixed $value): string
    {
        try { return json_encode($value, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION); }
        catch (\JsonException $error) { throw new UsageFailure('input cannot be converted to native JSON', previous: $error); }
    }
    public static function files(array $paths, array $reading = []): FileInput { return new FileInput($paths, $reading); }
    public static function item(mixed $value, array $fields = []): ItemInput { return new ItemInput($value, $fields); }
    public static function descriptor(mixed $value): array
    {
        $fields = $value instanceof ItemInput ? $value->fields : [];
        if ($value instanceof ItemInput) $value = $value->value;
        return ['original' => is_string($value) ? ['kind' => 'text', 'text' => $value] : ['kind' => 'json', 'value' => $value], ...$fields];
    }
    public function start(string $function, string|array|\stdClass $question, mixed $input,
        array $options = [], ?Cancellation $cancel = null): Operation
    {
        if ($this->engine === null) throw new UsageFailure('client is closed');
        if ($cancel?->cancelled) throw new CancelledFailure('the call was cancelled');
        $operation = new Operation($this->ffi, $this->engine, $function, $question, $input, $options, $cancel);
        $this->operations[$operation] = true;
        return $operation;
    }
    private function call(string $function, string|array|\stdClass $question, mixed $input,
        array $options, ?Cancellation $cancel): \ThinkThen\Results\Completed
    {
        $operation = $this->start($function, $question, $input, $options, $cancel);
        try { return $operation->result(); } finally { $operation->close(); }
    }
    public function decide(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('decide', $question, $input, $options, $cancel); }
    public function choose(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('choose', $question, $input, $options, $cancel); }
    public function tag(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('tag', $question, $input, $options, $cancel); }
    public function score(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('score', $question, $input, $options, $cancel); }
    public function filter(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('filter', $question, $input, $options, $cancel); }
    public function rank(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('rank', $question, $input, $options, $cancel); }
    public function find(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('find', $question, $input, $options, $cancel); }
    public function annotate(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('annotate', $question, $input, $options, $cancel); }
    public function recognize(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('recognize', $question, $input, $options, $cancel); }
    public function relate(string|array|\stdClass $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('relate', $question, $input, $options, $cancel); }
    public function usage_persistence(): UsageStatus { return $this->usageStatus(false); }
    public function finish_usage_status(): UsageStatus { return $this->usageStatus(true); }
    private function usageStatus(bool $finish): UsageStatus
    {
        if ($this->engine === null) throw new UsageFailure('client is closed');
        $state = $this->ffi->new('thinkthen_complete_usage_persistence_v1');
        $advice = $this->ffi->new('thinkthen_complete_utf8_v1');
        $code = $finish
            ? $this->ffi->thinkthen_engine_finish_usage_status_v1($this->engine, \FFI::addr($state), \FFI::addr($advice))
            : $this->ffi->thinkthen_engine_usage_persistence_v1($this->engine, \FFI::addr($state), \FFI::addr($advice));
        if ($code !== 0) self::throwImmediate($code, self::text($this->ffi->thinkthen_session_error_message()));
        return new UsageStatus(UsagePersistenceState::from($state->kind),
            \FFI::isNull($advice->data) ? null : \FFI::string($advice->data, $advice->len));
    }
    public function close(): void
    {
        foreach ($this->operations as $operation => $_) $operation->close();
        if ($this->engine !== null) {
            $this->ffi->thinkthen_engine_free($this->engine);
            $this->engine = null;
        }
    }
    public function __destruct() { $this->close(); }
    public static function throwImmediate(int $code, string $message): never
    {
        $class = match ($code) {
            \ThinkThen\Session\THINKTHEN_EUSAGE => UsageFailure::class,
            \ThinkThen\Session\THINKTHEN_EBACKEND => BackendFailure::class,
            \ThinkThen\Session\THINKTHEN_EDEADLINE => DeadlineFailure::class,
            \ThinkThen\Session\THINKTHEN_ELOCAL => LocalFailure::class,
            \ThinkThen\Session\THINKTHEN_ECANCELLED => CancelledFailure::class, default => DefectFailure::class,
        };
        throw new $class($message);
    }
    public function __debugInfo(): array { return ['type' => self::class]; }
}

final readonly class FileInput
{
    public function __construct(public array $paths, public array $reading = []) {}
}
final readonly class ItemInput
{
    public function __construct(public mixed $value, public array $fields = []) {}
}

final readonly class UsageStatus
{
    public function __construct(public UsagePersistenceState $state, public ?string $advice) {}
}

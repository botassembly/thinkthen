<?php
declare(strict_types=1);
namespace ThinkThen;

/** Native sessions own admission, scheduling, cache and every reading rule. */
final class Client
{
    use NamedCalls;
    private \FFI $ffi;
    private ?\FFI\CData $engine = null;
    private \WeakMap $operations;
    public function __construct(array|\ThinkThen\Inputs\Settings $settings = [], ?string $library = null)
    {
        $this->operations = new \WeakMap();
        $library ??= dirname(__DIR__, 2).'/native/libthinkthen.so';
        try {
            $declarations = @file_get_contents(__DIR__.'/ffi_generated.h');
            if ($declarations === false) throw new \RuntimeException('native declarations are missing');
            $this->ffi = \FFI::cdef($declarations, $library);
        } catch (\Throwable $error) { throw new LocalFailure('native library could not be loaded', previous: $error); }
        $engine = $this->ffi->thinkthen_engine_new_with(\ThinkThen\Internal\Encoder::encode(is_array($settings) ? (object)$settings : $settings));
        if ($engine === null || \FFI::isNull($engine)) {
            $code = $this->ffi->thinkthen_error_code(null);
            \ThinkThen\Internal\ErrorFactory::raise($this->ffi, $code, false);
        }
        $this->engine = $engine;
    }
    public static function files(array $paths, array $reading = [], ?string $framing = null, ?string $media = null): Files { return new Files($paths, $reading, $framing, $media); }
    public static function questionFile(string $path): \ThinkThen\Inputs\QuestionSource { return new \ThinkThen\Inputs\QuestionFile($path); }
    public static function questionNamed(string $name): \ThinkThen\Inputs\QuestionSource { return new \ThinkThen\Inputs\QuestionName($name); }
    public static function questionReference(string $reference): \ThinkThen\Inputs\QuestionSource { return new \ThinkThen\Inputs\QuestionReference($reference); }
    public static function images(array $images): Item { return new Item(null, ['images' => $images], false); }
    public static function item(mixed $value, array $fields = []): Item { return new Item($value, $fields); }
    public function start(string $function, string|array|\stdClass|\ThinkThen\Inputs\Document|\ThinkThen\Inputs\QuestionSource $question, mixed $input,
        array|\ThinkThen\Inputs\Options $options = [], ?Cancellation $cancel = null): Operation
    {
        if ($this->engine === null) throw new UsageFailure('client is closed');
        if ($cancel?->cancelled) throw new CancelledFailure('the call was cancelled');
        $operation = new Operation($this, $this->ffi, $this->engine, $function, $question, $input, $options, $cancel);
        $this->operations[$operation] = true;
        return $operation;
    }
    private function call(string $function, string|array|\stdClass|\ThinkThen\Inputs\Document|\ThinkThen\Inputs\QuestionSource $question, mixed $input,
        array|\ThinkThen\Inputs\Options $options, ?Cancellation $cancel): \ThinkThen\Results\Completed
    {
        $operation = $this->start($function, $question, $input, $options, $cancel);
        try { return $operation->result(); } finally { $operation->close(); }
    }
    public function usagePersistence(): UsageStatus { return $this->usageStatus(false); }
    public function finishUsageStatus(): UsageStatus { return $this->usageStatus(true); }
    private function usageStatus(bool $finish): UsageStatus
    {
        if ($this->engine === null) throw new UsageFailure('client is closed');
        $state = $this->ffi->new('thinkthen_complete_usage_persistence_v1');
        $advice = $this->ffi->new('thinkthen_complete_utf8_v1');
        $code = $finish
            ? $this->ffi->thinkthen_engine_finish_usage_status_v1($this->engine, \FFI::addr($state), \FFI::addr($advice))
            : $this->ffi->thinkthen_engine_usage_persistence_v1($this->engine, \FFI::addr($state), \FFI::addr($advice));
        if ($code !== 0) \ThinkThen\Internal\ErrorFactory::raise($this->ffi, $code, true);
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
    public function __debugInfo(): array { return ['type' => self::class]; }
}

final readonly class Files
{
    public function __construct(public array $paths, public array $reading = [], public ?string $framing = null, public ?string $media = null) {}
}
final readonly class Item
{
    public function __construct(public mixed $value, public array $fields = [], public bool $original = true) {}
}

final readonly class UsageStatus
{
    public function __construct(public UsagePersistenceState $state, public ?string $advice) {}
}

<?php
declare(strict_types=1);

/** PHP 8.3 FFI proof. An instance owns its engine; close after all calls and tokens finish. */
final class ThinkThen
{
    private FFI $ffi;
    private FFI\CData $engine;
    private bool $closed = true;
    private const DECLARATIONS = <<<'C'
typedef struct thinkthen_engine thinkthen_engine;
typedef struct thinkthen_cancel_token thinkthen_cancel_token;
typedef struct thinkthen_answer { int outcome; double probability; } thinkthen_answer;
thinkthen_engine *thinkthen_engine_new(void);
thinkthen_engine *thinkthen_engine_new_with(const char *settings_json);
void thinkthen_engine_free(thinkthen_engine *engine);
int thinkthen_error_code(const thinkthen_engine *engine);
int thinkthen_error_retryable(const thinkthen_engine *engine);
const char *thinkthen_error_message(const thinkthen_engine *engine);
const char *thinkthen_error_facts_json(const thinkthen_engine *engine);
thinkthen_cancel_token *thinkthen_cancel_token_new(void);
void thinkthen_cancel(thinkthen_cancel_token *token);
void thinkthen_cancel_token_free(thinkthen_cancel_token *token);
int thinkthen_decide_with_facts_opts(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, thinkthen_answer *, char **, size_t *);
int thinkthen_decide_many_with_facts_opts(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, thinkthen_answer *, char **, size_t *);
/* PHP converts a direct char* return to a PHP string, losing the pointer needed for native free. */
void *thinkthen_call_opts(const thinkthen_engine *, const char *, int64_t, thinkthen_cancel_token *);
int thinkthen_recognize_with_facts_opts(const thinkthen_engine *, const char *, const char *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *, char **, size_t *);
int thinkthen_relate_with_facts_opts(const thinkthen_engine *, const char *, const char *const *, const size_t *, size_t, int64_t, thinkthen_cancel_token *, char **, size_t *, char **, size_t *);
void thinkthen_free_string(char *);
C;

    public function __construct(string $absoluteLibrary, ?string $settingsJson = null)
    {
        if (!str_starts_with($absoluteLibrary, '/')) throw new InvalidArgumentException('absolute library path required');
        $this->ffi = FFI::cdef(self::DECLARATIONS, $absoluteLibrary);
        $settings = $settingsJson === null ? null : $this->checked($settingsJson);
        $created = $settings === null
            ? $this->ffi->thinkthen_engine_new()
            : $this->ffi->thinkthen_engine_new_with($settings);
        if ($created === null || FFI::isNull($created)) {
            $null = $this->ffi->cast('thinkthen_engine *', 0);
            $code = $this->ffi->thinkthen_error_code($null);
            $retry = $this->ffi->thinkthen_error_retryable($null);
            $message = $this->ffi->thinkthen_error_message($null);
            throw new ThinkThenFailure($code, (bool)$retry, is_string($message) ? $message : FFI::string($message));
        }
        $this->engine = $created;
        $this->closed = false;
    }

    public function close(): void
    {
        if (!$this->closed) { $this->ffi->thinkthen_engine_free($this->engine); $this->closed = true; }
    }
    public function __destruct() { $this->close(); }
    private function live(): void { if ($this->closed) throw new LogicException('engine closed'); }
    private function fail(int $code): never
    {
        // All three reads happen immediately, on the same PHP OS thread; copy the message.
        $actual = $this->ffi->thinkthen_error_code($this->engine);
        $retry = $this->ffi->thinkthen_error_retryable($this->engine);
        $raw = $this->ffi->thinkthen_error_message($this->engine);
        $message = is_string($raw) ? $raw : FFI::string($raw);
        $factsRaw = $this->ffi->thinkthen_error_facts_json($this->engine);
        $facts = $factsRaw === null || (!is_string($factsRaw) && FFI::isNull($factsRaw))
            ? null : (is_string($factsRaw) ? $factsRaw : FFI::string($factsRaw));
        if ($code !== 0 && $actual !== $code) throw new LogicException('native error slot does not match the returned code');
        throw new ThinkThenFailure($actual, (bool)$retry, $message, $facts);
    }
    private function checked(string $value): FFI\CData
    {
        if (str_contains($value, "\0")) throw new InvalidArgumentException('interior NUL in C string');
        if (!preg_match('//u', $value)) throw new InvalidArgumentException('invalid UTF-8 in C string');
        $bytes = $this->ffi->new('char[' . (strlen($value) + 1) . ']');
        FFI::memset($bytes, 0, strlen($value) + 1);
        FFI::memcpy($bytes, $value, strlen($value));
        return $bytes;
    }
    private function evidence(string $value): FFI\CData
    {
        $bytes = $this->ffi->new('char[' . (strlen($value) + 1) . ']');
        FFI::memset($bytes, 0, strlen($value) + 1);
        FFI::memcpy($bytes, $value, strlen($value));
        return $bytes;
    }
    public function token(): FFI\CData { $this->live(); return $this->ffi->thinkthen_cancel_token_new(); }
    public function fire(FFI\CData $token): void { $this->live(); $this->ffi->thinkthen_cancel($token); }
    public function freeToken(FFI\CData $token): void { $this->ffi->thinkthen_cancel_token_free($token); }
    private function facts(FFI\CData $pointer, FFI\CData $length): array
    {
        if (FFI::isNull($pointer)) throw new UnexpectedValueException('native facts pointer is null');
        $facts = json_decode(FFI::string($pointer, $length->cdata), true, 512, JSON_THROW_ON_ERROR);
        if (!is_array($facts) || array_is_list($facts)) throw new UnexpectedValueException('native facts must be an object');
        foreach (['records', 'requests_sent', 'cache_answers'] as $key) {
            if (!array_key_exists($key, $facts) || !is_int($facts[$key]) || $facts[$key] < 0)
                throw new UnexpectedValueException('invalid native facts ' . $key);
        }
        if (!array_key_exists('seconds', $facts) || !(is_int($facts['seconds']) || is_float($facts['seconds']))
            || !is_finite((float)$facts['seconds']) || $facts['seconds'] < 0)
            throw new UnexpectedValueException('invalid native facts seconds');
        foreach (['input_tokens', 'output_tokens'] as $key) {
            if (array_key_exists($key, $facts) && (!is_int($facts[$key]) || $facts[$key] < 0))
                throw new UnexpectedValueException('invalid native facts ' . $key);
        }
        if (array_key_exists('model', $facts) && !is_string($facts['model']))
            throw new UnexpectedValueException('invalid native facts model');
        return $facts;
    }
    public function decide(string $question, string $text, int $deadlineMs = -1, ?FFI\CData $token = null): array
    {
        $this->live(); $q = $this->checked($question); $t = $this->evidence($text);
        $answer = $this->ffi->new('thinkthen_answer');
        $facts = $this->ffi->new('char *'); $factsLen = $this->ffi->new('size_t');
        try {
            $code = $this->ffi->thinkthen_decide_with_facts_opts($this->engine, $q, $t, strlen($text), $deadlineMs, $token, FFI::addr($answer), FFI::addr($facts), FFI::addr($factsLen));
            if ($code) $this->fail($code);
            return ['value' => ['outcome' => $answer->outcome, 'probability' => $answer->probability], 'facts' => $this->facts($facts, $factsLen)];
        } finally { $this->ffi->thinkthen_free_string($facts); }
    }
    private function strings(array $texts): array
    {
        $count = count($texts);
        $ptrs = $this->ffi->new('char *[' . max(1, $count) . ']');
        $lens = $this->ffi->new('size_t[' . max(1, $count) . ']');
        $buffers = [];
        foreach (array_values($texts) as $i => $text) {
            if (!is_string($text)) throw new InvalidArgumentException('text must be bytes');
            $buffers[$i] = $this->evidence($text);
            $ptrs[$i] = FFI::cast('char *', $buffers[$i]);
            $lens[$i] = strlen($text);
        }
        return [$ptrs, $lens, $buffers]; // retain buffers until call returns
    }
    public function decideMany(string $question, array $texts, int $deadlineMs = -1, ?FFI\CData $token = null): array
    {
        $this->live(); $q = $this->checked($question); [$ptrs, $lens, $buffers] = $this->strings($texts);
        $count = count($texts); $answers = $this->ffi->new('thinkthen_answer[' . max(1, $count) . ']');
        $facts = $this->ffi->new('char *'); $factsLen = $this->ffi->new('size_t');
        try {
            $code = $this->ffi->thinkthen_decide_many_with_facts_opts($this->engine, $q, $ptrs, $lens, $count, $deadlineMs, $token, $answers, FFI::addr($facts), FFI::addr($factsLen));
            if ($code) $this->fail($code);
            $result = []; for ($i = 0; $i < $count; ++$i) $result[] = ['outcome' => $answers[$i]->outcome, 'probability' => $answers[$i]->probability];
            return ['value' => $result, 'facts' => $this->facts($facts, $factsLen)];
        } finally { $this->ffi->thinkthen_free_string($facts); }
    }
    public function call(string $json, int $deadlineMs = -1, ?FFI\CData $token = null): string
    {
        $this->live(); $request = $this->checked($json);
        $pointer = $this->ffi->thinkthen_call_opts($this->engine, $request, $deadlineMs, $token);
        if ($pointer === null || FFI::isNull($pointer)) $this->fail(0);
        $text = FFI::cast('char *', $pointer);
        try { return FFI::string($text); }
        finally { $this->ffi->thinkthen_free_string($text); }
    }
    public function recognize(string $spec, string $text, int $deadlineMs = -1, ?FFI\CData $token = null): array
    {
        $this->live(); $s = $this->checked($spec); $t = $this->evidence($text);
        $out = $this->ffi->new('char *'); $length = $this->ffi->new('size_t');
        $facts = $this->ffi->new('char *'); $factsLen = $this->ffi->new('size_t');
        try {
            $code = $this->ffi->thinkthen_recognize_with_facts_opts($this->engine, $s, $t, strlen($text), $deadlineMs, $token, FFI::addr($out), FFI::addr($length), FFI::addr($facts), FFI::addr($factsLen));
            if ($code) $this->fail($code);
            if (FFI::isNull($out)) throw new UnexpectedValueException('native result pointer is null');
            return ['value' => FFI::string($out, $length->cdata), 'facts' => $this->facts($facts, $factsLen)];
        } finally { $this->ffi->thinkthen_free_string($out); $this->ffi->thinkthen_free_string($facts); }
    }
    public function relate(string $spec, array $texts, int $deadlineMs = -1, ?FFI\CData $token = null): array
    {
        $this->live(); $s = $this->checked($spec); [$ptrs, $lens, $buffers] = $this->strings($texts);
        $out = $this->ffi->new('char *'); $length = $this->ffi->new('size_t');
        $facts = $this->ffi->new('char *'); $factsLen = $this->ffi->new('size_t');
        try {
            $code = $this->ffi->thinkthen_relate_with_facts_opts($this->engine, $s, $ptrs, $lens, count($texts), $deadlineMs, $token, FFI::addr($out), FFI::addr($length), FFI::addr($facts), FFI::addr($factsLen));
            if ($code) $this->fail($code);
            if (FFI::isNull($out)) throw new UnexpectedValueException('native result pointer is null');
            return ['value' => FFI::string($out, $length->cdata), 'facts' => $this->facts($facts, $factsLen)];
        } finally { $this->ffi->thinkthen_free_string($out); $this->ffi->thinkthen_free_string($facts); }
    }
}
final class ThinkThenFailure extends RuntimeException
{
    private const KINDS = [1 => 'usage', 2 => 'backend', 3 => 'deadline', 4 => 'local', 5 => 'cancelled', 6 => 'defect'];
    public readonly string $kind;
    public function __construct(public readonly int $nativeCode, public readonly bool $retryable, string $message,
                                public readonly ?string $factsJson = null)
    {
        $this->kind = self::KINDS[$nativeCode] ?? 'defect';
        parent::__construct($message, $nativeCode);
    }
}

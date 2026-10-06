<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class Position extends Carrier
{
    public function __construct(
        /** @var Optional<string|null> */
        public readonly Optional $file = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $first = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $last = new Optional(),
        /** @var Optional<list<string>> */
        public readonly Optional $images = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Position", $v, ["file", "first", "last", "images"], []);
        return new self(
            Read::optional($v, "file", "str|null"),
            Read::optional($v, "first", "positive"),
            Read::optional($v, "last", "positive"),
            Read::optional($v, "images", "[str]"),
        );
    }
}

final class Usage extends Carrier
{
    public function __construct(
        public readonly int $input_tokens,
        public readonly int $output_tokens,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Usage", $v, ["input_tokens", "output_tokens"], ["input_tokens", "output_tokens"]);
        return new self(
            Read::field($v, "input_tokens", "uint"),
            Read::field($v, "output_tokens", "uint"),
        );
    }
}

final class ProfileWarning extends Carrier
{
    public function __construct(
        public readonly string $tuned_for,
        public readonly string $running,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ProfileWarning", $v, ["tuned_for", "running"], ["tuned_for", "running"]);
        return new self(
            Read::field($v, "tuned_for", "str"),
            Read::field($v, "running", "str"),
        );
    }
}

final class BatchWarning extends Carrier
{
    public function __construct(
        public readonly Batch $tuned_for,
        public readonly Batch $running,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("BatchWarning", $v, ["tuned_for", "running"], ["tuned_for", "running"]);
        return new self(
            Read::field($v, "tuned_for", "batch"),
            Read::field($v, "running", "batch"),
        );
    }
}

final class Attempt extends Carrier
{
    public function __construct(
        public readonly int $ordinal,
        public readonly Digest $request_sha256,
        public readonly int $wall_ms,
        public readonly OutcomeKind $outcome,
        public readonly SdkRequestId $sdk_request_id,
        /** @var Optional<int> */
        public readonly Optional $status = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $server_ms = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $request_id = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Attempt", $v, ["ordinal", "request_sha256", "wall_ms", "outcome", "sdk_request_id", "status", "server_ms", "request_id"], ["ordinal", "request_sha256", "wall_ms", "outcome", "sdk_request_id"]);
        return new self(
            Read::field($v, "ordinal", "positive"),
            Read::field($v, "request_sha256", "Digest"),
            Read::field($v, "wall_ms", "uint"),
            Read::field($v, "outcome", "outcome"),
            Read::field($v, "sdk_request_id", "SdkRequestId"),
            Read::optional($v, "status", "http_status"),
            Read::optional($v, "server_ms", "uint"),
            Read::optional($v, "request_id", "str"),
        );
    }
}

final class Facts extends Carrier
{
    public function __construct(
        public readonly CallId $call_id,
        public readonly int $records,
        public readonly int $requests_sent,
        public readonly int $cache_answers,
        public readonly float $seconds,
        /** @var Optional<int> */
        public readonly Optional $input_tokens = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $output_tokens = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $estimated_cost_usd = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $command_ms = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Facts", $v, ["call_id", "records", "requests_sent", "cache_answers", "seconds", "input_tokens", "output_tokens", "model", "estimated_cost_usd", "command_ms"], ["call_id", "records", "requests_sent", "cache_answers", "seconds"]);
        return new self(
            Read::field($v, "call_id", "CallId"),
            Read::field($v, "records", "uint"),
            Read::field($v, "requests_sent", "uint"),
            Read::field($v, "cache_answers", "uint"),
            Read::field($v, "seconds", "number"),
            Read::optional($v, "input_tokens", "uint"),
            Read::optional($v, "output_tokens", "uint"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "estimated_cost_usd", "cost"),
            Read::optional($v, "command_ms", "uint"),
        );
    }
}

final class QuestionSource extends Carrier
{
    public function __construct(
        public readonly OriginKind $origin,
        public readonly string $answered_by,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("QuestionSource", $v, ["origin", "answered_by"], ["origin", "answered_by"]);
        return new self(
            Read::field($v, "origin", "origin"),
            Read::field($v, "answered_by", "str"),
        );
    }
}

final class Observed extends Carrier implements Observation
{
    public function __construct(
        public readonly ObservationId $observation_id,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Observed", $v, ["observation_id"], ["observation_id"]);
        return new self(
            Read::field($v, "observation_id", "ObservationId"),
        );
    }
}

final class FailedObservation extends Carrier implements Observation
{
    public function __construct(
        public readonly FailureId $failure_id,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FailedObservation", $v, ["failure_id"], ["failure_id"]);
        return new self(
            Read::field($v, "failure_id", "FailureId"),
        );
    }
}

final class Meta extends Carrier
{
    public function __construct(
        public readonly string $tool,
        public readonly string $url,
        public readonly string $model,
        public readonly int $requests_sent,
        public readonly bool $cached,
        /** @var list<Digest> */
        public readonly array $requests,
        public readonly int $failed_questions,
        public readonly OriginKind|null $origin,
        /** @var list<QuestionSource> */
        public readonly array $question_sources,
        /** @var list<Observation> */
        public readonly array $observations,
        /** @var Optional<Digest> */
        public readonly Optional $question_sha256 = new Optional(),
        /** @var Optional<Digest> */
        public readonly Optional $questions_sha256 = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $answered_by = new Optional(),
        /** @var Optional<Usage> */
        public readonly Optional $usage = new Optional(),
        /** @var Optional<ProfileWarning> */
        public readonly Optional $profile_warning = new Optional(),
        /** @var Optional<Batch> */
        public readonly Optional $batch_setting = new Optional(),
        /** @var Optional<BatchWarning> */
        public readonly Optional $batch_warning = new Optional(),
        /** @var Optional<Digest> */
        public readonly Optional $context_sha256 = new Optional(),
        /** @var Optional<list<Attempt>> */
        public readonly Optional $attempts = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Meta", $v, ["tool", "url", "model", "requests_sent", "cached", "requests", "failed_questions", "origin", "question_sources", "observations", "question_sha256", "questions_sha256", "answered_by", "usage", "profile_warning", "batch_setting", "batch_warning", "context_sha256", "attempts"], ["tool", "url", "model", "requests_sent", "cached", "requests", "failed_questions", "origin", "question_sources", "observations"]);
        return new self(
            Read::field($v, "tool", "str"),
            Read::field($v, "url", "str"),
            Read::field($v, "model", "str"),
            Read::field($v, "requests_sent", "uint"),
            Read::field($v, "cached", "bool"),
            Read::field($v, "requests", "[Digest]"),
            Read::field($v, "failed_questions", "uint"),
            Read::field($v, "origin", "origin|null"),
            Read::field($v, "question_sources", "[QuestionSource]"),
            Read::field($v, "observations", "[Observation]"),
            Read::optional($v, "question_sha256", "Digest"),
            Read::optional($v, "questions_sha256", "Digest"),
            Read::optional($v, "answered_by", "str"),
            Read::optional($v, "usage", "Usage"),
            Read::optional($v, "profile_warning", "ProfileWarning"),
            Read::optional($v, "batch_setting", "batch"),
            Read::optional($v, "batch_warning", "BatchWarning"),
            Read::optional($v, "context_sha256", "Digest"),
            Read::optional($v, "attempts", "[Attempt]"),
        );
    }
}

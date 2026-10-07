<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class CallError extends Carrier
{
    public function __construct(
        public readonly FailureKind $kind,
        public readonly string $message,
        public readonly bool $retryable,
        /** @var Optional<Facts> */
        public readonly Optional $facts = new Optional(),
        /** @var Optional<array> */
        public readonly Optional $attempts = new Optional(),
        /** @var Optional<Stopped> */
        public readonly Optional $stopped = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("CallError", $v, ["kind", "message", "retryable", "facts", "attempts", "stopped"], ["kind", "message", "retryable"]);
        return new self(
            Read::field($v, "kind", "error_kind"),
            Read::field($v, "message", "str"),
            Read::field($v, "retryable", "bool"),
            Read::optional($v, "facts", "Facts"),
            Read::optional($v, "attempts", "[Attempt]"),
            Read::optional($v, "stopped", "Stopped"),
        );
    }
}

final class Stopped extends Carrier
{
    public function __construct(
        public readonly StopCauseKind $cause,
        public readonly bool $retryable,
        /** @var Optional<int> */
        public readonly Optional $at = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $status = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Stopped", $v, ["cause", "retryable", "at", "status"], ["cause", "retryable"]);
        return new self(
            Read::field($v, "cause", "stop_cause"),
            Read::field($v, "retryable", "bool"),
            Read::optional($v, "at", "positive"),
            Read::optional($v, "status", "http_status"),
        );
    }
}

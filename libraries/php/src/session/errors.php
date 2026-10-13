<?php
declare(strict_types=1);
namespace ThinkThen;

class CallFailure extends \RuntimeException
{
    public const KIND = ErrorKind::Defect;
    public readonly ErrorKind $kind;
    public readonly bool $retryable;
    public function __construct(string $message, public readonly ?\ThinkThen\Results\CallError $failure = null,
        public readonly array $results = [], public readonly ?\ThinkThen\Results\SessionPacketTerminal $terminal = null,
        public readonly array $packets = [],
        ?\Throwable $previous = null, bool $retryable = false,
        private readonly ?\ThinkThen\Results\FailureFacts $immediateFacts = null)
    {
        $this->kind = static::KIND;
        $this->retryable = $failure?->error?->retryable ?? $retryable;
        parent::__construct($message, 0, $previous);
    }
    public function facts(): \ThinkThen\Results\Facts|\ThinkThen\Results\FailureFacts|null { return $this->failure?->facts ?? $this->immediateFacts; }
}
final class Cancellation
{
    public bool $cancelled = false;
    public function cancel(): void { $this->cancelled = true; }
}

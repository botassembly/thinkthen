<?php
declare(strict_types=1);
namespace ThinkThen;

class CallFailure extends \RuntimeException
{
    public function __construct(string $message, public readonly ?\ThinkThen\Results\Node $failure = null,
        public readonly array $results = [], public readonly ?\ThinkThen\Results\Node $terminal = null,
        public readonly array $packets = [],
        ?\Throwable $previous = null)
    {
        parent::__construct($message, 0, $previous);
    }
    public function facts(): ?\ThinkThen\Results\Node { return $this->failure?->facts; }
}
final class UsageFailure extends CallFailure {}
final class BackendFailure extends CallFailure {}
final class DeadlineFailure extends CallFailure {}
final class LocalFailure extends CallFailure {}
final class CancelledFailure extends CallFailure {}
final class DefectFailure extends CallFailure {}

final class Cancellation
{
    public bool $cancelled = false;
    public function cancel(): void { $this->cancelled = true; }
}

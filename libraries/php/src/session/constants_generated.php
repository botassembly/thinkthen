<?php
// Generated from the native C header; do not edit.
declare(strict_types=1);
namespace ThinkThen\Session;
const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1 = 1;
const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1 = 4;
const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1 = 2;
const THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1 = 3;
const THINKTHEN_EBACKEND = 2;
const THINKTHEN_ECANCELLED = 5;
const THINKTHEN_EDEADLINE = 3;
const THINKTHEN_EDEFECT = 6;
const THINKTHEN_ELOCAL = 4;
const THINKTHEN_EUSAGE = 1;
const THINKTHEN_SESSION_ACCEPTED_V1 = 0;
const THINKTHEN_SESSION_CLOSED_V1 = 2;
const THINKTHEN_SESSION_END_V1 = 2;
const THINKTHEN_SESSION_FULL_V1 = 1;
const THINKTHEN_SESSION_PENDING_V1 = 1;
const THINKTHEN_SESSION_READY_CANCELLED_V1 = 4;
const THINKTHEN_SESSION_READY_CLOSED_V1 = 2;
const THINKTHEN_SESSION_READY_END_V1 = 3;
const THINKTHEN_SESSION_READY_INPUT_V1 = 1;
const THINKTHEN_SESSION_READY_RESULT_V1 = 0;
const THINKTHEN_SESSION_READY_TIMEOUT_V1 = 5;
const THINKTHEN_SESSION_RESULT_V1 = 0;
const THINKTHEN_SESSION_WAIT_INPUT_V1 = 1;
const THINKTHEN_SESSION_WAIT_RESULT_V1 = 0;
const THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 = 2;
namespace ThinkThen;
enum UsagePersistenceState: int {
case Disabled = 1;
case Failed = 4;
case Pending = 2;
case Written = 3;
}
trait NativeErrors
{
    public static function throwImmediate(int $code, string $message): never
    {
        $class = match ($code) {
            \ThinkThen\Session\THINKTHEN_EBACKEND => BackendFailure::class,
            \ThinkThen\Session\THINKTHEN_ECANCELLED => CancelledFailure::class,
            \ThinkThen\Session\THINKTHEN_EDEADLINE => DeadlineFailure::class,
            \ThinkThen\Session\THINKTHEN_EDEFECT => DefectFailure::class,
            \ThinkThen\Session\THINKTHEN_ELOCAL => LocalFailure::class,
            \ThinkThen\Session\THINKTHEN_EUSAGE => UsageFailure::class,
            default => DefectFailure::class,
        };
        throw new $class($message);
    }
}

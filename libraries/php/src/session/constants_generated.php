<?php
// Generated from the native C header; do not edit.
declare(strict_types=1);
namespace ThinkThen\Internal\Session;
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
enum ErrorKind: int {
    case Backend = 2;
    case Cancelled = 5;
    case Deadline = 3;
    case Defect = 6;
    case Local = 4;
    case Usage = 1;
}
final class BackendFailure extends CallFailure { public const KIND = ErrorKind::Backend; }
final class CancelledFailure extends CallFailure { public const KIND = ErrorKind::Cancelled; }
final class DeadlineFailure extends CallFailure { public const KIND = ErrorKind::Deadline; }
final class DefectFailure extends CallFailure { public const KIND = ErrorKind::Defect; }
final class LocalFailure extends CallFailure { public const KIND = ErrorKind::Local; }
final class UsageFailure extends CallFailure { public const KIND = ErrorKind::Usage; }
namespace ThinkThen\Internal;
final class ErrorFactory
{
    public static function classFor(string $kind): string
    {
        return match ($kind) {
            'backend' => \ThinkThen\BackendFailure::class,
            'cancelled' => \ThinkThen\CancelledFailure::class,
            'deadline' => \ThinkThen\DeadlineFailure::class,
            'defect' => \ThinkThen\DefectFailure::class,
            'local' => \ThinkThen\LocalFailure::class,
            'usage' => \ThinkThen\UsageFailure::class,
            default => \ThinkThen\DefectFailure::class,
        };
    }
    public static function raise(\FFI $ffi, int $code, bool $session): never
    {
        $view = $session ? $ffi->thinkthen_session_error_view() : $ffi->thinkthen_error_view(null);
        $retryable = false; $facts = null;
        if ($view !== null && !\FFI::isNull($view)) {
            $code = $view->code;
            $retryable = (bool)$view->retryable;
            $message = $view->message->len === 0 ? '' : \FFI::string($view->message->data, $view->message->len);
            if (!\FFI::isNull($view->facts)) $facts = Decoder::failureFacts($view->facts);
        } else {
            $text = $session ? $ffi->thinkthen_session_error_message() : $ffi->thinkthen_error_message(null);
            $message = is_string($text) ? $text : \FFI::string($text);
        }
        $class = match ($code) {
            2 => \ThinkThen\BackendFailure::class,
            5 => \ThinkThen\CancelledFailure::class,
            3 => \ThinkThen\DeadlineFailure::class,
            6 => \ThinkThen\DefectFailure::class,
            4 => \ThinkThen\LocalFailure::class,
            1 => \ThinkThen\UsageFailure::class,
            default => \ThinkThen\DefectFailure::class,
        };
        throw new $class($message, retryable: $retryable, immediateFacts: $facts);
    }
}

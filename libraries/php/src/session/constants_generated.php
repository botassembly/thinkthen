<?php
// Generated from the native C header; do not edit.
declare(strict_types=1);
namespace ThinkThen\Session;
const REQUEST_VERSION = 'thinkthen.request/1';
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
const THINKTHEN_SESSION_RESULT_V1 = 0;
namespace ThinkThen;
enum UsagePersistenceState: int {
case Disabled = 1;
case Failed = 4;
case Pending = 2;
case Written = 3;
}

// Generated from the engine registry. Do not edit.
package thinkthen

/*
#include "loader.h"
*/
import "C"

const (
	// CodeUsageThreshold identifies the engine's usage.threshold error.
	CodeUsageThreshold Code = C.THINKTHEN_CODE_USAGE_THRESHOLD
	// CodeUsageQuestion identifies the engine's usage.question error.
	CodeUsageQuestion Code = C.THINKTHEN_CODE_USAGE_QUESTION
	// CodeUsageName identifies the engine's usage.name error.
	CodeUsageName Code = C.THINKTHEN_CODE_USAGE_NAME
	// CodeUsageSettings identifies the engine's usage.settings error.
	CodeUsageSettings Code = C.THINKTHEN_CODE_USAGE_SETTINGS
	// CodeUsageBackend identifies the engine's usage.backend error.
	CodeUsageBackend Code = C.THINKTHEN_CODE_USAGE_BACKEND
	// CodeUsageURL identifies the engine's usage.url error.
	CodeUsageURL Code = C.THINKTHEN_CODE_USAGE_URL
	// CodeUsageKeyMissing identifies the engine's usage.key-missing error.
	CodeUsageKeyMissing Code = C.THINKTHEN_CODE_USAGE_KEY_MISSING
	// CodeUsageBatch identifies the engine's usage.batch error.
	CodeUsageBatch Code = C.THINKTHEN_CODE_USAGE_BATCH
	// CodeUsagePointer identifies the engine's usage.pointer error.
	CodeUsagePointer Code = C.THINKTHEN_CODE_USAGE_POINTER
	// CodeUsageSQLPolicy identifies the engine's usage.sql-policy error.
	CodeUsageSQLPolicy Code = C.THINKTHEN_CODE_USAGE_SQL_POLICY
	// CodeUsageSQLSelection identifies the engine's usage.sql-selection error.
	CodeUsageSQLSelection Code = C.THINKTHEN_CODE_USAGE_SQL_SELECTION
	// CodeUsageABI identifies the engine's usage.abi error.
	CodeUsageABI Code = C.THINKTHEN_CODE_USAGE_ABI
	// CodeUsageClosed identifies the engine's usage.closed error.
	CodeUsageClosed Code = C.THINKTHEN_CODE_USAGE_CLOSED
	// CodeUsageFunction identifies the engine's usage.function error.
	CodeUsageFunction Code = C.THINKTHEN_CODE_USAGE_FUNCTION
	// CodeUsageRecordKind identifies the engine's usage.record-kind error.
	CodeUsageRecordKind Code = C.THINKTHEN_CODE_USAGE_RECORD_KIND
	// CodeUsageOffsets identifies the engine's usage.offsets error.
	CodeUsageOffsets Code = C.THINKTHEN_CODE_USAGE_OFFSETS
	// CodeUsageArgument identifies the engine's usage.argument error.
	CodeUsageArgument Code = C.THINKTHEN_CODE_USAGE_ARGUMENT
	// CodeUsageOption identifies the engine's usage.option error.
	CodeUsageOption Code = C.THINKTHEN_CODE_USAGE_OPTION
	// CodeRecordJSON identifies the engine's record.json error.
	CodeRecordJSON Code = C.THINKTHEN_CODE_RECORD_JSON
	// CodeRecordMissing identifies the engine's record.missing error.
	CodeRecordMissing Code = C.THINKTHEN_CODE_RECORD_MISSING
	// CodeRecordTooLarge identifies the engine's record.too-large error.
	CodeRecordTooLarge Code = C.THINKTHEN_CODE_RECORD_TOO_LARGE
	// CodeRecordInvalid identifies the engine's record.invalid error.
	CodeRecordInvalid Code = C.THINKTHEN_CODE_RECORD_INVALID
	// CodeRecordEmpty identifies the engine's record.empty error.
	CodeRecordEmpty Code = C.THINKTHEN_CODE_RECORD_EMPTY
	// CodeBackendTransport identifies the engine's backend.transport error.
	CodeBackendTransport Code = C.THINKTHEN_CODE_BACKEND_TRANSPORT
	// CodeBackendStatus identifies the engine's backend.status error.
	CodeBackendStatus Code = C.THINKTHEN_CODE_BACKEND_STATUS
	// CodeBackendReply identifies the engine's backend.reply error.
	CodeBackendReply Code = C.THINKTHEN_CODE_BACKEND_REPLY
	// CodeLocalCancelled identifies the engine's local.cancelled error.
	CodeLocalCancelled Code = C.THINKTHEN_CODE_LOCAL_CANCELLED
	// CodeLocalIO identifies the engine's local.io error.
	CodeLocalIO Code = C.THINKTHEN_CODE_LOCAL_IO
	// CodeLocalToolbox identifies the engine's local.toolbox error.
	CodeLocalToolbox Code = C.THINKTHEN_CODE_LOCAL_TOOLBOX
	// CodeInternalBug identifies the engine's internal.bug error.
	CodeInternalBug Code = C.THINKTHEN_CODE_INTERNAL_BUG
)

var codeNames = map[Code]string{
	CodeUsageThreshold:    "usage.threshold",
	CodeUsageQuestion:     "usage.question",
	CodeUsageName:         "usage.name",
	CodeUsageSettings:     "usage.settings",
	CodeUsageBackend:      "usage.backend",
	CodeUsageURL:          "usage.url",
	CodeUsageKeyMissing:   "usage.key-missing",
	CodeUsageBatch:        "usage.batch",
	CodeUsagePointer:      "usage.pointer",
	CodeUsageSQLPolicy:    "usage.sql-policy",
	CodeUsageSQLSelection: "usage.sql-selection",
	CodeUsageABI:          "usage.abi",
	CodeUsageClosed:       "usage.closed",
	CodeUsageFunction:     "usage.function",
	CodeUsageRecordKind:   "usage.record-kind",
	CodeUsageOffsets:      "usage.offsets",
	CodeUsageArgument:     "usage.argument",
	CodeUsageOption:       "usage.option",
	CodeRecordJSON:        "record.json",
	CodeRecordMissing:     "record.missing",
	CodeRecordTooLarge:    "record.too-large",
	CodeRecordInvalid:     "record.invalid",
	CodeRecordEmpty:       "record.empty",
	CodeBackendTransport:  "backend.transport",
	CodeBackendStatus:     "backend.status",
	CodeBackendReply:      "backend.reply",
	CodeLocalCancelled:    "local.cancelled",
	CodeLocalIO:           "local.io",
	CodeLocalToolbox:      "local.toolbox",
	CodeInternalBug:       "internal.bug",
}

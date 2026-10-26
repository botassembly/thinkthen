"""Generated from the engine error registry during package assembly."""
from enum import Enum

class ErrorCode(str, Enum):
    USAGE_THRESHOLD = "usage.threshold"
    USAGE_QUESTION = "usage.question"
    USAGE_NAME = "usage.name"
    USAGE_SETTINGS = "usage.settings"
    USAGE_BACKEND = "usage.backend"
    USAGE_URL = "usage.url"
    USAGE_KEY_MISSING = "usage.key-missing"
    USAGE_BATCH = "usage.batch"
    USAGE_POINTER = "usage.pointer"
    USAGE_SQL_POLICY = "usage.sql-policy"
    USAGE_SQL_SELECTION = "usage.sql-selection"
    USAGE_ABI = "usage.abi"
    RECORD_JSON = "record.json"
    RECORD_MISSING = "record.missing"
    RECORD_TOO_LARGE = "record.too-large"
    RECORD_INVALID = "record.invalid"
    BACKEND_TRANSPORT = "backend.transport"
    BACKEND_STATUS = "backend.status"
    BACKEND_REPLY = "backend.reply"
    LOCAL_CANCELLED = "local.cancelled"
    LOCAL_IO = "local.io"
    LOCAL_TOOLBOX = "local.toolbox"
    INTERNAL_BUG = "internal.bug"

__all__ = ["ErrorCode"]

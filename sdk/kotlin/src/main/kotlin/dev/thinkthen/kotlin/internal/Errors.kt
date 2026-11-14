package dev.thinkthen.kotlin.internal

import dev.thinkthen.internal.DriverException
import dev.thinkthen.kotlin.*
import kotlinx.coroutines.CancellationException

/** Preserve shared codes/messages while assigning the host exception family. */
internal inline fun <T> translateNative(setup: Boolean, operation: () -> T): T = try { operation() }
catch (error: DriverException) {
    val native = error.error()
    val code = ErrorCode.fromWire(native.name())
    val cause = when {
        code == ErrorCode.USAGE_KEY_MISSING || code == ErrorCode.USAGE_ABI || (setup && code == ErrorCode.LOCAL_IO) ->
            SetupException(code, native.message(), native.httpStatus(), error)
        code.family == ErrorFamily.USAGE -> UsageException(code, native.message(), native.httpStatus(), error)
        code == ErrorCode.LOCAL_CANCELLED -> ThinkThenException(code, native.message(), native.httpStatus(), error)
        else -> InternalException(code, native.message(), native.httpStatus(), error)
    }
    if (code == ErrorCode.LOCAL_CANCELLED) throw CancellationException(native.message(), cause)
    throw cause
}

package dev.thinkthen.kotlin

/** An immutable cumulative usage snapshot for one explicit client. */
public data class Usage(
    /** Remote sends started, including retries. */ public val attempts: ULong,
    /** Fully received bounded response bodies. */ public val responses: ULong,
    /** Retries following response status. */ public val statusRetries: ULong,
    /** Retries following connection failures. */ public val connectionRetries: ULong,
    /** Encoded body bytes attempted. */ public val requestBodyBytes: ULong,
    /** Attempts still in progress at this snapshot. */ public val activeAttempts: ULong,
    /** Engine workers still active, including those awaiting input. */ public val activeCalls: ULong,
    /** Provider-reported input usage. */ public val inputTokens: TokenUsage,
    /** Provider-reported output usage. */ public val outputTokens: TokenUsage,
)

/** Provider-reported subtotal and count of finished attempts with unknown usage. */
public data class TokenUsage(
    /** Known subtotal, or null on overflow. */ public val reported: ULong?,
    /** Finished attempts whose token usage is unknown. */ public val unknownAttempts: ULong,
)

internal fun dev.thinkthen.Usage.kotlinUsage(): Usage = Usage(
    attempts().toString().toULong(), responses().toString().toULong(),
    statusRetries().toString().toULong(), connectionRetries().toString().toULong(),
    requestBodyBytes().toString().toULong(), activeAttempts().toString().toULong(), activeCalls().toString().toULong(),
    TokenUsage(inputTokens().reported()?.toString()?.toULong(), inputTokens().unknownAttempts().toString().toULong()),
    TokenUsage(outputTokens().reported()?.toString()?.toULong(), outputTokens().unknownAttempts().toString().toULong()),
)

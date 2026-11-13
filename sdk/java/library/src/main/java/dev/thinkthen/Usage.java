package dev.thinkthen;

import java.math.BigInteger;
import org.jspecify.annotations.Nullable;

/** An immutable snapshot of cumulative remote usage for one explicit client. */
public record Usage(BigInteger attempts, BigInteger responses, BigInteger statusRetries,
                    BigInteger connectionRetries, BigInteger requestBodyBytes, BigInteger activeAttempts, BigInteger activeCalls,
                    Tokens inputTokens, Tokens outputTokens) {
    /** Provider-reported subtotal; null means overflow, unknown attempts remain separate. */
    public record Tokens(@Nullable BigInteger reported, BigInteger unknownAttempts) {}
}

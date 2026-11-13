namespace ThinkThen;

/// <summary>Provider-reported tokens and finished attempts with unavailable usage.</summary>
/// <param name="Reported">Known token sum, or null on overflow.</param>
/// <param name="UnknownAttempts">Finished attempts without reported usage.</param>
public sealed record TokenUsage(ulong? Reported, ulong UnknownAttempts);

/// <summary>An immutable cumulative snapshot for one explicit client lifetime.</summary>
/// <param name="Attempts">Started send attempts.</param>
/// <param name="Responses">Fully received bounded response bodies.</param>
/// <param name="StatusRetries">Status-retry attempts.</param>
/// <param name="ConnectionRetries">Connection-retry attempts.</param>
/// <param name="RequestBodyBytes">Attempted encoded request bytes.</param>
/// <param name="ActiveAttempts">Attempts not yet finished.</param>
/// <param name="ActiveCalls">Engine workers not yet settled.</param>
/// <param name="InputTokens">Reported input usage.</param>
/// <param name="OutputTokens">Reported output usage.</param>
public sealed record Usage(ulong Attempts, ulong Responses, ulong StatusRetries,
    ulong ConnectionRetries, ulong RequestBodyBytes, ulong ActiveAttempts, ulong ActiveCalls,
    TokenUsage InputTokens, TokenUsage OutputTokens);

# Ticket 0399 saved-response regression

The OpenRouter fixture retains the model, provider, answer, token counts and cost saved for experiment 0003's bare yes-or-no probe on 2026-10-02. The experiment saved those values as prose, rather than retaining a complete raw response envelope. This JSON reconstructs the documented fields. Its `id` is an explicitly synthetic fixture-only substitution; it is not a captured provider request identifier. The fixture contains no header or credential.

The outside-in regression imports this saved envelope through the existing recording converter and reads the converted fixture with `--replay`. It compares the answer against the existing System One decide response. Both replays send zero requests. This establishes offline decoder compatibility; it claims no new successful provider check or measured cost.

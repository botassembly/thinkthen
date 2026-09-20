# ADR 0025: DNS host text is lowercase in resolved URLs

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

The backend parser already writes a scheme in lowercase but keeps host case. DNS host names compare without regard to ASCII case, so `https://EXAMPLE.COM/v1` and `https://example.com/v1` reach the same service while producing different result URLs and recording digests. The loopback rule also accepts `LOCALHOST` without regard to case, then preserves that spelling in the digest.

Ticket 0019 deliberately kept host case to avoid changing older mixed-case recording digests. The first hands-on test showed that this leaves equivalent requests in separate recording entries. The repository currently commits 1,059 recording entries. Their URLs have only two already canonical forms: 1,057 use `https://api.typesafe.ai/v1/systemone`, and two use `http://127.0.0.1:8721/v1/systemone`.

## Decision

- The resolved URL lowercases ASCII letters in an unbracketed host. Equivalent DNS host-case spellings therefore produce one URL and one recording digest. This overturns ticket 0019's choice to preserve host case.
- Punycode labels are ASCII host text and become lowercase. Non-ASCII bytes are unchanged; the parser adds no Unicode case folding or IDNA conversion.
- A percent escape in the host is copied exactly and is not decoded. Literal ASCII letters around it become lowercase. The parser adds no percent-encoding canonicalization.
- A bracketed IPv6 literal is preserved exactly, including hexadecimal case. IPv4 text, the already lowercase scheme, the port spelling, and the path are preserved. Existing trimming and endpoint joining stay unchanged.
- The accepted and refused address sets stay unchanged. Normalization happens after the current scheme, authority, port, query, fragment, and clear-text loopback checks have accepted the base.

## Consequences

Plans, result metadata, recording entries, and digests use the lowercase DNS host. Host-case variants share a cache and replay entry. Every committed recording keeps its URL and filename because every committed host is already canonical.

An external recording made with a mixed-case host has an old digest and stored URL. This change provides no legacy lookup or migration, so that entry misses until the user records again or updates both its URL and filename consistently. The repository has no such committed entry. Keeping two lookup identities would preserve the duplicate the decision removes and would widen this repair beyond its smallest useful outcome.

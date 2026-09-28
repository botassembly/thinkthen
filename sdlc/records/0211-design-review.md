# 0211 design review handoff

Status: first independent review of `fdb4e6f84857b5e9ef858770acc467f359733b6f` returned corrections; this revised draft awaits the same reviewer's follow-up. No design ACCEPT result or runtime authority is claimed. Source base: main `5d15138e`, packaged ureq 3.4.2, accepted 0155 proof. Register 118 remains open.

## Chosen proposal and review boundary

The coordinator selected a named PEM CA bundle within register 118's accepted named-bundle-or-platform-verifier outcome. Unset retains Mozilla roots; explicit `THINKTHEN_CA_BUNDLE` or Rust `EngineBuilder::ca_bundle` **replaces** roots for that engine. The first reach is process environment plus Rust builder, without native per-instance or SQL-session settings. Explicitly selected bundles validate at construction even for offline replay, so an unreadable named file can stop an otherwise local replay. These are concrete design choices for review, not an implemented setting. Ian can overturn the recommendation. Do not broaden accepted 0149/0157 or add dependencies.

The first review found that ureq's `parse_pem` skips unsupported item types. The corrected ticket proposes a bounded certificate-only PEM label and delimiter policy before ureq decoding: matched `CERTIFICATE` blocks and ASCII whitespace only, with all other labels, unmatched delimiters and text refused as `Usage`. This is a lexical guard, not a second certificate decoder. Parser errors, empty/oversized/too-many-certificate input and unreadable files retain the ticket's `Usage`/`Local` mapping. Syntactically valid DER that rustls cannot use may still fail at the TLS handshake with 0155's fixed certificate diagnostic. Review whether this is a coherent small boundary and whether the proposed proof covers parser skip behavior.

The first review also exposed key timing. `public/settings.rs::EngineBuilder::from_env` reads `KEY_VAR` into `Secret` before `build`, so a later bundle setter/build cannot promise validation before the first environment key read. The corrected contract promises validation before first key **use/header/send** for library callers. CLI construction can and must validate before its existing late key accessor, with a fault-ordering check that directly observes which refusal wins. A zero HTTP request count alone cannot prove no environment read. This correction does not redesign key lifetime or change pending 0210.

## Follow-up review questions

1. Does the certificate-only lexical boundary reliably reject unsupported/private-key labels that `parse_pem` would skip, while using ureq for decoding and preserving safe `Usage`/`Local` diagnostics?
2. Is explicit-bundle validation, including a replay-only caller whose named file is missing, stated plainly and proved at the correct construction boundary?
3. Does the key-timing distinction match the real CLI and public builder paths, with no false claim about library `from_env` key capture?
4. Does the focused local CA/localhost-leaf matrix prove a genuine HTTPS request and fixed answer on trust success, and TLS failure with zero HTTP requests for default, wrong CA and wrong hostname? Keep 0155's accepted `InvalidData` limit distinct.
5. Are root snapshots preserved across fork/model state and all `facade::Settings` and direct `Client::new` initializers covered before future runtime claims? Near-500-line files and 0172/0201 held files remain excluded from this design claim.

No ADR 0088 text is drafted. No runtime code, build, test, certificate generation, provider call or key inspection occurred in this correction pass. The reviewer should return concrete corrections or a design-quality assessment; no register item closes from a design.

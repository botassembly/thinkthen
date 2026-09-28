# 0242 named-folder warning code review

Status: **ACCEPT** at `2e3e32c6263f27b35e2dbce33b2e7a232eaf842d` from a fresh read-only Sol Medium reviewer. The reviewer traced folder selection and the shared asking boundary, fixed warning text/order, quiet and output failures, Unix owner/write-bit policy and explicit default/absent/non-Unix limits. The implementation changes no library output or storage protocol. It reuses folder selection, output handling and existing test helpers. The measured 210-line growth is justified in the build record.

The reviewer independently ran the three compiled `cache_trust` cases on the exact candidate under its lane lock; all passed. Existing private-cache, identity and owner-predicate evidence remains valid. The coordinator integrated unchanged runtime and the exact recording/security additions reviewed in the build record; the documentation cleanup will preserve them when it merges current main.

## Original closure criterion

The reviewer separately confirmed that the original issue requires two writer-authority pages and a decision on the writable-folder check. Keyed integrity was explicitly optional, including in experiment 283 finding 40. A later status sentence had expanded scope by calling it required work. With the reviewed warning and page additions landed, register 40 and the issue may close. Advisory Unix metadata does not authenticate entries or cover ACLs, ancestor changes, concurrent edits, non-Unix permissions or unsolicited library advisories. Those limits and optional signing remain explicit.

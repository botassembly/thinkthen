

PM amendment, 2026-10-13. Remaining work. Close 0561 now. Exports fell from 111 to 44 and the header from 7,237 to 3,698 lines. The 0562 generator lane deletes `thinkthen_request_plan_named_value`, which no library calls, or names its caller, and regenerates the header. The candidate record lists `nm -D --defined-only` for `libthinkthen.so` with one caller per export. The 1,982-line C ratchet in 0568 replaces this ticket's 2,700-line target. The coordinator adds a superseded line citing the no-compatibility ruling to ADRs 0101, 0106, 0107 and 0129.

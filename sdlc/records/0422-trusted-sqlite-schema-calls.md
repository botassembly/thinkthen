# 0422: Permit explicitly trusted SQLite schema calls

A fresh connection can load sqlite3_thinkthen_trusted_init for reviewed views and ingestion triggers. Default loading retains restrictions on main and attached schema calls. Trusted loading respects trusted_schema OFF there; controls remain DIRECTONLY and host authorizers stay intact. Caller-created TEMP objects remain caller SQL in both modes, including OFF.

The whole High review found an overbroad TEMP claim in docs and test coverage. Its correction passed a fresh focused High review and ten native fixtures, including pinned SQLite 3.50 TEMP calls with zero sends. Existing host and conformance checks, policy, ratchets and the public site build passed. Full tests and lint run on the landing commit. SQLite kinds syntax is documented; JSON array text remains a literal custom kind.

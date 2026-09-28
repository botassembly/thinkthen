# 0228 code review

Fresh independent Sol High review accepted `10ae3238`. High review covered admission ordering and persistent first-writer races. The two probes create no state or checked flag; the existing gate remains the sole marker authority before send. A present or malformed marker and digest-shaped legacy entry still enter the gate before key lookup. Strict replay and bound hits stay keyless. The early key is consumed once; a competing writer observed after a failed key leads through the gate and entry lock to the existing mismatch or matching-hit result.

The reviewer checked the 0065, ADR 0035 and recording-page amendments, the new missing-key folder table, retained concurrency and replay cases, and the inherited Call.value test-consumer and unused-import correction. Existing assertions remain intact. The recorded focused tests and strict lint suffice for this change; no redundant test run was needed.

Register 10 remains open for its broader admitted-but-unsent, status and unbind criteria. This ticket completion does not close that work-plan row. Root integration changes completion records and claims only; product source matches the accepted candidate.

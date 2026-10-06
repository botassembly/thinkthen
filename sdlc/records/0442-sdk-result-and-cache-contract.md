# 0442: SDK result and cache contract

The reviewed result/2 contract defines all ten result shapes, separate call/request/observation/answer identities, truthful origins and models, bounded cache instructions, versioned keys and offline migration. Runtime adoption remains with 0443–0445, 0450 and the native and host owners.

Fresh High read-only review: ACCEPT. Focused settings, schema examples, links and zero-observation relation checks passed. Full tests and lint run on the landing commit.

## What the build taught us

Legacy identity must use existing validated metadata before conversion creates synthetic fields. An aggregate with no observations reports null origin and no answered model. Neither case may invent provenance. Bare and C compatibility projections stay intact.

# llama.cpp decisions require a native route and model

Status: open.

Kind: debt
Debt: 040
Severity: medium
Pay when: The supported llama.cpp release and selected model serve System One without setup restrictions

## Problem

llama.cpp decisions require a native route and model

Found at experiment 0030 and ticket 0421 while working a landed ticket.
Milestone: later

Experiments 0004 and 0030 establish native decision-route setup constraints. Use the supported source revision and a decision-capable GGUF. A chat-compatible server or model can return 404 or 501 for /v1/systemone; that is not evidence of model quality. Prompts must fit model context and the physical batch. The supported Kev4 example used 4096 context and 4096 physical batch, with source a7b94df2c616bc1f62a73b964b4a71cb0dcc488e. Ticket 0421 documents alias local instead of relying on a private model-file path.

Primary upstream source: https://github.com/ggml-org/llama.cpp/blob/a7b94df2c616bc1f62a73b964b4a71cb0dcc488e/tools/server/README.md . Keep each runtime limit here rather than making a new current-release ticket per model. No upstream issue number is claimed. The initial Kev4 check passed all ten functions and four rich probes. Ian later authorized the v0.6.0 release at d81235049384534c167caea52b85a694f6103d14 with Clef-Flash Q4_K_M revision 4a7a08c09bc63baf043b62b5ba89dd67a0357d95. That M5 check also passed all ten functions and four rich probes, exit 0 with no warnings. The setup keeps 4096 context, batch and physical batch, and alias local. The earlier pin remains recorded as the fallback if a later release breaks the native route. No runtime defect was observed in this check. Image support in ThinkThen awaits experiment 0034's design.

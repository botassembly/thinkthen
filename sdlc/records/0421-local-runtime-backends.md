# 0421: Run llama.cpp and MLX through the ten functions

Built-in llamacpp and mlx now resolve through the shared engine, so CLI, Rust, bindings and SQL use the same local defaults. The setup page names pinned decision-capable runtimes, model aliases and limits. MLX uses the existing text-description workaround; the three runtime issues retain their actual loader and encoder limitations.

The existing backend check retains its rich probes and runs all ten production functions. Plans label prepared requests before refusal splits and retries. Without profile splitting the admitted inputs can produce 19 original requests or 76 attempts. True body transport failures stop later functions; oversized replies remain local failures.

One fresh whole High review found those two behavior defects. Their corrections passed the focused regressions and a fresh High fix review. Strict Clippy, policy, named backend/C cases and the full site build passed. Full tests and lint passed on the initial landing at 622a77957.

Each runtime received one real, keyless check on M5 through the reviewed Linux CLI and owned loopback tunnels. llama.cpp source a7b94df2 with Kev-4B-Q4_K_M revision d924f2e2 and alias local answered all ten functions, exit 0, no warnings. Strands source 75c9fd32 with checkpoint bb282d78 and its pinned Qwen base, Python 3.13.13 and MLX 0.32.3 answered all ten, exit 0, with three expected description warnings. These show compatibility for those setups, not accuracy or model ranking. Owned servers, tunnels, builds, weights and scratch were removed; borrowed files stayed unchanged.

Ian's later v0.6.0/Clef instruction adds a second authorized llama.cpp setup check. Release commit d81235049384534c167caea52b85a694f6103d14 with Clef-Flash Q4_K_M revision 4a7a08c09bc63baf043b62b5ba89dd67a0357d95 passed four rich probes and all ten functions on M5, exit 0, no warnings. The 6.49 GB model was selected by setup size and matched its prior public pin. The published setup names Apple silicon, alias local and 4096 context/batch/physical batch. Owned server, tunnel and scratch were removed. The earlier successful Kev pin remains above; no image request or further MLX check ran.

The v0.6.0 setup documentation passed one fresh source review and the full public site build. Full tests and lint run on its landing commit.

Final specification replay found the old four-probe-only plan example still expected nine lines and an unprefixed summary. It now reads the rich-probes summary from the documented twenty-line plan; the four recorded request bodies stay unchanged. Experiment 0035 also resolved a local SQL demo configuration: a 4,244-token prompt exceeded physical batch 2,048 despite context 32,768. Physical batch/microbatch 8,192 and an allowance of three attempts completed the unchanged SQL in one attempt. The local setup now distinguishes these controls and prepared requests from later split attempts. No product guard, model call or verified setup pin changed.

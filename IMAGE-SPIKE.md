# Experiment 0034 CLI image spike

This branch adds one explicit JPEG or PNG attachment to a scalar `decide`, `choose`, or `score` call. It is a branch-only demo. The core reads no files. The CLI reads a regular file, bounds it to 32,768 bytes, and checks its signature and container structure. The final encoded request is bounded to 65,536 bytes. Pixel decoding, PNG CRC validation, resizing, folders, record framing, windows, multiple images, and other judging commands are deferred.

Use an explicit `--backend liquid`, `--backend openrouter`, or `--backend llamacpp`. Keep text evidence on standard input or in one regular `--input` file. For example:

```sh
printf 'Assess the supplied image.' | target/debug/thinkthen decide 'Are white clouds visible on the round object shown?' --image image.jpg --backend llamacpp --model local --url http://127.0.0.1:8080/v1 --record recordings/decide --max-retries 0
printf 'Assess the supplied image.' | target/debug/thinkthen choose 'What best describes the main object shown?' planet vehicle building --image image.jpg --backend liquid --model d1 --record recordings/choose --max-retries 0
printf 'Assess the supplied image.' | target/debug/thinkthen score "How much of the round object's outline is visible?" 'No round outline is visible' 'Only part of the outline is visible' 'Nearly the entire outline is visible' --image image.jpg --backend openrouter --model cloudflare/clef --record recordings/score --max-retries 0
```

Hosted examples require the existing authorized live helper and backend credentials. The builder performed no model calls. Loopback fixtures establish serialization and decoding, not live model vision support. The runtime worker establishes that support separately.

The adapter writes top-level `images` containing one base64 data URL. The pinned [llama.cpp v0.6.0 implementation](https://github.com/ggml-org/llama.cpp/blob/v0.6.0/tools/server/server-decision.cpp) accepts that form in `decision_load_image` and `parse_state`. Liquid and OpenRouter use the confirmed common shape supplied by the experiment plan. Live verification belongs to the experiment evidence.

Image question keys use the versioned `thinkthen.images/data-url/1` domain and length-framed adapter, posting address, model, text state, wire question, and canonical image array. Shared state stores the wire state and image array under a private versioned prefix. Its digest separates image packing groups. Text requests and text keys keep their original paths. Filenames never enter model evidence or keys.

`--record` stores answers through the existing store. Image live calls also save actual request and response JSON bodies under `RECORDING/exchanges/<digest>.json`, using the existing exchange schema without headers. Cache hits create no exchange. HTTP error bodies remain omitted. `--replay RECORDING` reads the answers with no credentials or sends and requires identical image bytes and media. `cache convert RECORDING` exports the ordinary portable `thinkthen.jsonl` answer fixture; keep that fixture and `exchanges/` as evidence. Older binaries cannot replay image entries. No text entries require migration.

No dependencies were added. Focused CLI tests cover the three verbs, the three explicit backends, PNG and JPEG transport, unchanged text bytes and keys, changed image cache misses, relocated-byte replay, actual exchange capture, and invalid input with no sends. Pure tests cover media identity and packing boundaries. Full landing and release gates are outside this spike.

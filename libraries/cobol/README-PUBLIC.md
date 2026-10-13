# ThinkThen for COBOL

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-cobol-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

This excerpt reads one typed boolean; the packaged `examples/session.cob` includes construction, session calls and cleanup.

```cobol
*> After receiving a decision row, retain packet-owner during this read.
set address of tt-n-complete-session-packet-v1 to packet-view
set address of tt-n-complete-session-packet-decide-row-v1
   to v-data-decide-row of tt-n-complete-session-packet-v1
set address of tt-n-complete-atomic-decide-value-v1
   to v-value of tt-n-complete-session-packet-decide-row-v1
if v-value-presence of tt-n-complete-atomic-decide-value-v1 =
   tt-n-complete-presence-value-v1-k
   set address of tt-n-complete-decide-value-v1
      to v-value-value of tt-n-complete-atomic-decide-value-v1
   set address of tt-n-complete-json-v1
      to v-value of tt-n-complete-decide-value-v1
   if v-kind of tt-n-complete-json-v1 = tt-n-complete-json-boolean-v1-k
      display v-data-boolean of tt-n-complete-json-v1
   end-if
end-if
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

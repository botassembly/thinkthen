# ThinkThen for C

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-c-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

This fragment reads one typed answer; the [complete example](https://github.com/botassembly/thinkthen/blob/main/libraries/c/examples/session.c) handles setup and cleanup.

```c
/* After starting a decision session, read a row while its owner is alive. */
const thinkthen_complete_session_packet_v1 *packet = NULL;
if (thinkthen_session_result_view(owner, &packet) != THINKTHEN_OK) return 1;
if (packet->kind == THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
    const thinkthen_complete_atomic_decide_value_v1 *answer = packet->data.decide_row->value;
    if (answer->value.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
        const thinkthen_complete_json_v1 *value = answer->value.value->value;
        if (value->kind == THINKTHEN_COMPLETE_JSON_BOOLEAN_V1)
            printf("%s\n", value->data.boolean ? "true" : "false");
    }
}
thinkthen_session_result_free(owner);
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

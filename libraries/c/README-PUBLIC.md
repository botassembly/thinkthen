# ThinkThen for C

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

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

# ThinkThen for C

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Download the matching archive from the [release downloads](https://github.com/botassembly/thinkthen/releases), then unpack it: `tar -xzf thinkthen-c-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform.
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```c
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>
int main(void) { unsigned answers = 0;
    const char *requests[] = {"{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"decide\",\"question\":{\"kind\":\"text\",\"text\":\"Does the customer ask for a refund?\"},\"input\":{\"kind\":\"records\",\"items\":[{\"original\":{\"kind\":\"text\",\"text\":\"Please refund my order. It arrived broken.\"}},{\"original\":{\"kind\":\"text\",\"text\":\"Thanks for the quick help yesterday!\"}},{\"original\":{\"kind\":\"text\",\"text\":\"I want to send this back.\"}}]}}}",
        "{\"schema\":\"thinkthen.request/1\",\"call\":{\"function\":\"choose\",\"question\":{\"kind\":\"definition\",\"value\":{\"choose\":\"Which team owns this?\",\"options\":{\"billing\":\"Invoices, fees, and refunds.\",\"shipping\":\"Parcels and delivery.\",\"account\":\"Logins and passwords.\"}}},\"input\":{\"kind\":\"records\",\"items\":[{\"original\":{\"kind\":\"text\",\"text\":\"Please refund the extra fee on my invoice.\"}},{\"original\":{\"kind\":\"text\",\"text\":\"My parcel went to the wrong address.\"}},{\"original\":{\"kind\":\"text\",\"text\":\"I cannot reset my password.\"}}]}}}"};
    thinkthen_engine *engine = thinkthen_engine_new(); if (!engine) return 1;
    for (unsigned i = 0; i < 2; ++i) { thinkthen_session *session = NULL;
        if (thinkthen_session_new(engine, requests[i], strlen(requests[i]), &session)) return 1;
        for (;;) {
            uint32_t status; thinkthen_session_result *owner = NULL;
            if (thinkthen_session_next_packet(session, THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, &status, &owner)) return 1;
            if (status == THINKTHEN_SESSION_END_V1) break;
            const thinkthen_complete_session_packet_v1 *packet = NULL;
            if (thinkthen_session_result_view(owner, &packet)) return 1;
            if (packet->kind == THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1 && packet->data.terminal->failure.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) return 1;
            if (packet->kind == THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1 && packet->data.observation->value->kind == THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1) {
                ++answers; const thinkthen_complete_session_judgment_v1 *judgment = packet->data.observation->value->data.row->value->data.judgment->value;
                if (!i) { const thinkthen_complete_u32_presence_v1 refund = judgment->data.decision->value; puts(refund.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? (refund.value == THINKTHEN_YES ? "true" : "false") : "not sure"); }
                else { const thinkthen_complete_utf8_presence_v1 team = judgment->data.choice->value; if (team.presence == THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) printf("%.*s\n", (int)team.value.len, team.value.data); else puts("not sure"); }
            }
            thinkthen_session_result_free(owner);
        }
        thinkthen_session_free(session);
    }
    thinkthen_engine_free(engine); return answers == 6 ? 0 : 1; }
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/c/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

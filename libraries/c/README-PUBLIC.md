# ThinkThen for C

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Download the matching archive from the [release downloads](https://github.com/botassembly/thinkthen/releases), then unpack it: `tar -xzf thinkthen-c-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform.
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```c
#include <thinkthen.h>
#include <stdio.h>
#include <string.h>
static thinkthen_complete_utf8_v1 text(const char *s) { return (thinkthen_complete_utf8_v1){s,strlen(s)}; }
int main(void) {
    thinkthen_engine *engine = thinkthen_engine_new(); if (!engine) return 1;
    thinkthen_named_request_v1 refund = {.function = {"decide",6},
        .question_json = text("{\"kind\":\"text\",\"text\":\"Does the customer ask for a refund?\"}"),
        .input_json = text("{\"kind\":\"text\",\"text\":\"Please refund my order. It arrived broken.\"}"), .options_json = text("{}")};
    thinkthen_named_request_v1 team = {.function = {"choose",6},
        .question_json = text("{\"kind\":\"definition\",\"value\":{\"choose\":\"Which team owns this?\",\"options\":{\"billing\":\"Invoices, fees, and refunds.\",\"shipping\":\"Parcels and delivery.\",\"account\":\"Logins and passwords.\"}}}"),
        .input_json = text("{\"kind\":\"text\",\"text\":\"My parcel went to the wrong address.\"}"), .options_json = text("{}")};
    thinkthen_call_result *refund_call = NULL; thinkthen_call_error *error = NULL;
    int code = thinkthen_run_call(engine, &refund, NULL, NULL, NULL, &refund_call, &error);
    if (code) { thinkthen_call_error_free(error); thinkthen_engine_free(engine); return 1; }
    thinkthen_complete_utf8_v1 json = thinkthen_call_result_json(refund_call);
    fwrite(json.data,1,json.len,stdout); putchar('\n'); thinkthen_call_result_free(refund_call);
    thinkthen_call_result *team_call = NULL;
    code = thinkthen_run_call(engine, &team, NULL, NULL, NULL, &team_call, &error);
    thinkthen_engine_free(engine); if (code) { thinkthen_call_error_free(error); return 1; }
    json = thinkthen_call_result_json(team_call); fwrite(json.data,1,json.len,stdout); putchar('\n');
    thinkthen_call_result_free(team_call); return 0;
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/c/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)

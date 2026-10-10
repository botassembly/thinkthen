"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
async function typed(client, q, input) {
    const done = await client.recognize(q, input);
    const row = done.results[0];
    console.log(row?.value.entities);
}
void typed;
// Execute the same fixture consumer after the compiler checks named calls.
require("./native_case.mjs");

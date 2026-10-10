"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const thinkthen_1 = require("thinkthen");
function usage(owner) {
    const status = owner.usagePersistence();
    const state = owner.finishUsageStatus().state;
    const advice = status.advice;
    // @ts-expect-error live status is immutable
    status.state = 'written';
    console.log(state, advice);
}
async function named(client) {
    const done = await client.decide('Question?', false);
    const row = done.results[0];
    const id = row.answer_id;
    const original = row.input;
    const value = row.value;
    const facts = done.facts;
    const terminal = done.terminal;
    if (terminal.has('failure'))
        console.log(terminal.failure?.error.kind);
    console.log(id, original, value, facts?.call_id);
}
function failure(error) { if (error instanceof thinkthen_1.ClientError) {
    const facts = error.facts;
    console.log(error.complete?.error.kind, facts?.call_id, error.results?.length);
} }
void [named, failure, usage];

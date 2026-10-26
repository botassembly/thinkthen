// @ts-check
import { isProxy } from "node:util/types";
import { CancelledError, engineError, hostValue, nativeError, loadNative } from "./bridge.js";
import { encodeRecord } from "./bridge.js";
/** @typedef {import('./bridge.js').NativeClient} NativeClient */
/** @typedef {import('./bridge.js').Operation} Operation */
/** @typedef {import('./bridge.js').Reply} Reply */
/** @typedef {import('./index.d.ts').AnyRow} Row */
/** @typedef {{row: Row, position: number, id: {kind: string, value?: string}}} Packet */
/** @typedef {{packet: Packet, reply: Reply}} Event */
/** @param {string} text @returns {Packet} */
function packet(text) {
  const parsed = /** @type {unknown} */(JSON.parse(text));
  const value = /** @type {Packet} */(parsed);
  if (!Number.isSafeInteger(value.position)) throw engineError(null);
  if (value.id.kind !== "absent") {
    const integer = value.id.kind === "string" ? undefined : BigInt(/** @type {string} */(value.id.value));
    const id = integer === undefined ? value.id.value : Number.isSafeInteger(Number(integer)) ? Number(integer) : integer;
    value.row = /** @type {Row} */({...value.row, id});
  }
  value.row = /** @type {Row} */(hostValue(value.row));
  return value;
}
/**
 * @param {NativeClient} client @param {string} request
 * @param {Iterable<import('./index.d.ts').RecordInput> | AsyncIterable<import('./index.d.ts').RecordInput>} records
 * @param {AbortSignal | undefined} signal @param {boolean} filter @param {() => Promise<void>} release
 * @returns {import('./index.d.ts').RowStream<unknown>}
 */
export function stream(client, request, records, signal, filter, release) {
  /** @type {Iterator<import('./index.d.ts').RecordInput> | AsyncIterator<import('./index.d.ts').RecordInput> | undefined} */
  let iterator;
  /** @type {Operation | undefined} */ let operation;
  /** @type {Event[]} */ const events = [];
  /** @type {Map<number, import('./index.d.ts').RecordInput>} */ const originals = new Map();
  /** @type {(() => void) | undefined} */ let wake;
  /** @type {(() => void) | undefined} */ let completed;
  /** @type {Promise<void> | undefined} */ let completion;
  /** @type {Promise<void> | undefined} */ let cleaning;
  /** @type {Reply | undefined} */ let acknowledgement;
  let started = false, stopped = false, ended = false, inputEnded = false, position = 0;
  let failed = false;
  /** @type {unknown} */ let failure;
  const notify = () => { wake?.(); wake = undefined; };
  /** @param {unknown} error */
  const fail = error => { if (!failed && !stopped) { failed = true; failure = error; } stopped = true; operation?.cancel(); notify(); void cleanup(); };
  const abort = () => {
    try { loadNative().rejectCancellation(); }
    catch (error) { const native = nativeError(error); fail(new CancelledError({ code: native.code, message: native.message }, { cause: signal?.reason })); }
  };
  function cleanup() {
    if (cleaning) return cleaning;
    stopped = true;
    signal?.removeEventListener("abort", abort);
    operation?.reference();
    operation?.cancel();
    acknowledgement?.("", false); acknowledgement = undefined;
    for (const event of events.splice(0)) event.reply("", false);
    notify();
    // Start iterator return immediately, independently of an outstanding next.
    const returned = Promise.resolve().then(async () => {
      if (!inputEnded && iterator?.return) await iterator.return();
    });
    cleaning = (async () => {
      try { await Promise.all([returned, completion]); }
      catch (error) { if (!failed) { failed = true; failure = error; } }
      finally { operation?.unreference(); operation?.close(); operation = undefined; iterator = undefined; originals.clear(); await release(); }
    })();
    return cleaning;
  }
  /** @type {import('./bridge.js').Callback} */
  const callback = (error, kind, data, reply) => {
    if (kind === "done") {
      ended = true;
      if (data && !stopped) { failed = true; failure = engineError(/** @type {unknown} */(JSON.parse(data))); }
      reply("", false); completed?.(); notify(); return;
    }
    if (error) { reply("", false); fail(nativeError(error)); return; }
    if (stopped) { reply("", false); return; }
    if (kind === "row") {
      try { events.push({packet: packet(data), reply}); notify(); }
      catch (error) { reply("", false); fail(error); }
      return;
    }
    void (async () => {
      try {
        iterator ??= Symbol.asyncIterator in records ? records[Symbol.asyncIterator]() : records[Symbol.iterator]();
        operation?.unreference();
        const item = await iterator.next();
        if (!isStopped()) operation?.reference();
        if (isStopped() || item.done) { inputEnded ||= item.done === true; reply("", false); return; }
        if (filter) originals.set(++position, item.value);
        try {
          const encoded = typeof item.value === "string" ? {data: item.value} : encodeRecord(item.value);
          if (encoded.idKind === undefined) reply(encoded.data, typeof item.value === "string");
          else reply(encoded.data, false, false, encoded.idKind, encoded.idValue);
        }
        catch { reply("", false, true); }
      } catch (error) { reply("", false); fail(error); }
    })();
  };
  function isStopped() { return stopped; }
  function start() {
    started = true;
    if (signal?.aborted) { abort(); return; }
    signal?.addEventListener("abort", abort, { once: true });
    completion = new Promise(resolve => { completed = resolve; });
    try {
      const finite = !isProxy(records) && Array.isArray(records) && Object.getPrototypeOf(records) === Array.prototype
        && !Object.hasOwn(records, Symbol.iterator) && !(Symbol.asyncIterator in records);
      operation = finite ? client.startFinite(request, callback) : client.start(request, callback);
    } catch (error) { completed?.(); fail(nativeError(error)); }
  }
  async function next() {
    if (!started && !stopped) start();
    if (!stopped && !ended) operation?.reference();
    acknowledgement?.("", false); acknowledgement = undefined;
    for (;;) {
      if (stopped || ended && events.length === 0) {
        await cleanup();
        if (failed) { failed = false; throw failure; }
        return {done: true, value: undefined};
      }
      const event = events.shift();
      if (event) {
        acknowledgement = event.reply;
        operation?.unreference();
        const row = event.packet.row;
        const item = originals.get(event.packet.position);
        originals.delete(event.packet.position);
        return { done: false, value: filter ? Object.freeze({item, row, kept: row.status === "answered" && row.answer === true}) : row };
      }
      await new Promise(resolve => { wake = () => { resolve(undefined); }; });
    }
  }
  let queue = Promise.resolve();
  const result = {
    [Symbol.asyncIterator]() { return this; },
    next() {
      const pending = queue.then(next);
      queue = pending.then(() => {}, () => {});
      return pending;
    },
    async return() { await cleanup(); return {done: true, value: undefined}; },
    async [Symbol.asyncDispose]() { await result.return(); },
    async toArray() { const rows = []; for await (const row of result) rows.push(row); return rows; },
  };
  return result;
}

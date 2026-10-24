// @ts-check
import { createRequire } from "node:module";
import { isNumberObject } from "node:util/types";

/** @typedef {import('./index.d.ts').ClientOptions} ClientOptions */
/** @typedef {import('./index.d.ts').CallOptions} CallOptions */
/** @typedef {import('./index.d.ts').RecordInput} RecordInput */
/** @typedef {import('./index.d.ts').Row} Row */
/** @typedef {import('./index.d.ts').ErrorInfo} ErrorInfo */
/** @typedef {{cancel(): void, reference(): void, unreference(): void}} Operation */
/** @typedef {(error: Error | null, kind: string, data: string, reply: (data: string, text: boolean) => void) => void} Callback */
/** @typedef {{start(request: string, callback: Callback): Operation}} NativeClient */
/** @typedef {{row: Row, detail: string | null, kept: boolean, position: number}} Packet */
/** @typedef {{packet: Packet, reply: (data: string, text: boolean) => void}} Event */

const require = createRequire(import.meta.url);
/** @type {unknown} */
const loaded = require("./thinkthen.node");
const native = /** @type {{Client: new (settings: string) => NativeClient, rejectNonFiniteJson(): never}} */ (loaded);

/** @param {string} text @returns {unknown} */
function parseJson(text) { return JSON.parse(text); }

export class ThinkThenError extends Error {
  /** @param {ErrorInfo} error */
  constructor(error) {
    super(error.message);
    this.name = new.target.name;
    /** @readonly */
    this.code = error.code;
  }
}
export class UsageError extends ThinkThenError {}
export class BackendError extends ThinkThenError {}
export class LocalError extends ThinkThenError {}

/** @param {ErrorInfo & {family?: string}} error */
function engineError(error) {
  const Kind = error.family === "usage" ? UsageError
    : error.family === "backend" ? BackendError
    : error.family === "local" ? LocalError : ThinkThenError;
  return new Kind(error);
}

/** @param {unknown} error @returns {unknown} */
function nativeError(error) {
  if (error instanceof Error) {
    try { return engineError(/** @type {ErrorInfo} */ (parseJson(error.message))); }
    catch { return error; }
  }
  return error;
}

/** @param {unknown} value @returns {string} */
function encode(value) {
  return JSON.stringify(value, (_key, item) => {
    let field = /** @type {unknown} */ (item);
    if (isNumberObject(field)) field = Number(field);
    if (typeof field === "number" && !Number.isFinite(field)) {
      try { native.rejectNonFiniteJson(); }
      catch (error) { throw nativeError(error); }
    }
    return field;
  });
}

/**
 * @template {RecordInput} T
 * @param {NativeClient} client
 * @param {string} name
 * @param {unknown} question
 * @param {Iterable<T> | AsyncIterable<T>} records
 * @param {CallOptions} options
 * @param {boolean} originalsNeeded
 * @returns {AsyncGenerator<{row: Row, kept: boolean, original: T | undefined}>}
 */
async function* iterate(client, name, question, records, options, originalsNeeded) {
  const { signal, ...questionOptions } = options;
  signal?.throwIfAborted();
  const iterator = Symbol.asyncIterator in records
    ? records[Symbol.asyncIterator]() : records[Symbol.iterator]();
  /** @type {Map<number, T>} */
  const originals = new Map();
  let position = 0;
  /** @type {Event[]} */
  const events = [];
  /** @type {(() => void) | undefined} */
  let wake;
  const state = { ended: false, finished: false, inputEnded: false, failed: false };
  /** @type {unknown} */
  let failure;
  /** @type {Operation | undefined} */
  let operation;
  const notify = () => { wake?.(); wake = undefined; };
  /** @param {unknown} error */
  const fail = (error) => {
    if (!state.failed) { state.failed = true; failure = error; }
    operation?.cancel();
    notify();
  };
  const abort = () => { fail(signal?.reason); };
  /** @type {Callback} */
  const callback = (error, kind, data, reply) => {
    if (error) { fail(nativeError(error)); reply("", false); return; }
    if (kind === "done") {
      if (data && !state.failed) { state.failed = true; failure = engineError(/** @type {ErrorInfo} */ (parseJson(data))); }
      state.ended = true;
      reply("", false);
      notify();
    } else if (state.finished || state.failed) {
      reply("", false);
    } else if (kind === "row") {
      events.push({ packet: /** @type {Packet} */ (parseJson(data)), reply });
      notify();
    } else {
      void (async () => {
        try {
          operation?.unreference();
          const item = await iterator.next();
          if (!state.finished && !state.failed) operation?.reference();
          if (state.finished || state.failed || item.done) {
            state.inputEnded ||= item.done === true;
            reply("", false);
          } else {
            const text = typeof item.value === "string";
            const json = text ? item.value : encode(item.value);
            if (originalsNeeded) originals.set(++position, item.value);
            reply(/** @type {string} */ (json), text);
          }
        } catch (error) { fail(error); reply("", false); }
      })();
    }
  };
  try {
    signal?.addEventListener("abort", abort, { once: true });
    const tagged = question !== null && typeof question === "object" && "type" in question ? question : { type: "inline", value: question };
    const request = encode({ function: name, question: tagged, options: questionOptions });
    try {
      operation = client.start(request, callback);
    } catch (error) { throw nativeError(error); }
    if (signal?.aborted) abort();
    for (;;) {
      if (state.failed) throw failure;
      const event = events.shift();
      if (event) {
        try {
          const row = event.packet.row;
          let rawDetail = event.packet.detail;
          /** @type {unknown} */
          let detail;
          Object.defineProperty(row, "detail", { enumerable: rawDetail !== null,
            get() { if (rawDetail !== null) { detail = parseJson(rawDetail); rawDetail = null; } return detail; } });
          const original = originals.get(event.packet.position);
          // Retire preceding inputs when an engine row skips their positions.
          for (const key of originals.keys()) {
            if (key > event.packet.position) break;
            originals.delete(key);
          }
          operation.unreference();
          yield { row, kept: event.packet.kept, original };
        } finally { operation.reference(); event.reply("", false); }
      } else if (state.ended) {
        return;
      } else {
        await new Promise(resolve => { wake = () => { resolve(undefined); }; });
      }
    }
  } finally {
    state.finished = true;
    signal?.removeEventListener("abort", abort);
    if (!state.ended) operation?.cancel();
    operation?.unreference();
    for (const event of events) event.reply("", false);
    if (!state.inputEnded && iterator.return) {
      try { void Promise.resolve(iterator.return()).catch(() => {}); } catch { /* Preserve the operation's result. */ }
    }
  }
}

/**
 * @template A
 * @param {NativeClient} client @param {string} name @param {unknown} question
 * @param {Iterable<RecordInput> | AsyncIterable<RecordInput>} records @param {CallOptions} [options]
 * @returns {AsyncGenerator<import('./index.d.ts').Row<A>>}
 */
async function* rows(client, name, question, records, options = {}) {
  for await (const packet of iterate(client, name, question, records, options, false)) {
    yield /** @type {import('./index.d.ts').Row<A>} */ (packet.row);
  }
}

/**
 * @template A
 * @param {NativeClient} client @param {string} name @param {unknown} question
 * @param {RecordInput} record @param {CallOptions} [options]
 * @returns {Promise<import('./index.d.ts').Row<A>>}
 */
async function single(client, name, question, record, options) {
  /** @type {import('./index.d.ts').Row<A> | undefined} */
  let result;
  for await (const packet of iterate(client, name, question, [record], options ?? {}, false)) result = /** @type {import('./index.d.ts').Row<A>} */ (packet.row);
  return /** @type {import('./index.d.ts').Row<A>} */ (result);
}

/**
 * @template {RecordInput} T
 * @param {NativeClient} client @param {import('./index.d.ts').Question<string>} question
 * @param {Iterable<T> | AsyncIterable<T>} records @param {CallOptions} options
 * @returns {AsyncGenerator<T>}
 */
async function* kept(client, question, records, options) {
  for await (const packet of iterate(client, "filter", question, records, options, true)) {
    if (packet.kept) yield /** @type {T} */ (packet.original);
  }
}

export class ThinkThen {
  /** @type {NativeClient} */
  #client;
  /** @param {ClientOptions} [options] */
  constructor(options = {}) {
    const { keySource, ...clientOptions } = options;
    const settings = encode({ ...clientOptions, key_source: keySource });
    try { this.#client = new native.Client(settings); }
    catch (error) { throw nativeError(error); }
  }
  /** @type {typeof import('./index.d.ts').decide} */
  decide = (q, r, o) => single(this.#client, "decide", q, r, o);
  /** @type {typeof import('./index.d.ts').decideStream} */
  decideStream = (q, r, o) => rows(this.#client, "decide", q, r, o);
  /** @type {typeof import('./index.d.ts').choose} */
  choose = (q, r, o) => single(this.#client, "choose", q, r, o);
  /** @type {typeof import('./index.d.ts').chooseStream} */
  chooseStream = (q, r, o) => rows(this.#client, "choose", q, r, o);
  /** @type {typeof import('./index.d.ts').tag} */
  tag = (q, r, o) => single(this.#client, "tag", q, r, o);
  /** @type {typeof import('./index.d.ts').tagStream} */
  tagStream = (q, r, o) => rows(this.#client, "tag", q, r, o);
  /** @type {typeof import('./index.d.ts').score} */
  score = (q, r, o) => single(this.#client, "score", q, r, o);
  /** @type {typeof import('./index.d.ts').scoreStream} */
  scoreStream = (q, r, o) => rows(this.#client, "score", q, r, o);
  /** @type {typeof import('./index.d.ts').find} */
  find = (q, r, o) => single(this.#client, "find", q, r, o);
  /** @type {typeof import('./index.d.ts').findStream} */
  findStream = (q, r, o) => rows(this.#client, "find", q, r, o);
  /** @type {typeof import('./index.d.ts').annotate} */
  annotate = (q, r, o) => single(this.#client, "annotate", q, r, o);
  /** @type {typeof import('./index.d.ts').annotateStream} */
  annotateStream = (q, r, o) => rows(this.#client, "annotate", q, r, o);
  /** @type {typeof import('./index.d.ts').recognize} */
  recognize = (q, r, o) => single(this.#client, "recognize", q, r, o);
  /** @type {typeof import('./index.d.ts').recognizeStream} */
  recognizeStream = (q, r, o) => rows(this.#client, "recognize", q, r, o);
  /** @type {typeof import('./index.d.ts').relate} */
  relate = (q, r, o) => single(this.#client, "relate", q, r, o);
  /** @type {typeof import('./index.d.ts').relateStream} */
  relateStream = (q, r, o) => rows(this.#client, "relate", q, r, o);
  /** @type {typeof import('./index.d.ts').rankStream} */
  rankStream = (q, r, o) => rows(this.#client, "rank", q, r, o);
  /** @type {typeof import('./index.d.ts').filterStream} */
  filterStream = (q, records, options = {}) => kept(this.#client, q, records, options);
}

/** @type {ThinkThen | undefined} */
let defaultClient;
function defaults() { return defaultClient ??= new ThinkThen(); }

/** @type {typeof import('./index.d.ts').decide} */
export const decide = async (q, r, o) => defaults().decide(q, r, o);
/** @type {typeof import('./index.d.ts').decideStream} */
export const decideStream = async function* (q, r, o) { yield* defaults().decideStream(q, r, o); };
/** @type {typeof import('./index.d.ts').choose} */
export const choose = async (q, r, o) => defaults().choose(q, r, o);
/** @type {typeof import('./index.d.ts').chooseStream} */
export const chooseStream = async function* (q, r, o) { yield* defaults().chooseStream(q, r, o); };
/** @type {typeof import('./index.d.ts').tag} */
export const tag = async (q, r, o) => defaults().tag(q, r, o);
/** @type {typeof import('./index.d.ts').tagStream} */
export const tagStream = async function* (q, r, o) { yield* defaults().tagStream(q, r, o); };
/** @type {typeof import('./index.d.ts').score} */
export const score = async (q, r, o) => defaults().score(q, r, o);
/** @type {typeof import('./index.d.ts').scoreStream} */
export const scoreStream = async function* (q, r, o) { yield* defaults().scoreStream(q, r, o); };
/** @type {typeof import('./index.d.ts').find} */
export const find = async (q, r, o) => defaults().find(q, r, o);
/** @type {typeof import('./index.d.ts').findStream} */
export const findStream = async function* (q, r, o) { yield* defaults().findStream(q, r, o); };
/** @type {typeof import('./index.d.ts').annotate} */
export const annotate = async (q, r, o) => defaults().annotate(q, r, o);
/** @type {typeof import('./index.d.ts').annotateStream} */
export const annotateStream = async function* (q, r, o) { yield* defaults().annotateStream(q, r, o); };
/** @type {typeof import('./index.d.ts').recognize} */
export const recognize = async (q, r, o) => defaults().recognize(q, r, o);
/** @type {typeof import('./index.d.ts').recognizeStream} */
export const recognizeStream = async function* (q, r, o) { yield* defaults().recognizeStream(q, r, o); };
/** @type {typeof import('./index.d.ts').relate} */
export const relate = async (q, r, o) => defaults().relate(q, r, o);
/** @type {typeof import('./index.d.ts').relateStream} */
export const relateStream = async function* (q, r, o) { yield* defaults().relateStream(q, r, o); };
/** @type {typeof import('./index.d.ts').rankStream} */
export const rankStream = async function* (q, r, o) { yield* defaults().rankStream(q, r, o); };
/** @type {typeof import('./index.d.ts').filterStream} */
export const filterStream = async function* (q, r, o) { yield* defaults().filterStream(q, r, o); };

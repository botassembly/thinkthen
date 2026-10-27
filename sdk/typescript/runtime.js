// @ts-check
import { ThinkThenError, RULE_FIELDS, conversion, encode, snapshot, loadNative, nativeError } from "./bridge.js";
import { stream } from "./stream.js";
export { ThinkThenError, UsageError, BackendError, CancelledError } from "./bridge.js";
export { ERROR_CODES } from "./errors.js";
/** @typedef {import('./index.d.ts').RecordInput} RecordInput */
/** @typedef {import('./index.d.ts').Records} Records */
/** @typedef {import('./index.d.ts').CallOptions} CallOptions */
/** @typedef {import('./index.d.ts').RowStream<unknown>} Stream */
/** @param {unknown} value @returns {unknown} */
function pointer(value) {
  return Array.isArray(value) ? value.length === 0 ? "" : "/" + value.map(part => String(part).replaceAll("~", "~0").replaceAll("/", "~1")).join("/") : value;
}
/** @param {object} value @param {Readonly<Record<string, string>>} names @returns {Record<string, unknown>} */
function rename(value, names) {
  /** @type {Record<string, unknown>} */ const result = {};
  for (const [key, child] of Object.entries(value)) result[names[key] ?? key] = /** @type {unknown} */(child);
  return result;
}
const hostRuleFields = Object.fromEntries(Object.entries(RULE_FIELDS).map(([host, wire]) => [wire, host]));
/** @param {unknown} value @returns {unknown} */
function wireQuestion(value) {
  if (!value || typeof value !== "object") return value;
  /** @type {Record<string, unknown>} */ const fields = Object.fromEntries(Object.entries(value).map(([key, child]) => [key, key === "evidence" ? pointer(child) : child]));
  if (Array.isArray(fields.rules)) fields.rules = fields.rules.map((/** @type {unknown} */ rule) => {
    if (!rule || typeof rule !== "object") return rule;
    return rename(rule, RULE_FIELDS);
  });
  if (fields.questions && typeof fields.questions === "object") fields.questions = Object.fromEntries(Object.entries(fields.questions).map(([key, child]) => [key, wireQuestion(child)]));
  return fields;
}
/** @template {import('./index.d.ts').Toolbox} T @param {T} value @returns {import('./index.d.ts').DeepReadonly<T>} */
export function defineToolbox(value) { return /** @type {import('./index.d.ts').DeepReadonly<T>} */(conversion(() => snapshot(value), () => loadNative().rejectQuestionConversion())); }

/** @param {string} json @returns {import('./index.d.ts').Toolbox} */
export function parseToolbox(json) {
  try {
    const decoded = /** @type {unknown} */(JSON.parse(loadNative().parseToolbox(json)));
    const definitions = /** @type {Record<string, Record<string, unknown>>} */(decoded);
    const entries = Object.entries(definitions).map(([name, question]) => {
      if (question.function !== "relate") return [name, question];
      const rules = /** @type {readonly object[]} */(question.rules);
      return [name, {...question, rules: rules.map(rule => rename(rule, hostRuleFields))}];
    });
    return /** @type {import('./index.d.ts').Toolbox} */(snapshot(Object.fromEntries(entries)));
  } catch (error) { throw nativeError(error); }
}
/** @param {import('./index.d.ts').Toolbox} toolbox @returns {string} */
export function serializeToolbox(toolbox) {
  const json = conversion(() => encode(Object.fromEntries(Object.entries(toolbox).map(([name, question]) => [name, wireQuestion(snapshot(question))]))), () => loadNative().rejectQuestionConversion());
  try { return loadNative().serializeToolbox(json); }
  catch (error) { throw nativeError(error); }
}

export class ThinkThen {
  /** @type {import('./bridge.js').NativeClient} */ #client;
  /** @type {Set<Stream>} */ #calls = new Set();
  /** @type {Promise<void> | undefined} */ #closing;
  /** @param {import('./index.d.ts').ClientOptions} [options] */
  constructor(options = {}) {
    const native = loadNative();
    const settings = conversion(() => {
      const { apiKey, retry, ...fields } = /** @type {import('./index.d.ts').ClientOptions} */(snapshot(options));
      const names = { maxRetries: "max_retries", initialBackoffMs: "initial_backoff_ms", attemptTimeoutMs: "attempt_timeout_ms" };
      return encode({ ...fields, ...(apiKey === undefined ? {} : {api_key: apiKey}),
        ...(retry === undefined ? {} : {retry: rename(retry, names)}) });
    }, () => native.rejectSettingsConversion());
    try { this.#client = new native.Client(settings); }
    catch (error) { throw nativeError(error); }
  }
  close() {
    if (!this.#closing) {
      this.#client.close();
      this.#closing = Promise.all([...this.#calls].map(call => call.return())).then(() => {});
    }
    return this.#closing;
  }
  async [Symbol.asyncDispose]() { await this.close(); }
  /** @param {string} name @param {unknown} question @param {Records} records @param {CallOptions & object} [options] @returns {Stream} */
  #stream(name, question, records, options = {}) {
    let signal;
    let request = "";
    /** @type {ThinkThenError | undefined} */ let failure;
    try {
      const {signal: liveSignal, ...fields} = options;
      signal = liveSignal;
      const executionName = fields.name; delete fields.name;
      const slot = name === "recognize" ? "kinds" : name === "relate" ? "rules" : name === "annotate" ? "questions" : "question";
      const definition = name !== "annotate" && question !== null && typeof question === "object" && "function" in question
        ? {...question, ...fields} : {function: name, [slot]: question, ...fields};
      request = conversion(() => encode({ question: wireQuestion(snapshot(definition)), ...(executionName === undefined ? {} : {name: executionName}) }), () => loadNative().rejectQuestionConversion());
    } catch (error) {
      if (error instanceof ThinkThenError) failure = error;
      else { try { loadNative().rejectQuestionConversion(); } catch (error) { failure = nativeError(error); } }
    }
    /** @type {Stream} */ let result;
    const release = () => { this.#calls.delete(result); return Promise.resolve(); };
    if (failure) {
      const error = failure;
      let raised = false;
      result = {
        [Symbol.asyncIterator]() { return this; },
        async next() { const reject = !raised; raised = true; await release(); if (reject) throw error; return {done: true, value: undefined}; },
        async return() { raised = true; await release(); return {done: true, value: undefined}; },
        async [Symbol.asyncDispose]() { await this.return(); },
        async toArray() { await this.next(); return []; },
      };
    } else result = stream(this.#client, request, records, signal, name, release);
    this.#calls.add(result);
    return result;
  }
  /** @param {string} name @param {unknown} question @param {RecordInput} record @param {CallOptions} [options] @returns {Promise<unknown>} */
  async #single(name, question, record, options) {
    const call = this.#stream(name, question, [record], options);
    try { return (await call.toArray())[0]; } finally { await call.return(); }
  }
  /** @type {typeof import("./index.d.ts").decide} */
  decide = /** @type {typeof import("./index.d.ts").decide} */((q, r, o) => this.#single("decide", q, r, o));
  /** @type {typeof import("./index.d.ts").decideStream} */
  decideStream = /** @type {typeof import("./index.d.ts").decideStream} */((q, r, o) => this.#stream("decide", q, r, o));
  /** @type {typeof import("./index.d.ts").choose} */
  choose = /** @type {typeof import("./index.d.ts").choose} */((q, r, o) => this.#single("choose", q, r, o));
  /** @type {typeof import("./index.d.ts").chooseStream} */
  chooseStream = /** @type {typeof import("./index.d.ts").chooseStream} */((q, r, o) => this.#stream("choose", q, r, o));
  /** @type {typeof import("./index.d.ts").tag} */
  tag = /** @type {typeof import("./index.d.ts").tag} */((q, r, o) => this.#single("tag", q, r, o));
  /** @type {typeof import("./index.d.ts").tagStream} */
  tagStream = /** @type {typeof import("./index.d.ts").tagStream} */((q, r, o) => this.#stream("tag", q, r, o));
  /** @type {typeof import("./index.d.ts").score} */
  score = /** @type {typeof import("./index.d.ts").score} */((q, r, o) => this.#single("score", q, r, o));
  /** @type {typeof import("./index.d.ts").scoreStream} */
  scoreStream = /** @type {typeof import("./index.d.ts").scoreStream} */((q, r, o) => this.#stream("score", q, r, o));
  /** @type {typeof import("./index.d.ts").find} */
  find = /** @type {typeof import("./index.d.ts").find} */((q, r, o) => this.#single("find", q, r, o));
  /** @type {typeof import("./index.d.ts").findStream} */
  findStream = /** @type {typeof import("./index.d.ts").findStream} */((q, r, o) => this.#stream("find", q, r, o));
  /** @type {typeof import("./index.d.ts").recognize} */
  recognize = /** @type {typeof import("./index.d.ts").recognize} */((q, r, o) => this.#single("recognize", q, r, o));
  /** @type {typeof import("./index.d.ts").recognizeStream} */
  recognizeStream = /** @type {typeof import("./index.d.ts").recognizeStream} */((q, r, o) => this.#stream("recognize", q, r, o));
  /** @type {typeof import("./index.d.ts").relate} */
  relate = /** @type {typeof import("./index.d.ts").relate} */((q, r, o) => this.#single("relate", q, r, o));
  /** @type {typeof import("./index.d.ts").relateStream} */
  relateStream = /** @type {typeof import("./index.d.ts").relateStream} */((q, r, o) => this.#stream("relate", q, r, o));
  /** @type {typeof import("./index.d.ts").annotate} */
  annotate = /** @type {typeof import("./index.d.ts").annotate} */((q, r, o) => this.#single("annotate", q, r, o));
  /** @type {typeof import("./index.d.ts").annotateStream} */
  annotateStream = /** @type {typeof import("./index.d.ts").annotateStream} */((q, r, o) => this.#stream("annotate", q, r, o));
  /** @type {typeof import("./index.d.ts").filterStream} */
  filterStream = /** @type {typeof import("./index.d.ts").filterStream} */((q, r, o) => this.#stream("filter", q, r, o));
  /** @type {typeof import("./index.d.ts").rankStream} */
  rankStream = /** @type {typeof import("./index.d.ts").rankStream} */((q, r, o) => this.#stream("rank", q, r, o));
}
/** @param {(client: ThinkThen) => Promise<unknown>} call @returns {Promise<unknown>} */
async function standaloneSingle(call) {
  const client = new ThinkThen();
  try { return await call(client); } finally { await client.close(); }
}
/** @param {(client: ThinkThen) => Stream} call @returns {Stream} */
function standaloneStream(call) {
  const client = new ThinkThen();
  const result = call(client);
  const finish = async () => { await result.return(); await client.close(); };
  return {
    [Symbol.asyncIterator]() { return this; },
    async next() {
      try { const value = await result.next(); if (value.done) await client.close(); return value; }
      catch (error) { await client.close(); throw error; }
    },
    async return() { await finish(); return {done: true, value: undefined}; },
    async [Symbol.asyncDispose]() { await finish(); },
    async toArray() { try { return await result.toArray(); } finally { await finish(); } },
  };
}
/** @type {typeof import("./index.d.ts").decide} */
export const decide = /** @type {typeof import("./index.d.ts").decide} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.decide))(q, r, o)));
/** @type {typeof import("./index.d.ts").decideStream} */
export const decideStream = /** @type {typeof import("./index.d.ts").decideStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.decideStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").choose} */
export const choose = /** @type {typeof import("./index.d.ts").choose} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.choose))(q, r, o)));
/** @type {typeof import("./index.d.ts").chooseStream} */
export const chooseStream = /** @type {typeof import("./index.d.ts").chooseStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.chooseStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").tag} */
export const tag = /** @type {typeof import("./index.d.ts").tag} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.tag))(q, r, o)));
/** @type {typeof import("./index.d.ts").tagStream} */
export const tagStream = /** @type {typeof import("./index.d.ts").tagStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.tagStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").score} */
export const score = /** @type {typeof import("./index.d.ts").score} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.score))(q, r, o)));
/** @type {typeof import("./index.d.ts").scoreStream} */
export const scoreStream = /** @type {typeof import("./index.d.ts").scoreStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.scoreStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").find} */
export const find = /** @type {typeof import("./index.d.ts").find} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.find))(q, r, o)));
/** @type {typeof import("./index.d.ts").findStream} */
export const findStream = /** @type {typeof import("./index.d.ts").findStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.findStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").recognize} */
export const recognize = /** @type {typeof import("./index.d.ts").recognize} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.recognize))(q, r, o)));
/** @type {typeof import("./index.d.ts").recognizeStream} */
export const recognizeStream = /** @type {typeof import("./index.d.ts").recognizeStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.recognizeStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").relate} */
export const relate = /** @type {typeof import("./index.d.ts").relate} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.relate))(q, r, o)));
/** @type {typeof import("./index.d.ts").relateStream} */
export const relateStream = /** @type {typeof import("./index.d.ts").relateStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.relateStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").annotate} */
export const annotate = /** @type {typeof import("./index.d.ts").annotate} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<unknown>} */(client.annotate))(q, r, o)));
/** @type {typeof import("./index.d.ts").annotateStream} */
export const annotateStream = /** @type {typeof import("./index.d.ts").annotateStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.annotateStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").filterStream} */
export const filterStream = /** @type {typeof import("./index.d.ts").filterStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.filterStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").rankStream} */
export const rankStream = /** @type {typeof import("./index.d.ts").rankStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.rankStream))(q, r, o)));

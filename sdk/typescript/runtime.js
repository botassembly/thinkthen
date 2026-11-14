// @ts-check
import { ThinkThenError, conversion, encode, snapshot, loadNative, nativeError, rejectRecordKind } from "./bridge.js";
import { readFile } from "node:fs/promises";
import { stream } from "./stream.js";
import { resultPromise, keptRecords, FailedError } from "./answers.js";
export { NoAnswerError, UnsureError, FailedError } from "./answers.js";
export { ThinkThenError, UsageError, SetupError, InternalError, CancelledError } from "./bridge.js";
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
/** @param {unknown} value @returns {unknown} */
function wireQuestion(value) {
  if (!value || typeof value !== "object") return value;
  /** @type {Record<string, unknown>} */ const fields = Object.fromEntries(Object.entries(value).map(([key, child]) => [key, key === "field" ? pointer(child) : child]));
  if (Object.hasOwn(fields, "yesMeans")) { fields.yes_means = fields.yesMeans; delete fields.yesMeans; }
  if (Object.hasOwn(fields, "noMeans")) { fields.no_means = fields.noMeans; delete fields.noMeans; }
  const descriptions = fields.descriptions;
  if (descriptions !== null && typeof descriptions === "object" && !Array.isArray(descriptions)) {
    const slot = fields.function === "choose" ? "choices" : fields.function === "tag" ? "labels" : "levels";
    const labels = fields[slot];
    if (Array.isArray(labels)) {
      fields[slot] = labels.map((/** @type {unknown} */ label) => typeof label === "string" && Object.hasOwn(descriptions, label)
        ? {name: label, description: /** @type {unknown} */(Reflect.get(descriptions, label))} : label);
      // Leave unsupported keys for the shared engine to refuse rather than drop them.
      if (Object.keys(descriptions).every(label => labels.includes(label))) delete fields.descriptions;
    }
  }
  if (fields.questions && typeof fields.questions === "object") fields.questions = Object.fromEntries(Object.entries(fields.questions).map(([key, child]) => [key, wireQuestion(child)]));
  return fields;
}
/** @template {import('./index.d.ts').ToolboxDefinitions} T @param {T} value @returns {import('./index.d.ts').DeepReadonly<T>} */
export function defineToolbox(value) { return /** @type {import('./index.d.ts').DeepReadonly<T>} */(conversion(() => snapshot(value), () => loadNative().rejectQuestionConversion())); }

/** @param {string} json @returns {import('./index.d.ts').ToolboxDefinitions} */
export function parseToolbox(json) {
  try {
    const decoded = /** @type {unknown} */(JSON.parse(loadNative().parseToolbox(json)));
    const definitions = /** @type {Record<string, Record<string, unknown>>} */(decoded);
    const entries = Object.entries(definitions).map(([name, question]) => {
      const fields = {...question};
      if (Object.hasOwn(fields, "yes_means")) { fields.yesMeans = fields.yes_means; delete fields.yes_means; }
      if (Object.hasOwn(fields, "no_means")) { fields.noMeans = fields.no_means; delete fields.no_means; }
      for (const slot of ["choices", "labels", "levels"]) {
        if (!Array.isArray(fields[slot])) continue;
        /** @type {[string, unknown][]} */ const descriptions = [];
        fields[slot] = fields[slot].map((/** @type {unknown} */ option) => {
          if (option === null || typeof option !== "object" || !("name" in option)) return option;
          if ("description" in option && typeof option.name === "string") descriptions.push([option.name, option.description]);
          return option.name;
        });
        if (descriptions.length) fields.descriptions = Object.fromEntries(descriptions);
      }
      return [name, fields];
    });
    return /** @type {import('./index.d.ts').ToolboxDefinitions} */(snapshot(Object.fromEntries(entries)));
  } catch (error) { throw nativeError(error); }
}
/** @param {import('./index.d.ts').ToolboxDefinitions} toolbox @returns {string} */
export function serializeToolbox(toolbox) {
  const json = conversion(() => encode(Object.fromEntries(Object.entries(toolbox).map(([name, question]) => [name, wireQuestion(snapshot(question))]))), () => loadNative().rejectQuestionConversion());
  try { return loadNative().serializeToolbox(json); }
  catch (error) { throw nativeError(error); }
}

/** @type {Client | undefined} */
let defaultClient;
/** @returns {Client} */
function sharedClient() { return defaultClient ??= new Client(); }

/** Saved questions admitted by the engine, with typed function-specific lookups. */
export class Toolbox {
  /** @type {import('./index.d.ts').ToolboxDefinitions} */ #definitions;
  /** @param {import('./index.d.ts').ToolboxDefinitions} definitions */
  constructor(definitions) { this.#definitions = parseToolbox(serializeToolbox(definitions)); }
  /** @param {string | URL} path */
  static async load(path) { return new Toolbox(parseToolbox(await readFile(path, "utf8"))); }
  /** @param {string} name @param {string} expected */
  #question(name, expected) {
    // The engine owns saved-name and function validation as well as wording.
    try { return parseToolbox(loadNative().toolboxQuestion(serializeToolbox(this.#definitions), name, expected))[name]; }
    catch (error) { throw nativeError(error); }
  }
  /** @param {string} name */
  decide(name) { return /** @type {import("./index.d.ts").DecideQuestion} */(this.#question(name, "decide")); }
  /** @param {string} name */
  choose(name) { return /** @type {import("./index.d.ts").ChooseQuestion} */(this.#question(name, "choose")); }
  /** @param {string} name */
  tag(name) { return /** @type {import("./index.d.ts").TagQuestion} */(this.#question(name, "tag")); }
  /** @param {string} name */
  score(name) { return /** @type {import("./index.d.ts").ScoreQuestion} */(this.#question(name, "score")); }
  /** @param {string} name */
  filter(name) { return /** @type {import("./index.d.ts").FilterQuestion} */(this.#question(name, "filter")); }
  /** @param {string} name */
  rank(name) { return /** @type {import("./index.d.ts").RankQuestion} */(this.#question(name, "rank")); }
  /** @param {string} name */
  find(name) { return /** @type {import("./index.d.ts").FindQuestion} */(this.#question(name, "find")); }
  /** @param {string} name */
  recognize(name) { return /** @type {import("./index.d.ts").RecognizeQuestion} */(this.#question(name, "recognize")); }
  /** @param {string} name */
  relate(name) { return /** @type {import("./index.d.ts").RelateQuestion} */(this.#question(name, "relate")); }
}

export class Client {
  /** @type {import('./bridge.js').NativeClient} */ #client;
  /** @type {Set<Stream>} */ #calls = new Set();
  /** @type {Promise<void> | undefined} */ #closing;
  /** @type {import('./index.d.ts').Usage | undefined} */ #finalUsage;
  /** @param {import('./index.d.ts').ClientOptions} [options] */
  constructor(options = {}) {
    const native = loadNative();
    const settings = conversion(() => {
      const { apiKey, retry, maxRequests, ...fields } = /** @type {import('./index.d.ts').ClientOptions} */(snapshot(options));
      if (fields.backend === undefined && process.env.THINKTHEN_BACKEND !== undefined) {
        fields.backend = /** @type {import('./index.d.ts').Backend} */(process.env.THINKTHEN_BACKEND);
      }
      const names = { maxRetries: "max_retries", initialBackoffMs: "initial_backoff_ms", attemptTimeoutMs: "attempt_timeout_ms" };
      if (typeof maxRequests === "number" && Math.abs(maxRequests) > Number.MAX_SAFE_INTEGER) throw new TypeError();
      return encode({ ...fields, ...(maxRequests == null ? {} : {max_requests: maxRequests}), ...(apiKey === undefined ? {} : {api_key: apiKey}),
        ...(retry === undefined ? {} : {retry: rename(retry, names)}) });
    }, () => native.rejectSettingsConversion());
    try { this.#client = new native.Client(settings); }
    catch (error) { throw nativeError(error); }
  }
  close() {
    if (!this.#closing) {
      this.#client.beginClose();
      this.#closing = Promise.allSettled([...this.#calls].map(call => call.return())).then(results => {
        try { this.#finalUsage = this.#usage(this.#client.settledUsage()); }
        finally { this.#client.close(); }
        const failed = results.find(result => result.status === "rejected");
        if (failed?.status === "rejected") throw failed.reason;
      });
    }
    return this.#closing;
  }
  /** Copy cumulative client usage; counters remain exact bigint values. */
  get usage() {
    if (this.#finalUsage) return this.#finalUsage;
    try { return this.#usage(this.#client.usage()); }
    catch (error) { throw nativeError(error); }
  }
  /** @param {string} json */
  #usage(json) {
    try {
      /** @type {unknown} */
      const decoded = JSON.parse(json);
      const value = /** @type {{attempts: string, responses: string, status_retries: string, connection_retries: string, request_body_bytes: string, active_attempts: string, active_calls: string, input_tokens: {reported: string | null, unknown_attempts: string}, output_tokens: {reported: string | null, unknown_attempts: string}}} */ (decoded);
      const tokens = (/** @type {{reported: string | null, unknown_attempts: string}} */ token) => Object.freeze({reported: token.reported === null ? null : BigInt(token.reported), unknownAttempts: BigInt(token.unknown_attempts)});
      return Object.freeze({attempts: BigInt(value.attempts), responses: BigInt(value.responses), statusRetries: BigInt(value.status_retries), connectionRetries: BigInt(value.connection_retries), requestBodyBytes: BigInt(value.request_body_bytes), activeAttempts: BigInt(value.active_attempts), activeCalls: BigInt(value.active_calls), inputTokens: tokens(value.input_tokens), outputTokens: tokens(value.output_tokens)});
    } catch (error) { throw nativeError(error); }
  }
  async [Symbol.asyncDispose]() { await this.close(); }
  /** @param {string} name @param {unknown} question @param {Records} records @param {CallOptions & object} [options] @returns {Stream} */
  #stream(name, question, records, options = {}) {
    let signal;
    let request = "";
    /** @type {ThinkThenError | undefined} */ let failure;
    try {
      if (typeof records === "string") rejectRecordKind();
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
  /** @param {string} name @param {unknown} question @param {RecordInput} record @param {CallOptions} [options] @returns {Promise<import("./index.d.ts").AnyRow>} */
  #single(name, question, record, options) {
    const row = (async () => {
      const collection = conversion(() => Array.isArray(record) || record !== null && typeof record === "object"
        && (Symbol.iterator in record || Symbol.asyncIterator in record), () => loadNative().rejectRecordKind());
      if (collection) throw new TypeError(`use ${name}Stream for a collection of records`);
      const call = this.#stream(name, question, [record], options);
      try {
        const result = /** @type {import('./index.d.ts').AnyRow} */((await call.toArray())[0]);
        if (result.status === "failed") throw new FailedError([result]);
        return result;
      }
      finally { await call.return(); }
    })();
    return resultPromise(row);
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
  recognize = /** @type {typeof import("./index.d.ts").recognize} */((r, o) => this.#single("recognize", undefined, r, o));
  /** @type {typeof import("./index.d.ts").recognizeStream} */
  recognizeStream = /** @type {typeof import("./index.d.ts").recognizeStream} */((r, o) => this.#stream("recognize", undefined, r, o));
  /** @type {typeof import("./index.d.ts").relate} */
  relate = /** @type {typeof import("./index.d.ts").relate} */((r, o) => this.#single("relate", undefined, r, o));
  /** @type {typeof import("./index.d.ts").relateStream} */
  relateStream = /** @type {typeof import("./index.d.ts").relateStream} */((r, o) => this.#stream("relate", undefined, r, o));
  /** @type {typeof import("./index.d.ts").annotate} */
  annotate = /** @type {typeof import("./index.d.ts").annotate} */((q, r, o) => this.#single("annotate", q, r, o));
  /** @type {typeof import("./index.d.ts").annotateStream} */
  annotateStream = /** @type {typeof import("./index.d.ts").annotateStream} */((q, r, o) => this.#stream("annotate", q, r, o));
  /** @type {typeof import("./index.d.ts").filter} */
  filter = /** @type {typeof import("./index.d.ts").filter} */((q, r, o) => keptRecords(/** @type {import('./index.d.ts').RowStream<import('./index.d.ts').FilterRow>} */(this.#stream("filter", q, r, o))));
  /** @type {typeof import("./index.d.ts").rank} */
  rank = /** @type {typeof import("./index.d.ts").rank} */((q, r, o) => this.#stream("rank", q, r, o));
}
/** @param {(client: Client) => Promise<import('./index.d.ts').AnyRow>} call @returns {Promise<import('./index.d.ts').AnyRow>} */
function standaloneSingle(call) {
  try { return call(sharedClient()); }
  catch (error) { return resultPromise(Promise.reject(nativeError(error))); }
}
/** @param {(client: Client) => Stream} call @returns {Stream} */
function standaloneStream(call) {
  /** @type {Stream | undefined} */ let result;
  /** @type {(() => never) | undefined} */ let rejection;
  try { result = call(sharedClient()); }
  catch (error) { rejection = () => { throw error; }; }
  return {
    [Symbol.asyncIterator]() { return this; },
    async next() {
      const reject = rejection; rejection = undefined;
      reject?.();
      return result ? result.next() : {done: true, value: undefined};
    },
    async return() { rejection = undefined; return result ? result.return() : {done: true, value: undefined}; },
    async [Symbol.asyncDispose]() { await this.return(); },
    async toArray() { const rows = []; for await (const row of this) rows.push(row); return rows; },
  };
}
/** @type {typeof import("./index.d.ts").decide} */
export const decide = /** @type {typeof import("./index.d.ts").decide} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<import("./index.d.ts").AnyRow>} */(client.decide))(q, r, o)));
/** @type {typeof import("./index.d.ts").decideStream} */
export const decideStream = /** @type {typeof import("./index.d.ts").decideStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.decideStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").choose} */
export const choose = /** @type {typeof import("./index.d.ts").choose} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<import("./index.d.ts").AnyRow>} */(client.choose))(q, r, o)));
/** @type {typeof import("./index.d.ts").chooseStream} */
export const chooseStream = /** @type {typeof import("./index.d.ts").chooseStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.chooseStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").tag} */
export const tag = /** @type {typeof import("./index.d.ts").tag} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<import("./index.d.ts").AnyRow>} */(client.tag))(q, r, o)));
/** @type {typeof import("./index.d.ts").tagStream} */
export const tagStream = /** @type {typeof import("./index.d.ts").tagStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.tagStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").score} */
export const score = /** @type {typeof import("./index.d.ts").score} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<import("./index.d.ts").AnyRow>} */(client.score))(q, r, o)));
/** @type {typeof import("./index.d.ts").scoreStream} */
export const scoreStream = /** @type {typeof import("./index.d.ts").scoreStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.scoreStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").find} */
export const find = /** @type {typeof import("./index.d.ts").find} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<import("./index.d.ts").AnyRow>} */(client.find))(q, r, o)));
/** @type {typeof import("./index.d.ts").findStream} */
export const findStream = /** @type {typeof import("./index.d.ts").findStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.findStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").recognize} */
export const recognize = /** @type {typeof import("./index.d.ts").recognize} */((r, o) => standaloneSingle(client => (/** @type {(r: RecordInput, o: object) => Promise<import('./index.d.ts').AnyRow>} */(client.recognize))(r, o)));
/** @type {typeof import("./index.d.ts").recognizeStream} */
export const recognizeStream = /** @type {typeof import("./index.d.ts").recognizeStream} */((r, o) => standaloneStream(client => (/** @type {(r: Records, o: object) => Stream} */(client.recognizeStream))(r, o)));
/** @type {typeof import("./index.d.ts").relate} */
export const relate = /** @type {typeof import("./index.d.ts").relate} */((r, o) => standaloneSingle(client => (/** @type {(r: RecordInput, o: object) => Promise<import('./index.d.ts').AnyRow>} */(client.relate))(r, o)));
/** @type {typeof import("./index.d.ts").relateStream} */
export const relateStream = /** @type {typeof import("./index.d.ts").relateStream} */((r, o) => standaloneStream(client => (/** @type {(r: Records, o: object) => Stream} */(client.relateStream))(r, o)));
/** @type {typeof import("./index.d.ts").annotate} */
export const annotate = /** @type {typeof import("./index.d.ts").annotate} */((q, r, o) => standaloneSingle(client => (/** @type {(q: unknown, r: RecordInput, o?: CallOptions) => Promise<import('./index.d.ts').AnyRow>} */(client.annotate))(q, r, o)));
/** @type {typeof import("./index.d.ts").annotateStream} */
export const annotateStream = /** @type {typeof import("./index.d.ts").annotateStream} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.annotateStream))(q, r, o)));
/** @type {typeof import("./index.d.ts").filter} */
export const filter = /** @type {typeof import("./index.d.ts").filter} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.filter))(q, r, o)));
/** @type {typeof import("./index.d.ts").rank} */
export const rank = /** @type {typeof import("./index.d.ts").rank} */((q, r, o) => standaloneStream(client => (/** @type {(q: unknown, r: Records, o?: CallOptions) => Stream} */(client.rank))(q, r, o)));

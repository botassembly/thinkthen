// @ts-check
import { inspect } from "node:util";
import { ThinkThenError } from "./bridge.js";
/** @typedef {import('./index.d.ts').Judgment} Judgment */
/** Either form of missing answer, without treating uncertainty as an error. */
export class NoAnswerError extends Error {
  /** @param {unknown} value */
  static [Symbol.hasInstance](value) {
    return (this === NoAnswerError && value instanceof FailedError)
      || Function.prototype[Symbol.hasInstance].call(this, value);
  }
}
export class UnsureError extends NoAnswerError {
  /** @param {Extract<Judgment, {status:'unsure'}>} row */
  constructor(row) {
    super("The judgment is not sure at the chosen thresholds.");
    this.name = "UnsureError";
    this.row = row;
    this.probability = row.probability;
  }
}
/** @template T @param {Readonly<Record<string, T>>} value @returns {[string, T][]} */
function orderedEntries(value) {
  return Object.entries(value).sort(([left], [right]) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}
/** @param {Judgment} row @returns {Extract<Judgment, {status:'failed'}> | undefined} */
function firstFailure(row) {
  if (row.status === "failed") return row;
  if (row.function === "annotate" && row.status === "answered" && row.answer && typeof row.answer === "object") {
    for (const [, child] of orderedEntries(/** @type {Record<string, Judgment>} */(row.answer))) {
      if (child.status === "failed") return child;
    }
  }
  return undefined;
}
export class FailedError extends ThinkThenError {
  /** @param {readonly Extract<Judgment, {status:'failed'}>[]} rows */
  constructor(rows) {
    const row = rows[0];
    if (!row) throw new TypeError("FailedError requires at least one failed row or form entry.");
    super(row.error);
    this.row = row;
    this.rows = Object.freeze([...rows]);
  }
}
/** @param {string} value @returns {string} */
function leaf(value) { return value.replace(/[\r\n\t]/g, " "); }
/** @param {unknown} value @returns {string} */
function printed(value) {
  if (value === null || value === undefined) return "";
  if (Array.isArray(value)) return value.map(printed).join("\t");
  if (typeof value === "string" || typeof value === "boolean" || typeof value === "number" || typeof value === "bigint") return leaf(String(value));
  if (typeof value !== "object") return "";
  if ("relation" in value && "source" in value && "target" in value) {
    const relation = /** @type {import('./index.d.ts').Relation} */(value);
    return `${leaf(relation.source.text)} --${leaf(relation.relation)}--> ${leaf(relation.target.text)}`;
  }
  if ("kind" in value && "text" in value) {
    const name = /** @type {import('./index.d.ts').RecognizedName} */(value);
    return `${leaf(name.kind)}: ${leaf(name.text)} (${String(name.start)}..${String(name.end)})`;
  }
  if ("text" in value && typeof value.text === "string") return leaf(value.text);
  return Object.entries(value).map(([name, entry]) => `${leaf(name)}=${printed(entry)}`).join("\t");
}
/** Project a native host row and its canonical saved representation once.
 * @param {import('./index.d.ts').AnyRow} value @param {string} canonical
 * @param {import('./index.d.ts').RecordInput} item @returns {import('./index.d.ts').AnyRow}
 */
export function result(value, canonical, item) {
  const decoded = /** @type {unknown} */(JSON.parse(canonical));
  const wire = /** @type {Record<string, unknown>} */(decoded);
  const {answer: raw, ...fields} = /** @type {Record<string, unknown>} */(/** @type {unknown} */(value));
  /** @type {unknown} */ const answer = value.function === "annotate" && raw && typeof raw === "object"
    ? Object.freeze(Object.fromEntries(Object.entries(/** @type {Record<string, unknown>} */(raw)).map(([name, child]) => [name,
      result(/** @type {import("./index.d.ts").AnyRow} */({position:value.position, ...(value.id === undefined ? {} : {id:value.id}), source:value.source, .../** @type {object} */(child)}), JSON.stringify(/** @type {Record<string, unknown>} */(wire.answer)[name]), item)]))) : raw;
  const text = value.status === "failed" ? `failed: ${value.error.code}: ${leaf(value.error.message)}`
    : value.status === "unsure" ? "unsure" : value.function === "annotate" && answer && typeof answer === "object"
      ? orderedEntries(answer).map(([name, child]) => `${leaf(name)}=${inspect(child)}`).join("\t")
      : value.function === "recognize" ? printed(wire.answer) : printed(answer);
  /** @type {Record<string, unknown>} */ const projected = {
    ...fields, item, answered: value.status === "answered", unsure: value.status === "unsure", failed: value.status === "failed",
    probability: "probability" in value ? value.probability : undefined,
    error: "error" in value ? value.error : undefined,
    detail: "detail" in value ? value.detail : undefined,
    get answer() {
      if (value.status === "unsure") throw new UnsureError(/** @type {Extract<Judgment, {status:'unsure'}>} */(/** @type {unknown} */(projected)));
      if (value.status === "failed") throw new FailedError([/** @type {Extract<Judgment, {status:'failed'}>} */(/** @type {unknown} */(projected))]);
      return answer;
    },
    throwIfFailed() {
      const row = /** @type {Judgment} */(/** @type {unknown} */(projected));
      const failure = firstFailure(row);
      if (failure) throw new FailedError([failure]);
      return projected;
    },
    toString() { return text; },
    [inspect.custom]() { return text; },
    /** @param {string} hint */
    [Symbol.toPrimitive](hint) {
      if (hint === "string") return text;
      throw new TypeError("read result.answer before comparing or converting a result");
    },
    toJSON() {
      const decoded = /** @type {unknown} */(JSON.parse(canonical));
      const saved = /** @type {Record<string, unknown>} */(decoded);
      if ("id" in saved && typeof value.id === "bigint") saved.id = /** @type {typeof JSON & {rawJSON(text: string): object}} */(JSON).rawJSON(value.id.toString());
      return saved;
    },
  };
  Object.defineProperty(projected, "answer", {enumerable: false});
  for (const name of ["throwIfFailed", "toString", "toJSON", inspect.custom, Symbol.toPrimitive]) {
    Object.defineProperty(projected, name, {enumerable: false});
  }
  return /** @type {import('./index.d.ts').AnyRow} */(Object.freeze(/** @type {unknown} */(projected)));
}
/** Promise of the one result; forgotten await cannot silently read no answer.
 * @template R @param {Promise<R>} pending @returns {Promise<R>}
 */
export function resultPromise(pending) {
  void pending.catch(() => {});
  for (const name of ["answer", "answered", "unsure", "failed", "probability", "error", "status", "detail", "item", "throwIfFailed"]) {
    void Object.defineProperty(pending, name, {get() { throw new TypeError("await the call before reading its result"); }});
  }
  return pending;
}
/** @template T @param {import('./index.d.ts').RowStream<import('./index.d.ts').AnyRow & {readonly item: T}>} source @returns {import('./index.d.ts').RowStream<T>} */
export function keptRecords(source) {
  async function* kept() {
    /** @type {Extract<import('./index.d.ts').AnyRow, {status:'failed'}>[]} */ const failures = [];
    try {
      for await (const row of source) {
        if (row.status === "failed") failures.push(row);
        else if (row.answer === true) yield row.item;
      }
      if (failures.length) throw new FailedError(failures);
    } finally { await source.return(); }
  }
  const iterator = kept();
  return {
    [Symbol.asyncIterator]() { return this; },
    next: () => iterator.next(),
    async return() { try { return await iterator.return(); } finally { await source.return(); } },
    async [Symbol.asyncDispose]() { await this.return(); },
    async toArray() { const result = []; for await (const item of iterator) result.push(item); return result; },
  };
}

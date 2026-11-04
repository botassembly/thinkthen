// @ts-check
import { ThinkThenError } from "./bridge.js";
/** @typedef {import('./index.d.ts').AnyRow} Row */
/** @typedef {import('./index.d.ts').Judgment} Judgment */
/** A judgment that cannot supply an answer at the chosen thresholds. */
export class UnsureError extends Error {
  /** @param {Extract<Judgment, {status:'unsure'}>} row */
  constructor(row) {
    super("The judgment is not sure at the chosen thresholds.");
    this.name = "UnsureError";
    this.row = row;
    this.probability = row.probability;
  }
}
/** Failed records retain the engine's code and message. */
export class RecordError extends ThinkThenError {
  /** @param {readonly Extract<Judgment, {status:'failed'}>[]} rows */
  constructor(rows) {
    const row = rows[0];
    if (!row) throw new TypeError("RecordError requires at least one failed row.");
    super(row.error);
    this.row = row;
    this.rows = Object.freeze([...rows]);
  }
}
/** @template {Judgment} R @param {R} row @returns {import('./index.d.ts').AnswerOf<R>} */
export function answer(row) {
  if (row.status === "unsure") throw new UnsureError(row);
  if (row.status === "failed") throw new RecordError([row]);
  return /** @type {import('./index.d.ts').AnswerOf<R>} */(row.answer);
}
/** @template {Row} R @param {Promise<R>} row @returns {import('./index.d.ts').AnswerPromise<R>} */
export function answerPromise(row) {
  const pending = row.then(answer);
  // Reading only row() is valid even when extracting the answer would reject.
  void pending.catch(() => {});
  return Object.assign(pending, { row: () => row });
}
/** @template T @param {import('./index.d.ts').RowStream<import('./index.d.ts').FilterItem<T>>} source @returns {import('./index.d.ts').RowStream<T>} */
export function keptRecords(source) {
  async function* kept() {
    /** @type {Extract<Row, {status:'failed'}>[]} */ const failures = [];
    try {
      for await (const item of source) {
        if (item.row.status === "failed") failures.push(item.row);
        else if (item.kept) yield item.item;
      }
      if (failures.length) throw new RecordError(failures);
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

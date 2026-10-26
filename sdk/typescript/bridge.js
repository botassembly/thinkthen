// @ts-check
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { ABI_VERSION, ABI_VERSION_EXPORT, ABI_EXPORTS, ABI_CODE, ABI_MESSAGE } from "./contract.js";
import { ERROR_CODES } from "./errors.js";
/** @typedef {import('./index.d.ts').ErrorCode} ErrorCode */
/** @typedef {(data: string, text: boolean, invalid?: boolean, idKind?: string, idValue?: string) => void} Reply */
/** @typedef {(error: Error | null, kind: string, data: string, reply: Reply) => void} Callback */
/** @typedef {{cancel(): void, close(): void, reference(): void, unreference(): void}} Operation */
/** @typedef {{start(request: string, callback: Callback): Operation, startFinite(request: string, callback: Callback): Operation, close(): void}} NativeClient */
/** @typedef {{Client: new (settings: string) => NativeClient, parseToolbox(json: string): string, serializeToolbox(json: string): string, rejectCancellation(): never, rejectSettingsConversion(): never, rejectQuestionConversion(): never}} Native */
export class ThinkThenError extends Error {
  /** @param {{code: ErrorCode, message: string, httpStatus?: number}} error @param {ErrorOptions} [options] */
  constructor(error, options) {
    super(error.message, options);
    this.name = new.target.name;
    this.code = error.code;
    if (error.httpStatus !== undefined) this.httpStatus = error.httpStatus;
  }
}
export class UsageError extends ThinkThenError {}
export class BackendError extends ThinkThenError {}
export class CancelledError extends ThinkThenError {}
/** @param {unknown} value @returns {ThinkThenError} */
export function engineError(value) {
  if (!value || typeof value !== "object" || !("code" in value) || !("message" in value)
    || !ERROR_CODES.includes(/** @type {ErrorCode} */(value.code)) || typeof value.message !== "string") {
    return new UsageError({ code: ABI_CODE, message: ABI_MESSAGE });
  }
  const code = /** @type {ErrorCode} */(value.code);
  const Kind = code.startsWith("usage.") ? UsageError : code === "local.cancelled" ? CancelledError
    : code.startsWith("backend.") ? BackendError : ThinkThenError;
  return new Kind({ code, message: value.message,
    ...("http_status" in value && typeof value.http_status === "number" ? {httpStatus: value.http_status} : {}) });
}
/** @param {unknown} error @returns {ThinkThenError} */
export function nativeError(error) {
  try { return engineError(/** @type {unknown} */(JSON.parse(error instanceof Error ? error.message : ""))); }
  catch { return new UsageError({ code: ABI_CODE, message: ABI_MESSAGE }); }
}
const require = createRequire(import.meta.url);
/** @type {Native | undefined} */
let cached;
/** @returns {Native} */
export function loadNative() {
  if (cached) return cached;
  /** @type {unknown} */ let loaded;
  try { loaded = require("./thinkthen.node"); }
  catch (error) { throw new ThinkThenError({ code: "local.io", message: `cannot load native engine ${fileURLToPath(new URL("./thinkthen.node", import.meta.url))}: ${error instanceof Error ? error.message : String(error)}` }); }
  /** @param {string} path @returns {unknown} */
  const member = path => {
    let value = loaded;
    for (const key of path.split(".")) {
      if (value === null || (typeof value !== "object" && typeof value !== "function")) return undefined;
      value = /** @type {unknown} */ (Reflect.get(value, key));
    }
    return value;
  };
  let compatible = false;
  try {
    const version = member(ABI_VERSION_EXPORT);
    if (typeof version === "function") {
      const actual = /** @type {unknown} */(Reflect.apply(version, loaded, []));
      compatible = typeof actual === "number" && actual === (actual >>> 0)
        && (actual >>> 16) === (ABI_VERSION >>> 16) && (actual & 0xffff) >= (ABI_VERSION & 0xffff)
        && ABI_EXPORTS.every(path => typeof member(path) === "function");
    }
  } catch { /* Refuse malformed modules before settings admission. */ }
  if (!compatible) throw new UsageError({ code: ABI_CODE, message: ABI_MESSAGE });
  cached = /** @type {Native} */(loaded);
  return cached;
}
/** @template T @param {() => T} value @param {() => never} reject @returns {T} */
export function conversion(value, reject) {
  try { return value(); }
  catch { try { return reject(); } catch (error) { throw nativeError(error); } }
}
/** @param {unknown} value @param {boolean} [record] @returns {{value: unknown, id?: bigint}} */
function jsonValue(value, record = false) {
  /** @type {Set<object>} */ const ancestors = new Set();
  /** @type {bigint | undefined} */ let id;
  /** @param {unknown} item @param {boolean} [root] @returns {unknown} */
  const visit = (item, root = false) => {
    if (item === null || typeof item === "string" || typeof item === "boolean") return item;
    if (typeof item === "number" && Number.isFinite(item)) return item;
    if (typeof item !== "object" || ancestors.has(item)) throw new TypeError();
    if (!Array.isArray(item) && Object.getPrototypeOf(item) !== Object.prototype && Object.getPrototypeOf(item) !== null) throw new TypeError();
    if (Object.getOwnPropertySymbols(item).some(key => Object.prototype.propertyIsEnumerable.call(item, key))) throw new TypeError();
    ancestors.add(item);
    /** @type {unknown} */ let result;
    if (Array.isArray(item)) {
      const children = [];
      const length = item.length;
      for (let index = 0; index < length; index++) children.push(visit(/** @type {unknown} */(item[index])));
      result = children;
    } else {
      /** @type {[string, unknown][]} */ const entries = [];
      for (const [key, child] of Object.entries(item)) {
        if (record && root && key === "id") {
          if (typeof child === "bigint") { id = child; continue; }
          if (typeof child === "number" && !Number.isSafeInteger(child)) throw new TypeError();
        }
        entries.push([key, visit(child)]);
      }
      result = Object.fromEntries(entries);
    }
    ancestors.delete(item);
    return result;
  };
  const cloned = visit(value, true);
  return {value: cloned, ...(id === undefined ? {} : {id})};
}
/** @param {unknown} value @returns {string} */
export function encode(value) { return JSON.stringify(jsonValue(value).value); }
/** @param {unknown} value @returns {unknown} */
export function hostValue(value) {
  if (Array.isArray(value)) return Object.freeze(value.map(hostValue));
  if (value === null || typeof value !== "object") return value;
  /** @type {Record<string, unknown>} */
  const fields = Object.fromEntries(Object.entries(value).map(([key, item]) => [key, hostValue(item)]));
  if (typeof fields.function === "string") {
    if (fields.error && typeof fields.error === "object" && "code" in fields.error
      && !ERROR_CODES.includes(/** @type {ErrorCode} */(fields.error.code))) throw engineError(null);
    if (fields.source && typeof fields.source === "object" && "question_identity" in fields.source) {
      const {question_identity, ...source} = fields.source;
      fields.source = Object.freeze({...source, questionIdentity: question_identity});
    }
    if (fields.error && typeof fields.error === "object" && "http_status" in fields.error) {
      const {http_status, ...error} = fields.error;
      fields.error = Object.freeze({...error, httpStatus: http_status});
    }
  }
  if (typeof fields.relation === "string" && fields.source !== null && typeof fields.source === "object"
    && fields.target !== null && typeof fields.target === "object" && "either" in fields) {
    fields.symmetric = fields.either;
    delete fields.either;
  }
  return Object.freeze(fields);
}
/** @param {unknown} value @returns {unknown} */
export function snapshot(value) { return hostSnapshot(JSON.parse(encode(value))); }
/** @param {unknown} value @returns {unknown} */
function hostSnapshot(value) {
  if (value !== null && typeof value === "object") {
    for (const item of Object.values(value)) hostSnapshot(item);
    Object.freeze(value);
  }
  return value;
}

/** @param {import('./index.d.ts').RecordInput} record @returns {{data: string, idKind?: string, idValue?: string}} */
export function encodeRecord(record) {
  const converted = jsonValue(record, true);
  return {data: JSON.stringify(converted.value), ...(converted.id === undefined ? {} : {idKind: "integer", idValue: converted.id.toString()})};
}

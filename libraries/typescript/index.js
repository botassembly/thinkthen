'use strict';
// The JavaScript face over the native door. This file shapes arguments into
// the question-file grammar, races each call against its AbortSignal, and
// raises the engine's failures as ThinkThenError. The engine owns every
// rule, retry, and send.

const native = require('./loader.js');

/** Each kind's code in the C door, 1 through 6. */
const CODES = { usage: 1, backend: 2, deadline: 3, local: 4, cancelled: 5, defect: 6 };

/** The one error class. `kind` is one of six words, `code` its C number, and
 * `retryable` says whether the same call may pass later. A failure never
 * reads as a value. */
class ThinkThenError extends Error {
  constructor(kind, message, retryable = false, options = undefined) {
    super(message, options);
    this.name = 'ThinkThenError';
    this.kind = Object.hasOwn(CODES, kind) ? kind : 'defect';
    this.code = CODES[this.kind];
    this.retryable = retryable === true;
  }
}

/** A decide answer's code in the C door. */
const YES = 1;
const NO = 0;
const UNSURE = 2;

/** YES, NO, or UNSURE for a decide answer: true, false, or null. */
function outcome(answer) {
  if (answer === true) return YES;
  if (answer === false) return NO;
  if (answer === null) return UNSURE;
  throw usageError('an outcome reads true, false, or null');
}

/** The failure of one annotate answer, `{ kind, cause }`, or null for any
 * value. null is unresolved, never a failure. */
function failed(member) {
  if (!isObject(member) || Object.keys(member).length !== 1 || !isObject(member.failed)) return null;
  return member.failed;
}

const usageError = (message) => new ThinkThenError('usage', message);
const freezeJson = (held) => {
  if (held && typeof held === 'object') {
    for (const value of Object.values(held)) freezeJson(value);
    Object.freeze(held);
  }
  return held;
};

function opened(envelope) {
  const parsed = JSON.parse(envelope);
  if (parsed.err) {
    const error = new ThinkThenError(parsed.err.kind, parsed.err.message, parsed.err.retryable);
    if (parsed.err.native_complete !== undefined) error.complete = require('./_complete.js').decode('CallError',parsed.err.native_complete);
    if (parsed.err.facts !== undefined) error.facts = freezeJson(parsed.err.facts);
    if (parsed.err.details !== undefined) error.details = freezeJson(parsed.err.details);
    throw error;
  }
  return parsed.ok;
}

const isQuestion = (value) => typeof value === 'function' && typeof value.__spec === 'string';
const isObject = (value) => value !== null && typeof value === 'object' && !Array.isArray(value);
const has = (held, key) => Object.prototype.hasOwnProperty.call(held, key);

// Inspect before stringify: JSON.stringify silently drops object members,
// changes array members to null, and throws bare TypeError for cycles/BigInt.
function jsonText(value, what) {
  const parents = new WeakSet();
  const check = (held) => {
    if (held === null || typeof held === 'string' || typeof held === 'boolean') return;
    if (typeof held === 'number' && Number.isFinite(held)) return;
    if (typeof held !== 'object' || (!Array.isArray(held) && !isObject(held))) throw usageError(`${what} is JSON`);
    if (parents.has(held)) throw usageError(`${what} is JSON without a cycle`);
    if (!Array.isArray(held) && ![Object.prototype, null].includes(Object.getPrototypeOf(held))) throw usageError(`${what} is JSON`);
    parents.add(held);
    if (Reflect.ownKeys(held).some((key) => typeof key === 'symbol' || (Array.isArray(held) && key !== 'length' && !/^(0|[1-9]\d*)$/.test(key)))) throw usageError(`${what} is JSON`);
    for (const entry of Array.isArray(held) ? held : Object.values(held)) check(entry);
    parents.delete(held);
  };
  try {
    check(value);
    return JSON.stringify(value);
  } catch (error) {
    if (error instanceof ThinkThenError) throw error;
    throw usageError(`${what} is JSON`);
  }
}

// The question file's JSON for a question. A band [low, high] becomes the
// file's "low:high"; everything else passes through, and the engine rules on it.
function specOf(questionOrSpec) {
  let spec;
  if (isQuestion(questionOrSpec)) return JSON.parse(questionOrSpec.__spec);
  if (typeof questionOrSpec === 'string') spec = { decide: questionOrSpec };
  else if (isObject(questionOrSpec)) {
    jsonText(questionOrSpec, 'the question');
    spec = { ...questionOrSpec };
  }
  else throw usageError('the question is missing: text, file keys, or a question value');
  if (!['decide', 'choose', 'score', 'tag'].some((kind) => has(spec, kind))) {
    throw usageError('the question names no decide, choose, score, or tag');
  }
  if (Array.isArray(spec.threshold)) {
    const [low, high] = spec.threshold;
    if (spec.threshold.length !== 2 || typeof low !== 'number' || typeof high !== 'number') {
      throw usageError('a band threshold is [low, high], two numbers');
    }
    spec.threshold = `${low}:${high}`;
  }
  return spec;
}

/** Build a question once from its parts and pass it to any verb. A function,
 * so no spread or stringify turns it back into text by accident. */
function question(spec) {
  if (!isObject(spec)) throw usageError('question takes an object: { decide | choose | score | tag, ... }');
  return builtQuestion(jsonText(specOf(spec), 'the question'));
}

function builtQuestion(source) {
  const held = () => {
    throw usageError('a question value is asked, not called');
  };
  held.__spec = source;
  return Object.freeze(held);
}

/** Load one named local question; ordinary strings remain literal questions. */
function questionFile(path) {
  return builtQuestion(opened(native.questionFile(filePath(path, 'questionFile'))));
}

function planFile(path, verb) {
  return opened(native.planFile(filePath(path, `${verb} file`), verb));
}

function filePath(path, label) {
  if (typeof path !== 'string' || path.length === 0 || !path.isWellFormed() || path.includes('\0')) {
    throw usageError(`${label} takes a valid text path`);
  }
  return path;
}

// The ruled shape of 2026-09-21: the last object carries the question's
// inputs and the call's options together, and any other key is refused.
const CALL_KEYS = new Set(['signal', 'deadlineMs', 'batch', 'context']);
const QUESTION_KEYS = {
  choose: ['options'],
  choose_many: ['options'],
  score: ['levels'],
  score_many: ['levels'],
  tag: ['labels'],
  tag_many: ['labels'],
  rank: ['top'],
  find: ['none'],
  recognize: ['kinds', 'relations', 'threshold', 'relationThreshold', 'file'],
  relate: ['relations', 'either', 'threshold', 'file'],
};

function splitLast(verb, last) {
  const inputs = {};
  const call = {};
  if (last === undefined || last === null) return { inputs, call };
  if (!isObject(last) || ![Object.prototype, null].includes(Object.getPrototypeOf(last))) throw usageError(`${verb} takes one options object last: question inputs and call options together`);
  if (Reflect.ownKeys(last).some((key) => typeof key === 'symbol')) throw usageError(`${verb} takes string option keys`);
  for (const [key, value] of Object.entries(last)) {
    if (CALL_KEYS.has(key)) {
      const allowed = key === 'batch' ? ['decide_many', 'choose_many', 'score_many', 'tag_many', 'filter', 'rank', 'annotate', 'plan'].includes(verb)
        : key === 'context' ? ['decide_many', 'choose_many', 'score_many', 'tag_many', 'filter', 'rank', 'plan'].includes(verb) : verb !== 'plan';
      if (!allowed) throw usageError(`options.${key} is not a ${verb} key`);
      call[key] = value;
    }
    else if ((QUESTION_KEYS[verb] ?? []).includes(key)) inputs[key] = value;
    else throw usageError(`options.${key} is not a ${verb} key`);
  }
  if (has(inputs, 'top') && (!Number.isInteger(inputs.top) || inputs.top < 1)) {
    throw usageError('options.top is a positive whole number');
  }
  return { inputs, call };
}

// The question a verb asks: a bare string with the last object's inputs, or
// a question value or spec that carries its own.
function specFrom(verb, questionOrSpec, inputs) {
  const listed = { choose: 'options', choose_many: 'options', tag: 'labels', tag_many: 'labels', score: 'levels', score_many: 'levels' }[verb];
  if (typeof questionOrSpec !== 'string') {
    for (const key of ['options', 'labels', 'levels']) {
      if (has(inputs, key)) throw usageError(`${verb}: a question value carries its own ${key}; the last object holds call options and top`);
    }
    return isQuestion(questionOrSpec) ? questionOrSpec.__spec : jsonText(specOf(questionOrSpec), 'the question');
  }
  if (!listed) return jsonText(specOf(questionOrSpec), 'the question');
  if (!has(inputs, listed)) throw usageError(`${verb} takes its ${listed} in the last object: { ${listed} }`);
  return jsonText({ [verb.replace('_many', '')]: questionOrSpec, [listed]: inputs[listed] }, 'the question');
}

// `rank` and `find` read the question's text alone and no rule.
function textFrom(verb, questionOrSpec) {
  const spec = typeof questionOrSpec === 'string' ? { decide: questionOrSpec } : specOf(questionOrSpec);
  const extra = Object.keys(spec).find((key) => key !== 'decide');
  if (extra !== undefined) throw usageError(`${verb} takes a decide question with no ${extra}`);
  if (typeof spec.decide !== 'string') throw usageError(`${verb} takes a decide question with text`);
  return spec.decide;
}

function checkText(text, what = 'the evidence') {
  if (typeof text !== 'string') throw usageError(`${what} is text`);
  if (!text.isWellFormed()) throw usageError(`${what} holds a lone surrogate`);
  return text;
}

function checkRecords(records) {
  if (!Array.isArray(records)) throw usageError('records is an array of strings');
  records.forEach((held, at) => checkText(held, `record ${at}`));
  return JSON.stringify(records);
}

function callOptions(call) {
  const { signal, deadlineMs, batch, context } = call;
  if (signal !== undefined && !(signal instanceof AbortSignal)) throw usageError('options.signal is an AbortSignal');
  if (deadlineMs !== undefined && deadlineMs !== null && typeof deadlineMs !== 'number') {
    throw usageError('options.deadlineMs is a number of milliseconds; no deadline is spelled null, left out, or -1');
  }
  if (has(call, 'batch') && (batch !== 'max' && (!Number.isSafeInteger(batch) || batch < 1))) {
    throw usageError('options.batch is max or a positive whole number');
  }
  if (has(call, 'context') && (typeof context !== 'string' || !context.isWellFormed() || !context.trim())) {
    throw usageError('options.context is nonblank text');
  }
  return { signal, deadlineMs: deadlineMs ?? null, batch: batch === undefined ? null : jsonText(batch, 'options.batch'), context: context ?? null };
}

const aborted = (signal) =>
  new ThinkThenError('cancelled', 'the call was cancelled by its AbortSignal', false, { cause: signal.reason });

const frozenCall = (held) => Object.freeze({ value: held.value, facts: freezeJson(held.facts),
  details: freezeJson(held.details) });
const mapped = (held, value) => frozenCall({ ...held, value });

// An early abort rejects promptly but retains one unreferenced native
// callback. A caller that observes completion re-references it while waiting.
function invoke(engine, op, spec, payload, call) {
  const { signal, deadlineMs, batch, context } = callOptions(call);
  if (signal?.aborted) return Promise.reject(aborted(signal));
  return new Promise((resolve, reject) => {
    let settled = false;
    let handle = null;
    let finalReport;
    let finishFinal;
    const final = new Promise((done) => { finishFinal = done; });
    const onAbort = () => {
      if (settled) return;
      settled = true;
      signal.removeEventListener('abort', onAbort);
      const stopped = aborted(signal);
      stopped.completion = Object.freeze({ wait() {
        if (finalReport !== undefined) return Promise.resolve(finalReport);
        handle.wait();
        return final;
      } });
      handle.stop();
      reject(stopped);
    };
    handle = native.call(engine, op, spec, payload, deadlineMs, batch, context, (envelope) => {
      signal?.removeEventListener('abort', onAbort);
      try {
        finalReport = { ok: frozenCall(opened(envelope)) };
      } catch (error) {
        finalReport = { err: error };
      }
      finishFinal(finalReport);
      handle.detach();
      if (settled) return;
      settled = true;
      if (finalReport.err) reject(finalReport.err);
      else resolve(finalReport.ok);
    });
    signal?.addEventListener('abort', onAbort, { once: true });
  });
}

// JavaScript indexes UTF-16 units and the engine counts Unicode scalar values.
function utf16(text) {
  const units = [0];
  for (const ch of text) units.push(units[units.length - 1] + ch.length);
  // The spread keeps the engine's key order: text, start, end, length, kind, strength.
  return (held) => ({ ...held, start: units[held.start], end: units[held.end], length: units[held.end] - units[held.start] });
}

function recognizeSpec(inputs) {
  if (has(inputs, 'file')) {
    if (Object.keys(inputs).length !== 1) throw usageError('recognize file takes no inline plan options');
    return planFile(inputs.file, 'recognize');
  }
  const kinds = has(inputs, 'kinds') ? inputs.kinds : [];
  if (!Array.isArray(kinds) && !isObject(kinds)) throw usageError('options.kinds is an array or object');
  if (Array.isArray(kinds) && !kinds.every((kind) => typeof kind === 'string')) throw usageError('options.kinds is an array of names');
  const recognize = {
    kinds: Array.isArray(kinds) ? Object.fromEntries(kinds.map((kind) => [kind, null])) : kinds,
  };
  if (Array.isArray(inputs.relations)) recognize.relations = inputs.relations;
  else if (has(inputs, 'relations')) {
    if (!isObject(inputs.relations)) throw usageError('options.relations is an object: { name: [source, target] }');
    recognize.relations = Object.entries(inputs.relations).map(([name, ends]) => {
      if (!Array.isArray(ends) || ends.length !== 2) throw usageError(`options.relations.${name} is [source, target]`);
      return { name, source: ends[0], target: ends[1] };
    });
  }
  const spec = { version: 1, recognize };
  if (has(inputs, 'threshold')) spec.threshold = inputs.threshold;
  if (has(inputs, 'relationThreshold')) spec.relation_threshold = inputs.relationThreshold;
  return jsonText(spec, 'the recognition question');
}

// A rule is a name, which relates any kind to any kind, or "name=source:target".
function relateSpec(inputs) {
  if (has(inputs, 'file')) {
    if (Object.keys(inputs).length !== 1) throw usageError('relate file takes no inline plan options');
    return planFile(inputs.file, 'relate');
  }
  const names = (key) => {
    const held = has(inputs, key) ? inputs[key] : [];
    if (!Array.isArray(held) || !held.every((name) => typeof name === 'string')) throw usageError(`options.${key} is an array of relation names`);
    return held;
  };
  const either = names('either');
  const relations = names('relations').map((rule) => {
    const [name, ends = '*:*'] = rule.split('=');
    const [source, target] = ends.split(':');
    return { name, source, target, either: either.includes(name) };
  });
  const spec = { version: 1, relate: { relations } };
  if (has(inputs, 'threshold')) spec.threshold = inputs.threshold;
  return jsonText(spec, 'the relation question');
}

// A name recognize found carries text in place of name; name wins when both are there.
function entityPair(held, at) {
  const [name, kind] = Array.isArray(held) ? held : [held?.name ?? held?.text, held?.kind];
  if (typeof name !== 'string' || typeof kind !== 'string') throw usageError(`entity ${at} is { name, kind } or [name, kind]`);
  return [checkText(name, `entity ${at}`), checkText(kind, `entity ${at}`)];
}

// Each verb takes the engine first: null is the default engine.
const verbs = {
  files(engine, question, paths, reader = {}, call = {}) {
    if (!isObject(question) || !isObject(reader)) throw usageError('files takes a question object and reader options');
    if (Object.keys(reader).some(key => key !== 'unit' && key !== 'window')) throw usageError('reader takes unit and optional window');
    const many = ['decide','choose','tag','score','filter','rank','annotate'].some(verb => has(question, verb));
    if (has(call, 'batch') && !many) throw usageError('this source verb takes no batch');
    if (has(call, 'context') && (!many || has(question, 'annotate'))) throw usageError('this source verb takes no shared context');
    const source = { paths: typeof paths === 'string' ? [paths] : paths, ...reader };
    return invoke(engine, 'files', jsonText(question, 'files question'), jsonText(source, 'source'), call)
      .then((done) => {
        const value = Array.isArray(done.value) && question.recognize ? done.value.map((row) => {
          const offsets = utf16(row.record);
          const located = { ...row.value, entities: row.value.entities.map(offsets) };
          if (row.value.relations !== undefined) located.relations = row.value.relations.map(
            (edge) => ({ ...edge, source: offsets(edge.source), target: offsets(edge.target) }));
          return { ...row, value: located };
        }) : done.value;
        return mapped(done, value);
      });
  },
  async decide(engine, asked, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('decide', last);
    return invoke(engine, 'decide', specFrom('decide', asked, inputs), text, call);
  },
  async decide_many(engine, asked, records, last) {
    const { inputs, call } = splitLast('decide_many', last);
    return invoke(engine, 'decide_many', specFrom('decide_many', asked, inputs), checkRecords(records), call);
  },
  async choose_many(engine, asked, records, last) {
    const { inputs, call } = splitLast('choose_many', last);
    return invoke(engine, 'choose_many', specFrom('choose_many', asked, inputs), checkRecords(records), call);
  },
  async score_many(engine, asked, records, last) {
    const { inputs, call } = splitLast('score_many', last);
    return invoke(engine, 'score_many', specFrom('score_many', asked, inputs), checkRecords(records), call);
  },
  async tag_many(engine, asked, records, last) {
    const { inputs, call } = splitLast('tag_many', last);
    return invoke(engine, 'tag_many', specFrom('tag_many', asked, inputs), checkRecords(records), call);
  },
  async choose(engine, asked, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('choose', last);
    return invoke(engine, 'choose', specFrom('choose', asked, inputs), text, call);
  },
  async score(engine, asked, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('score', last);
    return invoke(engine, 'score', specFrom('score', asked, inputs), text, call);
  },
  async tag(engine, asked, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('tag', last);
    return invoke(engine, 'tag', specFrom('tag', asked, inputs), text, call);
  },
  async filter(engine, asked, records, last) {
    const { inputs, call } = splitLast('filter', last);
    const kept = await invoke(engine, 'filter', specFrom('filter', asked, inputs), checkRecords(records), call);
    return mapped(kept, kept.value.map((at) => records[at]));
  },
  async rank(engine, asked, records, last) {
    const { inputs, call } = splitLast('rank', last);
    const ranked = await invoke(engine, 'rank', textFrom('rank', asked), checkRecords(records), call);
    const rows = ranked.value.map(({ index, probability }) => ({ index, record: records[index], probability }));
    return mapped(ranked, inputs.top === undefined ? rows : rows.slice(0, inputs.top));
  },
  async find(engine, asked, units, last) {
    const { inputs, call } = splitLast('find', last);
    if (has(inputs, 'none') && typeof inputs.none !== 'boolean') throw usageError('options.none is true or false');
    const op = inputs.none ? 'find_none' : 'find';
    const found = await invoke(engine, op, textFrom('find', asked), checkRecords(units), call);
    return mapped(found, found.value === null ? null : { index: found.value.index, unit: units[found.value.index], probability: found.value.probability });
  },
  async annotate(engine, set, records, last) {
    const { call } = splitLast('annotate', last);
    const spec = isObject(set) ? jsonText(set, 'the question set') : set;
    if (typeof spec !== 'string' || spec.length === 0) throw usageError('annotate takes a question set: a file path, the set JSON, or a set object');
    return invoke(engine, 'annotate', spec, checkRecords(records), call);
  },
  async details(engine, asked, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('details', last);
    return invoke(engine, 'details', specFrom('details', asked, inputs), text, call);
  },
  async recognize(engine, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('recognize', last);
    const found = await invoke(engine, 'recognize', recognizeSpec(inputs), text, call);
    const offsets = utf16(text);
    const shaped = { entities: found.value.entities.map(offsets) };
    if (found.value.relations !== undefined) {
      shaped.relations = found.value.relations.map((held) => ({ ...held, source: offsets(held.source), target: offsets(held.target) }));
    }
    return mapped(found, shaped);
  },
  async relate(engine, entities, last) {
    if (!Array.isArray(entities)) throw usageError('relate takes an array of entities');
    const { inputs, call } = splitLast('relate', last);
    return invoke(engine, 'relate', relateSpec(inputs), jsonText(entities.map(entityPair), 'the entities'), call);
  },
  // A preview, so it answers at once: the result schema's plan object. It
  // reads no key and no cache and sends nothing.
  plan(engine, asked, records, last) {
    const { inputs, call } = splitLast('plan', last);
    const { batch, context } = callOptions(call);
    return freezeJson(opened(native.plan(engine, specFrom('plan', asked, inputs), checkRecords(records), batch, context)));
  },
};

/** This process's totals: requests sent, cache answers, and tokens. */
const usage = () => opened(native.usage(null));

/** An engine with its own settings. The address, key, and cache start from
 * the environment, and each given option overrides one of them. */
class Engine {
  #native;

  constructor(options = {}) {
    if (!isObject(options)) throw usageError('new Engine takes one options object');
    try {
      if (has(options, 'batch') && options.batch !== 'max' && (!Number.isSafeInteger(options.batch) || options.batch < 1)) {
        throw usageError('options.batch is max or a positive whole number');
      }
      this.#native = native.engine(jsonText(options, 'new Engine options'));
    } catch (error) {
      if (String(error.message).startsWith('{"err"')) opened(error.message);
      throw error;
    }
  }

  usage() {
    return opened(native.usage(this.#native));
  }

  get complete() {
    return new (require('./complete.js').Functions)((request, controls) => invoke(this.#native, 'complete', null, request, controls), (request, controls) => completeBatch(this.#native, request, controls), opened);
  }

  static {
    for (const [name, verb] of Object.entries(verbs)) {
      Object.defineProperty(this.prototype, name, {
        value(...args) {
          return verb(this.#native, ...args);
        },
        writable: true,
        configurable: true,
      });
    }
  }
}

function completeBatch(engine, request, controls) {
  const { deadlineMs, context } = callOptions(controls);
  try { return native.completeBatch(engine, request, deadlineMs, context); }
  catch (error) { return opened(error.message); }
}
const complete = new (require('./complete.js').Functions)((request, controls) => invoke(null, 'complete', null, request, controls), (request, controls) => completeBatch(null, request, controls), opened);
const exported = { complete, ThinkThenError, Engine, question, questionFile, usage, failed, outcome, YES, NO, UNSURE };
for (const [name, verb] of Object.entries(verbs)) exported[name] = (...args) => verb(null, ...args);

module.exports = exported;

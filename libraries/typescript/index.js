'use strict';
// The JavaScript face over the native door. This file shapes arguments into
// the question-file grammar, races each call against its AbortSignal, and
// raises the engine's failures as ThinkThenError. The engine owns every
// rule, retry, and send.

const native = require('./loader.js');

const KINDS = ['usage', 'backend', 'local', 'cancelled', 'deadline', 'defect'];

/** The one error class. `kind` is one of six words, and `retryable` says
 * whether the same call may pass later. A failure never reads as a value. */
class ThinkThenError extends Error {
  constructor(kind, message, retryable = false, options = undefined) {
    super(message, options);
    this.name = 'ThinkThenError';
    this.kind = KINDS.includes(kind) ? kind : 'defect';
    this.retryable = retryable === true;
  }
}

const usageError = (message) => new ThinkThenError('usage', message);

function opened(envelope) {
  const parsed = JSON.parse(envelope);
  if (parsed.err) throw new ThinkThenError(parsed.err.kind, parsed.err.message, parsed.err.retryable);
  return parsed.ok;
}

const isQuestion = (value) => typeof value === 'function' && typeof value.__spec === 'string';
const isObject = (value) => value !== null && typeof value === 'object' && !Array.isArray(value);
const has = (held, key) => Object.prototype.hasOwnProperty.call(held, key);

// The question file's JSON for a question. A band [low, high] becomes the
// file's "low:high"; everything else passes through, and the engine rules on it.
function specOf(questionOrSpec) {
  let spec;
  if (isQuestion(questionOrSpec)) return JSON.parse(questionOrSpec.__spec);
  if (typeof questionOrSpec === 'string') spec = { decide: questionOrSpec };
  else if (isObject(questionOrSpec)) spec = { ...questionOrSpec };
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
  const held = () => {
    throw usageError('a question value is asked, not called');
  };
  held.__spec = JSON.stringify(specOf(spec));
  return Object.freeze(held);
}

// The ruled shape of 2026-09-21: the last object carries the question's
// inputs and the call's options together, and any other key is refused.
const CALL_KEYS = new Set(['signal', 'deadlineMs']);
const QUESTION_KEYS = {
  choose: ['options'],
  score: ['levels'],
  tag: ['labels'],
  rank: ['top'],
  find: ['none'],
  recognize: ['kinds', 'relations', 'threshold', 'relationThreshold'],
  relate: ['relations', 'either', 'threshold'],
};

function splitLast(verb, last) {
  const inputs = {};
  const call = {};
  if (last === undefined || last === null) return { inputs, call };
  if (!isObject(last)) throw usageError(`${verb} takes one options object last: question inputs and call options together`);
  for (const [key, value] of Object.entries(last)) {
    if (CALL_KEYS.has(key)) call[key] = value;
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
  const listed = { choose: 'options', tag: 'labels', score: 'levels' }[verb];
  if (typeof questionOrSpec !== 'string') {
    for (const key of ['options', 'labels', 'levels']) {
      if (has(inputs, key)) throw usageError(`${verb}: a question value carries its own ${key}; the last object holds call options and top`);
    }
    return JSON.stringify(specOf(questionOrSpec));
  }
  if (!listed) return JSON.stringify(specOf(questionOrSpec));
  if (!has(inputs, listed)) throw usageError(`${verb} takes its ${listed} in the last object: { ${listed} }`);
  return JSON.stringify({ [verb]: questionOrSpec, [listed]: inputs[listed] });
}

// `rank` and `find` read the question's text alone and no rule.
function textFrom(verb, questionOrSpec) {
  const spec = typeof questionOrSpec === 'string' ? { decide: questionOrSpec } : specOf(questionOrSpec);
  const extra = Object.keys(spec).find((key) => key !== 'decide');
  if (extra !== undefined) throw usageError(`${verb} takes a decide question with no ${extra}`);
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

function callOptions({ signal, deadlineMs }) {
  if (signal !== undefined && !(signal instanceof AbortSignal)) throw usageError('options.signal is an AbortSignal');
  if (deadlineMs !== undefined && deadlineMs !== null && typeof deadlineMs !== 'number') {
    throw usageError('options.deadlineMs is a number of milliseconds; no deadline is spelled null, left out, or -1');
  }
  return { signal, deadlineMs: deadlineMs ?? null };
}

const aborted = (signal) =>
  new ThinkThenError('cancelled', 'the call was cancelled by its AbortSignal', false, { cause: signal.reason });

// One call on its own worker thread, raced against its AbortSignal. The
// handle is detached in the same step that settles the promise, so Node may
// exit after any settle; a late envelope is dropped in Rust.
function invoke(engine, op, spec, payload, call) {
  const { signal, deadlineMs } = callOptions(call);
  if (signal?.aborted) return Promise.reject(aborted(signal));
  return new Promise((resolve, reject) => {
    let settled = false;
    let handle = null;
    const onAbort = () => {
      if (settled) return;
      settled = true;
      handle.detach();
      reject(aborted(signal));
    };
    handle = native.call(engine, op, spec, payload, deadlineMs, (envelope) => {
      if (settled) return;
      settled = true;
      signal?.removeEventListener('abort', onAbort);
      handle.detach();
      try {
        resolve(opened(envelope));
      } catch (error) {
        reject(error);
      }
    });
    signal?.addEventListener('abort', onAbort, { once: true });
  });
}

// JavaScript indexes UTF-16 units and the engine counts Unicode scalar values.
function utf16(text) {
  const units = [0];
  for (const ch of text) units.push(units[units.length - 1] + ch.length);
  return (held) => ({ ...held, start: units[held.start], end: units[held.end] });
}

function recognizeSpec(inputs) {
  const kinds = inputs.kinds ?? ['person', 'organization', 'place'];
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
  return JSON.stringify(spec);
}

// A rule is a name, which relates any kind to any kind, or "name=source:target".
function relateSpec(inputs) {
  const names = (key) => {
    const held = inputs[key] ?? [];
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
  return JSON.stringify(spec);
}

function entityPair(held, at) {
  const [name, kind] = Array.isArray(held) ? held : [held?.name, held?.kind];
  if (typeof name !== 'string' || typeof kind !== 'string') throw usageError(`entity ${at} is { name, kind } or [name, kind]`);
  return [checkText(name, `entity ${at}`), checkText(kind, `entity ${at}`)];
}

// Each verb takes the engine first: null is the default engine.
const verbs = {
  async decide(engine, asked, text, last) {
    checkText(text);
    const { inputs, call } = splitLast('decide', last);
    return invoke(engine, 'decide', specFrom('decide', asked, inputs), text, call);
  },
  async decide_many(engine, asked, records, last) {
    const { inputs, call } = splitLast('decide_many', last);
    return invoke(engine, 'decide_many', specFrom('decide_many', asked, inputs), checkRecords(records), call);
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
    return kept.map((at) => records[at]);
  },
  async rank(engine, asked, records, last) {
    const { inputs, call } = splitLast('rank', last);
    const ranked = await invoke(engine, 'rank', textFrom('rank', asked), checkRecords(records), call);
    const rows = ranked.map(({ index, probability }) => ({ index, record: records[index], probability }));
    return inputs.top === undefined ? rows : rows.slice(0, inputs.top);
  },
  async find(engine, asked, units, last) {
    const { inputs, call } = splitLast('find', last);
    if (has(inputs, 'none') && typeof inputs.none !== 'boolean') throw usageError('options.none is true or false');
    const op = inputs.none ? 'find_none' : 'find';
    const found = await invoke(engine, op, textFrom('find', asked), checkRecords(units), call);
    return found === null ? null : { index: found.index, unit: units[found.index], probability: found.probability };
  },
  async annotate(engine, set, records, last) {
    const { call } = splitLast('annotate', last);
    const spec = isObject(set) ? JSON.stringify(set) : set;
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
    const shaped = { entities: found.entities.map(offsets) };
    if (found.relations !== undefined) {
      shaped.relations = found.relations.map((held) => ({ ...held, source: offsets(held.source), target: offsets(held.target) }));
    }
    return shaped;
  },
  async relate(engine, entities, last) {
    if (!Array.isArray(entities)) throw usageError('relate takes an array of entities');
    const { inputs, call } = splitLast('relate', last);
    return invoke(engine, 'relate', relateSpec(inputs), JSON.stringify(entities.map(entityPair)), call);
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
      this.#native = native.engine(JSON.stringify(options));
    } catch (error) {
      if (String(error.message).startsWith('{"err"')) opened(error.message);
      throw error;
    }
  }

  usage() {
    return opened(native.usage(this.#native));
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

const exported = { ThinkThenError, Engine, question, usage };
for (const [name, verb] of Object.entries(verbs)) exported[name] = (...args) => verb(null, ...args);

module.exports = exported;

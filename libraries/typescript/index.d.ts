// The typed face of `thinkthen` for Node. Every verb is async, `null` is
// unsure, a list crosses into the engine once, an `AbortSignal` cancels a
// call at once, and `deadlineMs` bounds a whole call.

/** A cut threshold: the probability at or above which the answer is yes. */
export type Cut = number;

/** A band threshold: unsure between the two numbers. */
export type Band = [low: number, high: number];

export interface DecideSpec {
  decide: string;
  threshold?: Cut | Band;
  model?: string;
}

export interface ChooseSpec {
  choose: string;
  options: readonly string[];
  threshold?: Cut;
  model?: string;
}

export interface ScoreSpec {
  score: string;
  levels: readonly string[];
  model?: string;
}

export interface TagSpec {
  tag: string;
  labels: readonly string[];
  threshold?: Cut;
  model?: string;
}

export type QuestionSpec = DecideSpec | ChooseSpec | ScoreSpec | TagSpec;

/** A question value, built by {@link question} and passed to any verb. */
export type Question = (() => never) & { readonly __spec: string };

/** What every verb takes in its last object. */
export interface CallOptions {
  /** Cancels the call at once: the promise rejects with `cancelled`, no new
   * request starts, and a request already sent finishes. */
  signal?: AbortSignal;
  /** Bounds the whole call in milliseconds. `0` is spent, so the call sends
   * nothing and rejects with `deadline`. No deadline is spelled null, left out, or -1.
   * Any other value is a whole number of milliseconds up to 4294967295000;
   * a fraction, a NaN, or a value out of range rejects with `usage`. */
  deadlineMs?: number | null;
}

export interface ChooseOptions extends CallOptions {
  options: readonly string[];
}

export interface TagOptions extends CallOptions {
  labels: readonly string[];
}

export interface ScoreOptions extends CallOptions {
  levels: readonly string[];
}

export interface RankOptions extends CallOptions {
  /** Keep the first `top` of the ordered result. */
  top?: number;
}

export interface FindOptions extends CallOptions {
  /** Offer a none candidate, as `find --none` does, so the answer may be `null`. */
  none?: boolean;
}

/** `true`, `false`, or `null` when the answer is unsure. */
export type Answer = boolean | null;

/** One judgment as the command's `--details` prints it: `thinkthen.result/1`. */
export interface Details {
  schema: 'thinkthen.result/1';
  value: Answer | string | number | string[] | null;
  question: Record<string, unknown>;
  answer: {
    kind: 'yes_no' | 'choice' | 'score' | 'tag';
    probability?: number;
    probabilities?: Record<string, number>;
    pick?: string | null;
    level?: string;
    confidence?: number;
  };
  threshold?: number | string;
  meta: {
    tool: string;
    question_sha256: string;
    url: string;
    model: string;
    requests_sent: number;
    cached: boolean;
    requests: string[];
    failed_questions: number;
    usage?: { input_tokens: number; output_tokens: number };
  };
}

/** One record's place and probability of yes, most likely first. */
export interface Ranked {
  index: number;
  record: string;
  probability: number;
}

/** The unit the backend selected, by its index in the input. */
export interface Found {
  index: number;
  unit: string;
  probability: number;
}

/** Why one question failed while its neighbours answered. */
export type FailureCause =
  | 'missing_answer'
  | 'wrong_kind'
  | 'missing_probability'
  | 'invalid_probability'
  | 'invalid_distribution'
  | 'unexpected_probability';

/** The failed-question marker. It is never `null`, which means unsure. */
export interface FailedField {
  failed: { kind: 'backend'; cause: FailureCause };
}

/** One annotated value: an answer, a pick, a position, labels, or the marker. */
export type AnnotatedField = Answer | string | number | string[] | FailedField;

/** One record's values by question name, in the set's order. */
export type AnnotatedRow = Record<string, AnnotatedField>;

/** A question set: a file path, the set's JSON text, or the set object. */
export type QuestionSet = string | { version: 1; questions: Record<string, QuestionSpec> };

/** One name `recognize` found. `text.slice(start, end)` is the name, since
 * `start` and `end` count UTF-16 units. `strength` is the least word
 * probability behind the name times the mean kind probability. */
export interface RecognizedEntity {
  name: string;
  kind: string;
  start: number;
  end: number;
  strength: number;
}

export interface RecognizedRelation {
  relation: string;
  source: RecognizedEntity;
  target: RecognizedEntity;
  probability: number;
}

/** The names in one text, and their relations when rules were given. */
export interface Recognized {
  entities: RecognizedEntity[];
  relations?: RecognizedRelation[];
}

export interface RecognizeOptions extends CallOptions {
  /** Kind words, or kind words with their descriptions. */
  kinds?: readonly string[] | Record<string, string | null>;
  /** Each rule's ends, as kind words or the any-kind end "*", or the file's rule list. */
  relations?:
    | Record<string, readonly [source: string, target: string]>
    | readonly { name: string; source: string; target: string; reads?: string; either?: boolean }[];
  threshold?: number;
  relationThreshold?: number;
}

/** An entity `relate` reads. */
export type Entity = { name: string; kind: string } | readonly [name: string, kind: string];

export interface Edge {
  relation: string;
  source: { name: string; kind: string };
  target: { name: string; kind: string };
  probability: number;
}

export interface RelateOptions extends CallOptions {
  /** Rule names, each `name` or `name=source:target`. */
  relations?: readonly string[];
  /** The rule names that hold both ways. */
  either?: readonly string[];
  threshold?: number;
}

/** This process's totals. Failed calls and retries count, and nothing resets them. */
export interface Usage {
  requests_sent: number;
  cache_answers: number;
  input_tokens: number;
  output_tokens: number;
}

/** The engine's failure, its kind, and whether the same call may pass later. */
export class ThinkThenError extends Error {
  kind: 'usage' | 'backend' | 'local' | 'cancelled' | 'deadline' | 'defect';
  retryable: boolean;
}

/** An engine's settings. Each given key overrides what the environment set. */
export interface EngineOptions {
  baseUrl?: string;
  model?: string;
  /** The most requests in flight at once, 1 through 32, for this loaded copy of the engine. */
  throttle?: number;
  /** Refuse a call over more records than this. */
  maxRequests?: number;
  /** `false` reads and writes no cache; a string names the cache folder. */
  cache?: false | string;
  cacheBytes?: number;
}

export interface Verbs {
  decide(question: string | Question | DecideSpec, text: string, options?: CallOptions): Promise<Answer>;
  decide_many(question: string | Question | DecideSpec, records: readonly string[], options?: CallOptions): Promise<Answer[]>;
  choose(question: string, text: string, options: ChooseOptions): Promise<string | null>;
  choose(question: ChooseSpec | Question, text: string, options?: CallOptions): Promise<string | null>;
  score(question: string, text: string, options: ScoreOptions): Promise<number>;
  score(question: ScoreSpec | Question, text: string, options?: CallOptions): Promise<number>;
  tag(question: string, text: string, options: TagOptions): Promise<string[]>;
  tag(question: TagSpec | Question, text: string, options?: CallOptions): Promise<string[]>;
  filter(question: string | Question | DecideSpec, records: readonly string[], options?: CallOptions): Promise<string[]>;
  rank(question: string | Question | DecideSpec, records: readonly string[], options?: RankOptions): Promise<Ranked[]>;
  find(question: string | Question | DecideSpec, units: readonly string[], options?: FindOptions): Promise<Found | null>;
  /** A set member whose `on` names a part reads it from each record as JSON text. */
  annotate(set: QuestionSet, records: readonly string[], options?: CallOptions): Promise<AnnotatedRow[]>;
  details(question: string | Question | QuestionSpec, text: string, options?: CallOptions): Promise<Details>;
  recognize(text: string, options?: RecognizeOptions): Promise<Recognized>;
  relate(entities: readonly Entity[], options?: RelateOptions): Promise<Edge[]>;
  usage(): Usage;
}

/** An engine with its own settings, built on the environment. */
export class Engine implements Verbs {
  constructor(options?: EngineOptions);
  decide: Verbs['decide'];
  decide_many: Verbs['decide_many'];
  choose: Verbs['choose'];
  score: Verbs['score'];
  tag: Verbs['tag'];
  filter: Verbs['filter'];
  rank: Verbs['rank'];
  find: Verbs['find'];
  annotate: Verbs['annotate'];
  details: Verbs['details'];
  recognize: Verbs['recognize'];
  relate: Verbs['relate'];
  usage: Verbs['usage'];
}

/** Build a question from parts, once, and pass it to any verb. */
export function question(spec: QuestionSpec): Question;

export const decide: Verbs['decide'];
export const decide_many: Verbs['decide_many'];
export const choose: Verbs['choose'];
export const score: Verbs['score'];
export const tag: Verbs['tag'];
export const filter: Verbs['filter'];
export const rank: Verbs['rank'];
export const find: Verbs['find'];
export const annotate: Verbs['annotate'];
export const details: Verbs['details'];
export const recognize: Verbs['recognize'];
export const relate: Verbs['relate'];
export const usage: Verbs['usage'];

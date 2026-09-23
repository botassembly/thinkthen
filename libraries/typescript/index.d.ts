// The typed surface of `thinkthen` for Node. The eight verbs are async
// functions; `null` is "not sure"; a list crosses into the engine once and
// runs at the process width; an `AbortSignal` cancels a batch and its
// bill, and `deadlineMs` bounds a whole call.

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
  model?: string;
}

export type QuestionSpec = DecideSpec | ChooseSpec | ScoreSpec | TagSpec;

/** A question value, built by {@link question} and passed to any verb. */
export type Question = {
  readonly __question: true;
};

/** What every verb takes beside its arguments. */
export interface CallOptions {
  /** Cancels the call: no new request starts, sent requests finish. */
  signal?: AbortSignal;
  /** Bounds the whole call in milliseconds. A budget of zero is legal and
   * spent immediately: the call sends nothing and rejects with the
   * `deadline` kind naming the budget. No deadline is spelled `null`,
   * leaving it out, or the minus-one sentinel — the one spelling every
   * surface shares, including this one, so a computed `end - Date.now()`
   * that lands on -1 means none rather than refusing. Every other
   * negative, a NaN, and an oversized budget reject with the `usage`
   * kind. */
  deadlineMs?: number | null;
}

/** `choose`'s last object: the option list and the call options together. */
export interface ScoreOptions extends CallOptions {
  /** The question's levels, beside the text. */
  levels: readonly string[];
}

export interface ChooseOptions extends CallOptions {
  options: readonly string[];
}

/** `tag`'s last object: the label list and the call options together. */
export interface TagOptions extends CallOptions {
  labels: readonly string[];
}

/** `rank`'s last object: the top count and the call options together. */
export interface RankOptions extends CallOptions {
  top?: number;
}

/** `true`, `false`, or `null` when the answer is unsure. */
export type Answer = boolean | null;

/** One judgment from the audit trail. */
export interface DetailsAnswer {
  probability: number;
  /** The ruled key every surface uses, and the contract's own field name. */
  answer: Answer;
  model: string;
  digest: string;
  sends: number;
  /** The nearest level's name on a score question; null on every other
   * verb (ADR 0017 pick 6, settled 2026-09-21). */
  nearest: string | null;
  /** The ordered recording digests of the logical requests (0053): always
   * an array, one element for a one-request result, in construction order;
   * a retry adds no element. */
  requests: string[];
  /** Failed logical questions in this result (0054); always present,
   * including zero. */
  failed_questions: number;
}

/** One record's place and probability, most likely yes first. */
export interface RankedAnswer {
  index: number;
  record: string;
  probability: number;
}

/** The unit that best answers the question. `unit` is `null` when nothing
 * fits and the question carries a `none` arm. */
export interface FoundAnswer {
  index: number | null;
  unit: string | null;
  probability: number;
}

/** Why one logical question failed while its neighbours answered (0054).
 * The list is closed; `kind` is `backend` today. */
export type FailureCause =
  | 'missing_answer'
  | 'wrong_kind'
  | 'missing_probability'
  | 'invalid_probability'
  | 'invalid_distribution'
  | 'unexpected_probability';

/** The ruled failed-question marker (0054), this host's spelling: a plain
 * object, never `null`, which already means "not sure". */
export interface FailedField {
  failed: { kind: 'backend'; cause: FailureCause };
}

/** The fields `annotate` adds: a decision's answer, a choice's label or
 * `null`, a score's position, a tag's held labels, or the failed marker. */
export type AnnotatedField = Answer | string | null | number | readonly string[] | FailedField;

/** One name `recognize` found. `start` and `end` slice the name out of the
 * original text in JavaScript's own indexing (UTF-16 units); the contract
 * counts code points and the wrapper converts once. `strength` is a number
 * we compute — the least of the word probabilities behind the name times
 * the mean of the kind probabilities — defined once in the manual, with
 * its parts under details. It is never the vendor's `confidence`, which
 * passes through under details only. */
export interface Entity {
  id: number;
  text: string;
  kind: string;
  start: number;
  end: number;
  strength: number;
}

/** One relation between two names, by entity id. The number is
 * `probability`, a Jev number passed through. */
export interface Relation {
  name: string;
  source: number;
  target: number;
  probability: number;
}

/** What `recognize` returned: the names, and the relations when a rule
 * was given. */
export interface Recognized {
  entities: Entity[];
  relations: Relation[];
}

/** One edge `relate` found, by record numbers counted from 1 in input
 * order. An `either` edge prints once with the lower number in `source`. */
export interface Edge {
  name: string;
  source: number;
  target: number;
  probability: number;
  source_kind?: string;
  target_kind?: string;
}

/** `recognize`'s last object: the kinds, the relation rules, the bars,
 * and the call options together. A rule's ends are kind words, and the
 * any-kind end is the one-character string "*". */
export interface RecognizeOptions extends CallOptions {
  kinds?: readonly string[];
  relations?: Record<string, readonly [string, string]>;
  threshold?: number;
  relationThreshold?: number;
}

/** `relate`'s last object: the rule names, the both-ways names, the bar,
 * and the call options together. */
export interface RelateOptions extends CallOptions {
  relations?: readonly string[];
  either?: readonly string[];
  threshold?: number;
}

/** One record's fields, in the set's file order, one object a record in
 * input order. The input record itself is `records[index]`, the shape
 * Python, Ruby, R, and the C door return. */
export type AnnotatedRow = Record<string, AnnotatedField>;

export interface UsageAnswer {
  requests: number;
  cache_answers: number;
  tokens: number;
}

/** The engine's failure, with its kind and whether a second try could
 * help. `kind` is one of `usage`, `backend`, `deadline`, `local`,
 * `cancelled`, `defect`. */
export class ThinkThenError extends Error {
  kind: 'usage' | 'backend' | 'deadline' | 'local' | 'cancelled' | 'defect';
  retryable: boolean;
}

/** Build a question from parts, once, and pass it to any verb. */
export function question(spec: QuestionSpec): Question;

/** Ask once. A plain string takes the default cut. */
export function decide(question: string | Question, text: string, options?: CallOptions): Promise<Answer>;

/** `decide`'s bulk spelling: the same answer, once per record, in order. */
export function decide_many(question: string | Question, records: readonly string[], options?: CallOptions): Promise<Answer[]>;

/** Pick the option the evidence fits best, or `null` under the cut. A
 * bare question string takes its options in the last object. */
export function choose(question: string, text: string, options: ChooseOptions): Promise<string | null>;
export function choose(question: ChooseSpec | Question, text: string, options?: CallOptions): Promise<string | null>;

/** Place the evidence on the question's levels: the position from 0 to
 * K−1. The nearest level's name rides in the audit trail. A bare
 * question string takes its levels in the last object. */
export function score(question: string, text: string, options: ScoreOptions): Promise<number>;
export function score(question: ScoreSpec | Question, text: string, options?: CallOptions): Promise<number>;

/** Name the labels that held, in the question's order. A bare question
 * string takes its labels in the last object. */
export function tag(question: string, text: string, options: TagOptions): Promise<string[]>;
export function tag(question: TagSpec | Question, text: string, options?: CallOptions): Promise<string[]>;

/** Keep the records whose evidence reached the mark. A cut alone. */
export function filter(question: string | Question, records: readonly string[], options?: CallOptions): Promise<string[]>;

/** Order the records most likely yes first, ties in input order. No
 * threshold. `top` holds the first n of the ordered result. */
export function rank(question: string | Question, records: readonly string[], options?: RankOptions): Promise<RankedAnswer[]>;

/** Pick the unit that best answers the question, out of two to 255 sent
 * together. */
export function find(question: string | Question, units: readonly string[], options?: CallOptions): Promise<FoundAnswer>;

/** Ask every question in the set of every record, once each, adding one
 * field per question to each record. The set is a file path or the set
 * JSON. */
export function annotate(
  set: string,
  records: readonly string[],
  options?: CallOptions,
): Promise<AnnotatedRow[]>;

/** One judgment plus the audit trail, with the sends that produced it. */
export function details(question: string | Question, text: string, options?: CallOptions): Promise<DetailsAnswer>;

/** Find every name in a text and say what kind it is, with the relations
 * between them when rules are given. `start` and `end` index the original
 * text in UTF-16 units, so `text.slice(start, end)` is the name. */
export function recognize(text: string, options?: RecognizeOptions): Promise<Recognized>;

/** Say how the records relate to each other: one question per legal pair,
 * every record crossing at once. More than 255 records is a usage error. */
export function relate(records: readonly string[], options?: RelateOptions): Promise<Edge[]>;

/** The counters since the last reset: sends, cache answers, tokens. */
export function usage(): UsageAnswer;


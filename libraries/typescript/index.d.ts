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
  /** Bounds the whole call in positive milliseconds. */
  deadlineMs?: number;
}

/** `choose`'s last object: the option list and the call options together. */
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
  value: Answer;
  model: string;
  digest: string;
  sends: number;
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

/** The fields `annotate` adds: a decision's answer, a choice's label or
 * `null`, a score's position, or a tag's held labels. */
export type AnnotatedField = Answer | string | null | number | readonly string[];

export type AnnotatedRow<T> = T & Record<string, AnnotatedField>;

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
 * K−1. The nearest level's name rides in the audit trail. */
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
export function annotate<T extends object = { record: string }>(
  set: string,
  records: readonly (string | T)[],
  options?: CallOptions,
): Promise<AnnotatedRow<T>[]>;

/** One judgment plus the audit trail, with the sends that produced it. */
export function details(question: string | Question, text: string, options?: CallOptions): Promise<DetailsAnswer>;

/** The counters since the last reset: sends, cache answers, tokens. */
export function usage(): UsageAnswer;

/** Reset the counters. */
export function reset_usage(): void;

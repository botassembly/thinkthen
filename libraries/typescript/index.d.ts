// The typed face of `thinkthen` for Node. Every verb is async, `null` is
// unsure, a list crosses into the engine once, an `AbortSignal` cancels a
// call at once, and `deadlineMs` bounds a whole call.

/** A cut threshold: the probability at or above which the answer is yes. */
export type Cut = number;

/** A band threshold: unsure between the two numbers. */
export type Band = [low: number, high: number];

/** JSON carried unchanged through the version-one question grammar. */
export type JsonValue = null | boolean | number | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };
export type Description = string | null | readonly JsonValue[] | { readonly [key: string]: JsonValue };
export type QuestionText = string | readonly JsonValue[] | { readonly [key: string]: JsonValue };
export type LabelSet = readonly string[] | { readonly [key: string]: Description };

export interface DecideSpec {
  decide: QuestionText;
  batch?: 'max' | number;
  true?: Description;
  false?: Description;
  threshold?: Cut | Band;
  model?: string;
  profile?: string;
}

export interface ChooseSpec {
  choose: QuestionText;
  batch?: 'max' | number;
  options: LabelSet;
  threshold?: Cut;
  model?: string;
  profile?: string;
}

export interface ScoreSpec {
  score: QuestionText;
  batch?: 'max' | number;
  levels: LabelSet;
  model?: string;
  profile?: string;
}

export interface TagSpec {
  tag: QuestionText;
  batch?: 'max' | number;
  labels: LabelSet;
  threshold?: Cut;
  model?: string;
  profile?: string;
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

export interface ManyCallOptions extends CallOptions {
  batch?: 'max' | number;
  context?: string;
}

export interface AnnotateCallOptions extends CallOptions { batch?: 'max' | number; }

export interface ChooseOptions extends CallOptions {
  options: LabelSet;
}

export interface TagOptions extends CallOptions {
  labels: LabelSet;
}

export interface ScoreOptions extends CallOptions {
  levels: LabelSet;
}

export interface ChooseManyOptions extends ManyCallOptions { options: LabelSet; }
export interface ScoreManyOptions extends ManyCallOptions { levels: LabelSet; }
export interface TagManyOptions extends ManyCallOptions { labels: LabelSet; }

export interface RankOptions extends ManyCallOptions {
  /** Keep the first `top` of the ordered result. */
  top?: number;
}

export interface FindOptions extends CallOptions {
  /** Offer a none candidate, as `find --none` does, so the answer may be `null`. */
  none?: boolean;
}

/** `true`, `false`, or `null` when the answer is unsure. */
export type Answer = boolean | null;

/** Final facts of this call alone, including rows omitted from its value.
 * Facts are the engine's JSON, so a member not named here still reads. */
export interface Facts {
  readonly [member: string]: unknown;
  readonly records: number;
  readonly requests_sent: number;
  readonly cache_answers: number;
  readonly seconds: number;
  readonly input_tokens?: number;
  readonly output_tokens?: number;
  readonly model?: string;
}

/** One completed question; a failed question has `failed`, not `answer: null`. */
export interface QuestionObservation {
  readonly index: number;
  readonly position: number;
  readonly member?: string;
  readonly stage?: string;
  readonly question_sha256: string;
  readonly answer?: Answer | string | number | readonly string[];
  readonly failed?: FailedField['failed'];
  readonly probabilities?: number | readonly (readonly [string, number])[];
  readonly confidence?: number;
  readonly model: string;
  readonly url: string;
  readonly requests: readonly string[];
  readonly requests_sent: number;
  readonly usage?: { readonly input_tokens: number; readonly output_tokens: number };
  readonly cached: boolean;
  readonly failed_questions: number;
}

export interface Call<T> {
  readonly value: T;
  readonly facts: Facts;
  readonly details: readonly QuestionObservation[];
}

export interface Completion<T> {
  wait(): Promise<{ ok: Call<T> } | { err: ThinkThenError }>;
}

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
    profile_warning?: { tuned_for: string; running: string };
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

/** The failure of one annotate answer, or null for any value; null is unresolved, never a failure. */
export function failed(member: unknown): FailedField['failed'] | null;

/** A decide answer's code in the C door: YES 1, NO 0, UNSURE 2. */
export const YES: 1;
export const NO: 0;
export const UNSURE: 2;
export type Outcome = typeof YES | typeof NO | typeof UNSURE;

/** YES, NO, or UNSURE for a decide answer. */
export function outcome(answer: Answer): Outcome;

/** One annotated value: an answer, a pick, a position, labels, or the marker. */
export type AnnotatedField = Answer | string | number | string[] | FailedField;

/** One record's values by question name, in the set's order. */
export type AnnotatedRow = Record<string, AnnotatedField>;

/** A question set: a file path, the set's JSON text, or the set object. */
export type QuestionSet = string | { version: 1; batch?: 'max' | number; questions: Record<string, QuestionSpec> };

/** One name `recognize` found. `text.slice(start, end)` is the name, since
 * `start` and `end` count UTF-16 units, and `length` is `end - start`.
 * `kind` is `ENTITY` when the call named no kind. `strength` ranks names:
 * the kind probability times the span probability. It is not itself a
 * probability. */
export interface RecognizedEntity {
  text: string;
  start: number;
  end: number;
  length: number;
  kind: string;
  strength: number;
}

export interface RecognizedRelation {
  relation: string;
  source: RecognizedEntity;
  target: RecognizedEntity;
  probability: number;
  /** Present only on a relation of a both-ways rule, whose ends are then in the order found. */
  either?: true;
}

/** The names in one text, and their relations when rules were given. */
export interface Recognized {
  entities: RecognizedEntity[];
  relations?: RecognizedRelation[];
}

export interface RecognizeOptions extends CallOptions {
  /** A named version-one recognition plan; exclusive with inline plan options. */
  file?: string;
  /** Kind words, or kind words with their descriptions. */
  kinds?: readonly string[] | { readonly [key: string]: Description };
  /** Each rule's ends, as kind words or the any-kind end "*", or the file's rule list. */
  relations?:
    | Record<string, readonly [source: string, target: string]>
    | readonly { name: string; source: string; target: string; reads?: string; either?: boolean }[];
  threshold?: number;
  relationThreshold?: number;
}

/** An entity `relate` reads. A name `recognize` found is read by its `text`. */
export type Entity =
  | { name: string; kind: string }
  | { text: string; kind: string }
  | readonly [name: string, kind: string];

export interface Edge {
  relation: string;
  source: { name: string; kind: string };
  target: { name: string; kind: string };
  probability: number;
  /** Present only on an edge of a both-ways rule, whose ends are then in input order. */
  either?: true;
}

export interface RelateOptions extends CallOptions {
  /** A named version-one relation plan; exclusive with inline plan options. */
  file?: string;
  /** Rule names, each `name` or `name=source:target`. */
  relations?: readonly string[];
  /** The rule names that hold both ways. */
  either?: readonly string[];
  threshold?: number;
}

export interface PlanOptions { batch?: 'max' | number; context?: string; }

/** The requests a call would send: the result schema's `plan` object. */
export interface Plan {
  readonly records: number;
  readonly requests: number;
  readonly estimated_bytes: number;
  readonly estimated_input_tokens: { readonly lower: number; readonly upper: number };
  readonly upper_bound: boolean;
  readonly first_body_utf8: string | null;
  readonly [member: string]: unknown;
}

/** This process's totals. Failed calls and retries count, and nothing resets them. */
export interface Usage {
  requests_sent: number;
  retries: number;
  cache_answers: number;
  input_tokens: number;
  output_tokens: number;
}

/** The engine's failure, its kind, and whether the same call may pass later. */
export class ThinkThenError extends Error {
  kind: 'usage' | 'backend' | 'local' | 'cancelled' | 'deadline' | 'defect';
  /** The kind's code in the C door: usage 1, backend 2, deadline 3, local 4, cancelled 5, defect 6. */
  code: 1 | 2 | 3 | 4 | 5 | 6;
  retryable: boolean;
  facts?: Facts;
  details?: readonly QuestionObservation[];
  completion?: Completion<unknown>;
}

/** An engine's settings. Each given key overrides what the environment set. */
export interface EngineOptions {
  baseUrl?: string;
  model?: string;
  /** The most requests in flight at once, 1 through 32, for this loaded copy of the engine. */
  throttle?: number;
  /** Refuse a call over more records than this. */
  maxRequests?: number;
  /** Refuse a live send once the process has sent this many; every engine counts. */
  maxRequestsTotal?: number;
  /** Request-byte ceiling for a split plan; a lone question still goes alone. */
  maxRequestBytes?: number;
  /** `false` reads and writes no cache; a string names the cache folder. */
  cache?: false | string;
  timeoutSeconds?: number;
  maxRetries?: number;
  record?: string;
  replay?: string;
  profile?: string;
  batch?: 'max' | number;
}

export interface Verbs {
  decide(question: string | Question | DecideSpec, text: string, options?: CallOptions): Promise<Call<Answer>>;
  decide_many(question: string | Question | DecideSpec, records: readonly string[], options?: ManyCallOptions): Promise<Call<Answer[]>>;
  choose(question: string, text: string, options: ChooseOptions): Promise<Call<string | null>>;
  choose(question: ChooseSpec | Question, text: string, options?: CallOptions): Promise<Call<string | null>>;
  choose_many(question: string, records: readonly string[], options: ChooseManyOptions): Promise<Call<(string | null)[]>>;
  choose_many(question: ChooseSpec | Question, records: readonly string[], options?: ManyCallOptions): Promise<Call<(string | null)[]>>;
  score(question: string, text: string, options: ScoreOptions): Promise<Call<number>>;
  score(question: ScoreSpec | Question, text: string, options?: CallOptions): Promise<Call<number>>;
  score_many(question: string, records: readonly string[], options: ScoreManyOptions): Promise<Call<number[]>>;
  score_many(question: ScoreSpec | Question, records: readonly string[], options?: ManyCallOptions): Promise<Call<number[]>>;
  tag(question: string, text: string, options: TagOptions): Promise<Call<string[]>>;
  tag(question: TagSpec | Question, text: string, options?: CallOptions): Promise<Call<string[]>>;
  tag_many(question: string, records: readonly string[], options: TagManyOptions): Promise<Call<string[][]>>;
  tag_many(question: TagSpec | Question, records: readonly string[], options?: ManyCallOptions): Promise<Call<string[][]>>;
  filter(question: string | Question | DecideSpec, records: readonly string[], options?: ManyCallOptions): Promise<Call<string[]>>;
  rank(question: string | Question | DecideSpec, records: readonly string[], options?: RankOptions): Promise<Call<Ranked[]>>;
  find(question: string | Question | DecideSpec, units: readonly string[], options?: FindOptions): Promise<Call<Found | null>>;
  /** A set member whose `on` names a part reads it from each record as JSON text. */
  annotate(set: QuestionSet, records: readonly string[], options?: AnnotateCallOptions): Promise<Call<AnnotatedRow[]>>;
  details(question: string | Question | QuestionSpec, text: string, options?: CallOptions): Promise<Call<Details>>;
  recognize(text: string, options?: RecognizeOptions): Promise<Call<Recognized>>;
  relate(entities: readonly Entity[], options?: RelateOptions): Promise<Call<Edge[]>>;
  /** Preview a decide, choose, score, or tag call: nothing is read from a key or cache, and nothing is sent. */
  plan(question: string | Question | QuestionSpec, records: readonly string[], options?: PlanOptions): Plan;
  usage(): Usage;
}

/** An engine with its own settings, built on the environment. */
export class Engine implements Verbs {
  constructor(options?: EngineOptions);
  decide: Verbs['decide'];
  decide_many: Verbs['decide_many'];
  choose_many: Verbs['choose_many'];
  score_many: Verbs['score_many'];
  tag_many: Verbs['tag_many'];
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
  plan: Verbs['plan'];
  usage: Verbs['usage'];
}

/** Build a question from parts, once, and pass it to any verb. */
export function question(spec: QuestionSpec): Question;

/** Load one named local question; invalid files raise a Local error. */
export function questionFile(path: string): Question;

export const decide: Verbs['decide'];
export const decide_many: Verbs['decide_many'];
export const choose_many: Verbs['choose_many'];
export const score_many: Verbs['score_many'];
export const tag_many: Verbs['tag_many'];
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
export const plan: Verbs['plan'];

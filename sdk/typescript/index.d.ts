/** Supported backends; stub runs locally without credentials. */
export type Backend = "systemone" | "openai" | "stub";
/** A decision threshold, or an explicit not-sure band. */
export type Threshold = number | {
  /** Below this probability, answer no. */
  readonly low: number;
  /** At or above this probability, answer yes; the band between is not sure. */
  readonly high: number };
/** A JSON value, allowing nested arrays inside structured records. */
export type JsonValue = null | boolean | number | string
  | readonly JsonValue[] | {
  readonly [key: string]: JsonValue };
/** One scalar or object record. Top-level arrays belong to a stream; null and undefined are missing records. */
export type RecordInput = null | undefined | boolean | number | string | {
  readonly [key: string]: JsonValue } | {
  /** Caller id in its original string or integer type. JSON.stringify throws for bigint ids. */
  readonly id: bigint;
  readonly [key: string]: JsonValue | bigint };
/** A synchronous or asynchronous object iterable of records. Strings are not record collections. */
export type Records<T extends RecordInput = RecordInput> = (Iterable<T> | AsyncIterable<T>) & object;
/** JSON object keys or array indices used to select field. */
export type KeyPath = readonly (string | number)[];
/** Backend settings admitted once when the client is constructed. */
export interface ClientOptions {
  /** Select the service backend; otherwise use THINKTHEN_BACKEND, then systemone. */
  readonly backend?: Backend;
  /** Override the selected backend model. */
  readonly model?: string;
  /** Override the backend URL; credentials may use plaintext only on loopback. */
  readonly url?: string;
  /** Use this credential instead of the selected backend environment variable. */
  readonly apiKey?: string;
  /** Maximum records admitted to each request. */
  readonly batch?: number;
  /** Configure bounded retries and attempt deadlines. */
  readonly retry?: {
  /** Maximum retries after the first attempt. */
  readonly maxRetries?: number;
  /** Initial retry delay in milliseconds. */
  readonly initialBackoffMs?: number;
  /** Deadline for each attempt in milliseconds. */
  readonly attemptTimeoutMs?: number;
  };
}
/** Per-call field, naming and cancellation options. */
export interface CallOptions {
  /** Select field with a JSON pointer or an escaped key path. */
  readonly field?: string | KeyPath;
  /** Execution name retained in row source metadata. */
  readonly name?: string;
  /** Cancel this call with an AbortSignal. */
  readonly signal?: AbortSignal;
}
/** Decision options, including a not-sure threshold band. */
export interface DecisionOptions extends CallOptions {
  /** Set the answer threshold, or the low/high not-sure band for decisions. */
  readonly threshold?: Threshold }
/** Options for functions with one numeric threshold. */
export interface NumericOptions extends CallOptions {
  /** Minimum probability for a positive match. */
  readonly threshold?: number }
/** Choices required for a direct choose call. */
export interface ChooseOptions<C extends readonly string[]> extends DecisionOptions, Descriptions<C[number]> {
  /** Allowed answers, preserving the literal TypeScript domain. */
  readonly choices: C }
/** Labels required for a direct tag call. */
export interface TagOptions<L extends readonly string[]> extends NumericOptions, Descriptions<L[number]> {
  /** Allowed tags, or each label probability in result detail. */
  readonly labels: L }
/** Ordered levels required for a direct score call. */
export interface ScoreOptions extends CallOptions, Descriptions {
  /** Ordered score levels, or their probabilities in result detail. */
  readonly levels: readonly string[] }
/** Ordered levels and optional output limit for ranking. */
export interface RankOptions extends ScoreOptions {
  /** Maximum ranked records to emit. */
  readonly top?: number }

/** Backend and admitted-question provenance. */
export interface Source {
  /** Stable identity of the admitted question. */
  readonly questionIdentity: string;
  /** Execution name retained in row source metadata. */
  readonly name?: string;
  /** Select the service backend; otherwise use THINKTHEN_BACKEND, then systemone. */
  readonly backend: Backend;
  /** Override the selected backend model. */
  readonly model: string;
}
/** Stable engine error details for one failed record. */
export interface ErrorInfo {
  /** Stable engine error code; generated from the shared registry. */
  readonly code: RowErrorCode;
  /** Engine diagnostic without credentials or raw backend replies. */
  readonly message: string;
  /** HTTP status when the backend supplied one. */
  readonly httpStatus?: number;
}
/** Answered, not-sure or failed judgment; only answered outcomes contain an answer. */
export type Outcome<A, D = never> =
  | {
  /** Narrow the outcome before reading its answer, error or probability. */
  readonly status: "answered";
  /** The answered value; absent on failed and not-sure outcomes. */
  readonly answer: A;
  /** Answer confidence. For decide and filter this is the probability of yes, including on a no. */
  readonly probability?: number;
  /** Typed supporting probabilities and spans; never the raw backend reply. */
  readonly detail?: D }
  | {
  /** Narrow the outcome before reading its answer, error or probability. */
  readonly status: "unsure";
  /** Answer confidence. For decide and filter this is the probability of yes, including on a no. */
  readonly probability: number;
  /** Typed supporting probabilities and spans; never the raw backend reply. */
  readonly detail?: D }
  | {
  /** Narrow the outcome before reading its answer, error or probability. */
  readonly status: "failed";
  /** The failed record error, with its engine code and message. */
  readonly error: ErrorInfo };
/** Immutable judgment with function, position, id and provenance. */
export type Row<F extends FunctionName, A, D = never> = Outcome<A, D> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: F;
  /** One-based input position, independent of the record id. */
  readonly position: number;
  /** Caller id in its original string or integer type. JSON.stringify throws for bigint ids. */
  readonly id?: string | number | bigint;
  /** Provenance of the judgment, or the source span of a relation. */
  readonly source: Source;
};
/** Boolean judgment whose probability is always the probability of yes. */
export type DecideRow = ProbabilityRow<"decide">;
/** Boolean rows always carry the probability of yes when answered. */
type ProbabilityRow<F extends "decide" | "filter"> = Exclude<Row<F, boolean>, {readonly status:"answered"}> | (Extract<Row<F, boolean>, {readonly status:"answered"}> & {readonly probability:number});
/** One allowed choice and its label probabilities. */
export type ChooseRow<C extends string = string> = Row<"choose", C, LabelsDetail<C>>;
/** Allowed tags and their label probabilities. */
export type TagRow<L extends string = string> = Row<"tag", readonly L[], LabelsDetail<L>>;
/** Numeric score with probabilities in level order. */
export type ScoreRow = Row<"score", number, LevelsDetail>;
/** Filter judgment whose probability is always the probability of yes. */
export type FilterRow = ProbabilityRow<"filter">;
/** One-based rank with the original input position. */
export type RankRow<T extends RecordInput = RecordInput> = Row<"rank", number, LevelsDetail> & {
  /** Equal-value, immutable native snapshot of the input, not the caller's object. Unconvertible records have no snapshot. */
  readonly item: T | undefined;
};
/** Matched span, or null when nothing was found. */
export type FindRow = Row<"find", Span | null>;
/** Recognized names and their typed probability detail. */
export type RecognizeRow<K extends string = string> = Row<"recognize", readonly RecognizedName<K>[], RecognizeDetail>;
/** Relations with their typed name and pair detail. */
export type RelateRow<R extends string = string> = Row<"relate", readonly Relation<R>[], RelateDetail<R>>;
/** Matched text and half-open UTF-16 offsets. */
export interface Span {
  /** Matched text for the span. */
  readonly text: string;
  /** Inclusive offset in UTF-16 code units, suitable for String.slice. */
  readonly start: number;
  /** Exclusive offset in UTF-16 code units, suitable for String.slice. */
  readonly end: number }
/** A matched span with its entity kind. */
export interface NamedSpan<K extends string = string> extends Span {
  /** Recognized entity kind. */
  readonly kind: K }
/** A typed entity span and its probability. */
export interface RecognizedName<K extends string = string> extends NamedSpan<K> {
  /** Answer confidence. For decide and filter this is the probability of yes, including on a no. */
  readonly probability: number }
/** A directed or symmetric relation between two spans. */
export interface Relation<R extends string = string> {
  /** Provenance of the judgment, or the source span of a relation. */
  readonly source: NamedSpan;
  /** Target span of the relation. */
  readonly target: NamedSpan;
  /** Relation name from the supplied rule domain. */
  readonly relation: R;
  /** Answer confidence. For decide and filter this is the probability of yes, including on a no. */
  readonly probability: number;
  /** Whether the relation may be read in either direction. */
  readonly symmetric?: boolean;
}
/** One allowed relation between source and target kinds. */
export interface RelationRule {
  /** Relation name used in the returned answers. */
  readonly name: string;
  /** Kind allowed at the source of this relation. */
  readonly source: string;
  /** Kind allowed at the target of this relation. */
  readonly target: string;
  /** Description of how to read this relation. */
  readonly phrase?: string;
  /** Whether the relation may be read in either direction. */
  readonly symmetric?: boolean;
}
/** Each allowed label mapped to its probability. */
export interface LabelsDetail<L extends string = string> {
  /** Allowed tags, or each label probability in result detail. */
  readonly labels: Readonly<Record<L, number>> }
/** Probabilities in the same order as the supplied levels. */
export interface LevelsDetail {
  /** Ordered score levels, or their probabilities in result detail. */
  readonly levels: readonly number[] }
/** Boundary and entity-kind probabilities supporting recognition. */
export interface RecognizeDetail {
  /** Text pieces and their boundary-tag probabilities. */
  readonly pieces: readonly (Span & {
  /** Probabilities for each boundary tag. */
  readonly tags: Readonly<Record<"BEGIN" | "INSIDE" | "END" | "SINGLE" | "OUT", number>> })[];
  /** Recognized names, with kind probabilities or typed relation details. */
  readonly names: readonly {
  /** Matched text retained with the supporting kind probabilities. */
  readonly text: string;
  /** Inclusive offset in UTF-16 code units, suitable for String.slice. */
  readonly start: number;
  /** Exclusive offset in UTF-16 code units, suitable for String.slice. */
  readonly end: number;
  /** Allowed entity kinds, or their probabilities in result detail. */
  readonly kinds: Readonly<Record<string, number>>;
  }[];
}
/** Names and pairs supporting a relation judgment. */
export interface RelateDetail<R extends string = string> {
  /** Recognized names, with kind probabilities or typed relation details. */
  readonly names: readonly RecognizedName[];
  /** Relations between recognized names. */
  readonly pairs: readonly Relation<R>[] }

type QuestionFields = {
  /** Select field with a JSON pointer or an escaped key path. */
  readonly field?: string | KeyPath };
type DecisionFields = QuestionFields & {
  /** Set the answer threshold, or the low/high not-sure band for decisions. */
  readonly threshold?: Threshold };
type NumericFields = QuestionFields & {
  /** Set the answer threshold, or the low/high not-sure band for decisions. */
  readonly threshold?: number };
/** A reusable yes-or-no question with its field and threshold. */
export type DecideQuestion = DecisionFields & Pick<BooleanOptions, "yesMeans" | "noMeans"> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "decide";
  /** The bounded question to ask about the selected record field. */
  readonly question: string };
/** A reusable question that keeps records answered yes. */
export type FilterQuestion = DecisionFields & Pick<BooleanOptions, "yesMeans" | "noMeans"> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "filter";
  /** The bounded question to ask about the selected record field. */
  readonly question: string };
/** A reusable question whose answer comes from the supplied choices. */
export type ChooseQuestion<C extends readonly string[] = readonly string[]> = DecisionFields & Descriptions<C[number]> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "choose";
  /** The bounded question to ask about the selected record field. */
  readonly question: string;
  /** Allowed answers, preserving the literal TypeScript domain. */
  readonly choices: C;
};
/** A reusable question that selects any matching labels. */
export type TagQuestion<L extends readonly string[] = readonly string[]> = NumericFields & Descriptions<L[number]> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "tag";
  /** The bounded question to ask about the selected record field. */
  readonly question: string;
  /** Allowed tags, or each label probability in result detail. */
  readonly labels: L;
};
/** A reusable question scored against ordered levels. */
export type ScoreQuestion = QuestionFields & Descriptions & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "score";
  /** The bounded question to ask about the selected record field. */
  readonly question: string;
  /** Ordered score levels, or their probabilities in result detail. */
  readonly levels: readonly string[] };
/** A reusable ranking question with ordered levels and an optional limit. */
export type RankQuestion = QuestionFields & Descriptions & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "rank";
  /** The bounded question to ask about the selected record field. */
  readonly question: string;
  /** Ordered score levels, or their probabilities in result detail. */
  readonly levels: readonly string[];
  /** Maximum ranked records to emit. */
  readonly top?: number };
/** A reusable question that finds a matching text span. */
export type FindQuestion = NumericFields & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "find";
  /** The bounded question to ask about the selected record field. */
  readonly question: string };
/** A reusable entity recognition question with the allowed kinds. */
export type RecognizeQuestion<K extends readonly string[] = readonly string[]> = NumericFields & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "recognize";
  /** Allowed entity kinds, or their probabilities in result detail. */
  readonly kinds: K };
/** A reusable relation question with its allowed rules. */
export type RelateQuestion<R extends readonly RelationRule[] = readonly RelationRule[]> = NumericFields & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "relate";
  /** Allowed relation definitions, including source and target kinds. */
  readonly rules: R };
/** A typed question definition for any saved function. */
export type ToolboxQuestion = DecideQuestion | FilterQuestion | ChooseQuestion | TagQuestion
  | ScoreQuestion | RankQuestion | FindQuestion | RecognizeQuestion | RelateQuestion;
/** Question definitions indexed by caller-chosen entry names. */
export type Toolbox = Readonly<Record<string, ToolboxQuestion>>;
/** Load saved question toolbox JSON through shared engine admission. */
export function parseToolbox(json: string): Toolbox;
/** Save a toolbox for another SDK or the command. */
export function serializeToolbox(toolbox: Toolbox): string;

/** A question usable within annotate; rank cannot be nested. */
export type AnnotationQuestion = Exclude<ToolboxQuestion, RankQuestion>;
/** Named questions evaluated against one record. */
export type AnnotationQuestions = Readonly<Record<string, AnnotationQuestion>>;
/** The typed outcome of one annotation question. */
export type EntryOf<Q extends AnnotationQuestion> =
  Q extends ChooseQuestion<infer C> ? Outcome<C[number], LabelsDetail<C[number]>> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "choose" } :
  Q extends TagQuestion<infer L> ? Outcome<readonly L[number][], LabelsDetail<L[number]>> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "tag" } :
  Q extends RecognizeQuestion<infer K> ? Outcome<readonly RecognizedName<K[number]>[], RecognizeDetail> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "recognize" } :
  Q extends RelateQuestion<infer R> ? Outcome<readonly Relation<R[number]["name"]>[], RelateDetail<R[number]["name"]>> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "relate" } :
  Q extends DecideQuestion | FilterQuestion ? Outcome<boolean> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: Q["function"] } :
  Q extends ScoreQuestion ? Outcome<number, LevelsDetail> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: Q["function"] } :
  Q extends FindQuestion ? Outcome<Span | null> & {
  /** Discriminant naming the function that produced this outcome. */
  readonly function: "find" } : never;
/** Full child outcomes indexed by annotation question name. */
export type AnnotationOf<T extends AnnotationQuestions> = {
  readonly [N in keyof T]: EntryOf<T[N]> };
/** Annotation outcomes with the outer record metadata. */
export type AnnotateRow<T extends AnnotationQuestions = AnnotationQuestions> = Row<"annotate", AnnotationOf<T>>;
/** Recursively readonly snapshot type. */
export type DeepReadonly<T> = T extends object
  ? {
  readonly [P in keyof T]: DeepReadonly<T[P]> } : T;
/** Copy and deeply freeze question definitions without freezing their owner. */
export function defineToolbox<const T extends Toolbox>(value: T): DeepReadonly<T>;

/** The ten public function names. */
export type FunctionName = "decide" | "choose" | "tag" | "score" | "filter" | "rank" | "find" | "recognize" | "relate" | "annotate";
/** Any typed public row, narrowed by function and status. */
export type AnyRow = DecideRow | ChooseRow | TagRow | ScoreRow | FilterRow | RankRow | FindRow | RecognizeRow | RelateRow | AnnotateRow;
/** Options for a call whose question already carries its configuration. */
export type SignalOptions = Pick<CallOptions, "signal" | "name">;
/** Lazy iterator that owns cancellation and cleanup; dispose or return it when stopping early. */
export interface RowStream<R> extends AsyncIterableIterator<R>, AsyncDisposable {
  /** Stop reading and release this call's native resources and input iterator. */
  return(): Promise<IteratorResult<R>>;
  /** Read the stream to completion; reject on a call failure. */
  toArray(): Promise<R[]>;
  /** Release this call when leaving an await using scope. */
  [Symbol.asyncDispose](): Promise<void>;
}
/** Every-row filter view with the original record and kept decision. */
export interface FilterItem<T> {
  /** Original caller record; the SDK does not clone or freeze it. */
  readonly item: T;
  /** Full immutable row, including not-sure and failed outcomes. */
  readonly row: FilterRow;
  /** True only for an answered yes. */
  readonly kept: boolean; }
/** Evaluate decide for one record. Await its answer; call row() for every status and metadata. */
export function decide(question: string, record: RecordInput, options?: BooleanOptions): AnswerPromise<DecideRow>;
/** Evaluate decide for one record. Await its answer; call row() for every status and metadata. */
export function decide(question: DecideQuestion, record: RecordInput, options?: SignalOptions): AnswerPromise<DecideRow>;
/** Evaluate decide lazily and retain every outcome with its input position. */
export function decideStream(question: string, records: Records, options?: BooleanOptions): RowStream<DecideRow>;
/** Evaluate decide lazily and retain every outcome with its input position. */
export function decideStream(question: DecideQuestion, records: Records, options?: SignalOptions): RowStream<DecideRow>;
/** Evaluate choose for one record. Await its answer; call row() for every status and metadata. */
export function choose<const C extends readonly string[]>(question: string, record: RecordInput, options: ChooseOptions<C>): AnswerPromise<ChooseRow<C[number]>>;
/** Evaluate choose for one record. Await its answer; call row() for every status and metadata. */
export function choose<const C extends readonly string[]>(question: ChooseQuestion<C>, record: RecordInput, options?: SignalOptions): AnswerPromise<ChooseRow<C[number]>>;
/** Evaluate choose lazily and retain every outcome with its input position. */
export function chooseStream<const C extends readonly string[]>(question: string, records: Records, options: ChooseOptions<C>): RowStream<ChooseRow<C[number]>>;
/** Evaluate choose lazily and retain every outcome with its input position. */
export function chooseStream<const C extends readonly string[]>(question: ChooseQuestion<C>, records: Records, options?: SignalOptions): RowStream<ChooseRow<C[number]>>;
/** Evaluate tag for one record. Await its answer; call row() for every status and metadata. */
export function tag<const L extends readonly string[]>(question: string, record: RecordInput, options: TagOptions<L>): AnswerPromise<TagRow<L[number]>>;
/** Evaluate tag for one record. Await its answer; call row() for every status and metadata. */
export function tag<const L extends readonly string[]>(question: TagQuestion<L>, record: RecordInput, options?: SignalOptions): AnswerPromise<TagRow<L[number]>>;
/** Evaluate tag lazily and retain every outcome with its input position. */
export function tagStream<const L extends readonly string[]>(question: string, records: Records, options: TagOptions<L>): RowStream<TagRow<L[number]>>;
/** Evaluate tag lazily and retain every outcome with its input position. */
export function tagStream<const L extends readonly string[]>(question: TagQuestion<L>, records: Records, options?: SignalOptions): RowStream<TagRow<L[number]>>;
/** Evaluate score for one record. Await its answer; call row() for every status and metadata. */
export function score(question: string, record: RecordInput, options: ScoreOptions): AnswerPromise<ScoreRow>;
/** Evaluate score for one record. Await its answer; call row() for every status and metadata. */
export function score(question: ScoreQuestion, record: RecordInput, options?: SignalOptions): AnswerPromise<ScoreRow>;
/** Evaluate score lazily and retain every outcome with its input position. */
export function scoreStream(question: string, records: Records, options: ScoreOptions): RowStream<ScoreRow>;
/** Evaluate score lazily and retain every outcome with its input position. */
export function scoreStream(question: ScoreQuestion, records: Records, options?: SignalOptions): RowStream<ScoreRow>;
/** Evaluate find for one record. Await its answer; call row() for every status and metadata. */
export function find(question: string, record: RecordInput, options?: NumericOptions): AnswerPromise<FindRow>;
/** Evaluate find for one record. Await its answer; call row() for every status and metadata. */
export function find(question: FindQuestion, record: RecordInput, options?: SignalOptions): AnswerPromise<FindRow>;
/** Evaluate find lazily and retain every outcome with its input position. */
export function findStream(question: string, records: Records, options?: NumericOptions): RowStream<FindRow>;
/** Evaluate find lazily and retain every outcome with its input position. */
export function findStream(question: FindQuestion, records: Records, options?: SignalOptions): RowStream<FindRow>;
/** Evaluate recognize for one record. Await its answer; call row() for every status and metadata. */
export function recognize<const K extends readonly string[]>(question: K, record: RecordInput, options?: NumericOptions): AnswerPromise<RecognizeRow<K[number]>>;
/** Evaluate recognize for one record. Await its answer; call row() for every status and metadata. */
export function recognize<const K extends readonly string[]>(question: RecognizeQuestion<K>, record: RecordInput, options?: SignalOptions): AnswerPromise<RecognizeRow<K[number]>>;
/** Evaluate recognize lazily and retain every outcome with its input position. */
export function recognizeStream<const K extends readonly string[]>(question: K, records: Records, options?: NumericOptions): RowStream<RecognizeRow<K[number]>>;
/** Evaluate recognize lazily and retain every outcome with its input position. */
export function recognizeStream<const K extends readonly string[]>(question: RecognizeQuestion<K>, records: Records, options?: SignalOptions): RowStream<RecognizeRow<K[number]>>;
/** Evaluate relate for one record. Await its answer; call row() for every status and metadata. */
export function relate<const R extends readonly RelationRule[]>(question: R, record: RecordInput, options?: NumericOptions): AnswerPromise<RelateRow<R[number]["name"]>>;
/** Evaluate relate for one record. Await its answer; call row() for every status and metadata. */
export function relate<const R extends readonly RelationRule[]>(question: RelateQuestion<R>, record: RecordInput, options?: SignalOptions): AnswerPromise<RelateRow<R[number]["name"]>>;
/** Evaluate relate lazily and retain every outcome with its input position. */
export function relateStream<const R extends readonly RelationRule[]>(question: R, records: Records, options?: NumericOptions): RowStream<RelateRow<R[number]["name"]>>;
/** Evaluate relate lazily and retain every outcome with its input position. */
export function relateStream<const R extends readonly RelationRule[]>(question: RelateQuestion<R>, records: Records, options?: SignalOptions): RowStream<RelateRow<R[number]["name"]>>;
/** Evaluate annotate for one record. Await its answer; call row() for every status and metadata. */
export function annotate<const T extends AnnotationQuestions>(question: T, record: RecordInput, options?: CallOptions): AnswerPromise<AnnotateRow<T>>;
/** Evaluate annotate lazily and retain every outcome with its input position. */
export function annotateStream<const T extends AnnotationQuestions>(question: T, records: Records, options?: CallOptions): RowStream<AnnotateRow<T>>;
/** Evaluate filter lazily and retain every outcome with its input position. */
export function filterStream<T extends RecordInput>(question: string, records: Records<T>, options?: BooleanOptions): RowStream<FilterItem<T>>;
/** Evaluate filter lazily and retain every outcome with its input position. */
export function filterStream<T extends RecordInput>(question: FilterQuestion, records: Records<T>, options?: SignalOptions): RowStream<FilterItem<T>>;
/** Evaluate rank lazily and retain every outcome with its input position. */
export function rankStream<T extends RecordInput>(question: string, records: Records<T>, options: RankOptions): RowStream<RankRow<T>>;
/** Evaluate rank lazily and retain every outcome with its input position. */
export function rankStream<T extends RecordInput>(question: RankQuestion, records: Records<T>, options?: SignalOptions): RowStream<RankRow<T>>;

/** Reusable native engine client. Close it, or use await using, after its calls finish. */
export declare class ThinkThen implements AsyncDisposable {
  /** Create an engine with explicit settings or the documented environment defaults. */
  constructor(options?: ClientOptions);
  /** Cancel active calls and release native resources; repeated closes are safe. */
  close(): Promise<void>;
  /** Close the client when leaving an await using scope. */
  [Symbol.asyncDispose](): Promise<void>;
  /** Judge one record; await the answer or read its row(). */
  readonly decide: typeof decide;
  /** Evaluate decide lazily, retaining all outcomes. */
  readonly decideStream: typeof decideStream;
  /** Judge one record; await the answer or read its row(). */
  readonly choose: typeof choose;
  /** Evaluate choose lazily, retaining all outcomes. */
  readonly chooseStream: typeof chooseStream;
  /** Judge one record; await the answer or read its row(). */
  readonly tag: typeof tag;
  /** Evaluate tag lazily, retaining all outcomes. */
  readonly tagStream: typeof tagStream;
  /** Judge one record; await the answer or read its row(). */
  readonly score: typeof score;
  /** Evaluate score lazily, retaining all outcomes. */
  readonly scoreStream: typeof scoreStream;
  /** Judge one record; await the answer or read its row(). */
  readonly find: typeof find;
  /** Evaluate find lazily, retaining all outcomes. */
  readonly findStream: typeof findStream;
  /** Judge one record; await the answer or read its row(). */
  readonly recognize: typeof recognize;
  /** Evaluate recognize lazily, retaining all outcomes. */
  readonly recognizeStream: typeof recognizeStream;
  /** Judge one record; await the answer or read its row(). */
  readonly relate: typeof relate;
  /** Evaluate relate lazily, retaining all outcomes. */
  readonly relateStream: typeof relateStream;
  /** Judge one record; await the answer or read its row(). */
  readonly annotate: typeof annotate;
  /** Evaluate annotate lazily, retaining all outcomes. */
  readonly annotateStream: typeof annotateStream;
  /** Yield kept original records, then one error carrying all failures. */
  readonly filter: typeof filter;
  /** Evaluate filter lazily, retaining all outcomes. */
  readonly filterStream: typeof filterStream;
  /** Evaluate rank lazily, retaining all outcomes. */
  readonly rankStream: typeof rankStream;
}
export { ERROR_CODES } from "./errors.js";
import type { ErrorCode, RowErrorCode, UsageCode, BackendCode } from "./errors.js";
export type { ErrorCode, RowErrorCode, UsageCode, BackendCode } from "./errors.js";
/** A failure carrying the shared engine code and message. */
export declare class ThinkThenError extends Error {
  /** Preserve the engine diagnostic and optional underlying cause. */
  constructor(error: {
  /** Stable engine error code; generated from the shared registry. */
  readonly code: ErrorCode;
  /** Engine diagnostic without credentials or raw backend replies. */
  readonly message: string;
  /** HTTP status when the backend supplied one. */
  readonly httpStatus?: number }, options?: ErrorOptions);
  /** Stable engine error code; generated from the shared registry. */
  readonly code: ErrorCode;
  /** HTTP status when the backend supplied one. */
  readonly httpStatus?: number; }
/** Invalid configuration, question, argument or client lifetime. */
export declare class UsageError extends ThinkThenError {
  /** Preserve the engine diagnostic and optional underlying cause. */
  constructor(error: {
  /** Stable engine error code; generated from the shared registry. */
  readonly code: UsageCode;
  /** Engine diagnostic without credentials or raw backend replies. */
  readonly message: string;
  /** HTTP status when the backend supplied one. */
  readonly httpStatus?: number }, options?: ErrorOptions);
  /** Stable engine error code; generated from the shared registry. */
  readonly code: UsageCode; }
/** A service failure with a shared backend code. */
export declare class BackendError extends ThinkThenError {
  /** Preserve the engine diagnostic and optional underlying cause. */
  constructor(error: {
  /** Stable engine error code; generated from the shared registry. */
  readonly code: BackendCode;
  /** Engine diagnostic without credentials or raw backend replies. */
  readonly message: string;
  /** HTTP status when the backend supplied one. */
  readonly httpStatus?: number }, options?: ErrorOptions);
  /** Stable engine error code; generated from the shared registry. */
  readonly code: BackendCode; }
/** Cancellation error named AbortError, with the signal reason as its cause. */
export declare class CancelledError extends ThinkThenError {
  /** Preserve the engine diagnostic and optional underlying cause. */
  constructor(error: {
  /** Stable engine error code; generated from the shared registry. */
  readonly code: "local.cancelled";
  /** Engine diagnostic without credentials or raw backend replies. */
  readonly message: string }, options?: ErrorOptions);
  /** Stable engine error code; generated from the shared registry. */
  readonly code: "local.cancelled"; }

/** The answered value of a row, preserving its label domain. */
export type AnswerOf<R> = R extends {
  /** Narrow the outcome before reading its answer, error or probability. */
  readonly status: "answered";
  /** The answered value; absent on failed and not-sure outcomes. */
  readonly answer: infer A } ? A : never;
/** Await the answer, or use row() to inspect every status from the same call. */
export interface AnswerPromise<R extends AnyRow> extends Promise<AnswerOf<R>> {
  /** Read the immutable row without extracting its answer or executing again. */
  row(): Promise<R>;
}
/** A full row or an annotation entry; entries omit position, id and source. */
export type Judgment = Outcome<unknown, unknown> & {
  /** Function that produced this judgment. */
  readonly function: FunctionName };
/** Extract the typed answer of a row or annotation entry; throw for not sure or failure. */
export function answer<R extends Judgment>(row: R): AnswerOf<R>;
/** A judgment whose probability lies between the decision thresholds. */
export declare class UnsureError extends Error {
  /** Preserve the row or annotation entry and its probability. Not sure has no engine error code. */
  constructor(row: Extract<Judgment, { readonly status: "unsure" }>);
  /** Original immutable row or annotation entry. */
  readonly row: Extract<Judgment, { readonly status: "unsure" }>;
  /** Confidence of the judgment at the chosen thresholds. */
  readonly probability: number;
}
/** A failed record or annotation entry; filters carry all failed rows. */
export declare class RecordError extends ThinkThenError {
  /** Use the first failure's engine code and retain every failed judgment. */
  constructor(rows: readonly Extract<Judgment, { readonly status: "failed" }>[]);
  /** First failed row or annotation entry. */
  readonly row: Extract<Judgment, { readonly status: "failed" }>;
  /** All failed judgments in input order. */
  readonly rows: readonly Extract<Judgment, { readonly status: "failed" }>[];
}
/** Yield original kept records, then reject once with all failed rows. */
export function filter<T extends RecordInput>(question: string, records: Records<T>, options?: BooleanOptions): RowStream<T>;
/** Yield original kept records using a saved filter question. */
export function filter<T extends RecordInput>(question: FilterQuestion, records: Records<T>, options?: SignalOptions): RowStream<T>;

/** Set the caller-owned client used by module functions; pass undefined to reset it. Module functions never close this client. */
export function setDefaultClient(client: ThinkThen | undefined): void;
/** Read toolbox JSON from a UTF-8 file through shared engine admission. */
export function loadToolbox(path: string | URL): Promise<Toolbox>;

/** Meanings for the yes and no outcomes of a boolean judgment. */
export interface BooleanOptions extends DecisionOptions {
  /** Meaning of an answered yes. */
  readonly yesMeans?: string;
  /** Meaning of an answered no. */
  readonly noMeans?: string;
}
/** Optional descriptions indexed only by names in the supplied domain. */
export interface Descriptions<N extends string = string> {
  /** Meaning of each named answer or ordered level. Unknown keys are refused. */
  readonly descriptions?: Readonly<Partial<Record<N, string>>>;
}

export type Backend = "systemone" | "openai" | "stub";
export type Threshold = number | { readonly low: number; readonly high: number };
export type JsonValue = null | boolean | number | string
  | readonly JsonValue[] | { readonly [key: string]: JsonValue };
export type RecordInput = JsonValue | { readonly id: bigint; readonly [key: string]: JsonValue | bigint };
export type Records<T extends RecordInput = RecordInput> = Iterable<T> | AsyncIterable<T>;
export type KeyPath = readonly (string | number)[];
export interface ClientOptions {
  readonly backend?: Backend;
  readonly model?: string;
  readonly url?: string;
  readonly apiKey?: string;
  readonly batch?: number;
  readonly retry?: {
    readonly maxRetries?: number;
    readonly initialBackoffMs?: number;
    readonly attemptTimeoutMs?: number;
  };
}
export interface CallOptions {
  readonly evidence?: string | KeyPath;
  readonly name?: string; readonly signal?: AbortSignal;
}
export interface DecisionOptions extends CallOptions { readonly threshold?: Threshold }
export interface NumericOptions extends CallOptions { readonly threshold?: number }
export interface ChooseOptions<C extends readonly string[]> extends DecisionOptions { readonly choices: C }
export interface TagOptions<L extends readonly string[]> extends NumericOptions { readonly labels: L }
export interface ScoreOptions extends CallOptions { readonly levels: readonly string[] }
export interface RankOptions extends ScoreOptions { readonly top?: number }

export interface Source {
  readonly questionIdentity: string;
  readonly name?: string;
  readonly backend: Backend;
  readonly model: string;
}
export interface ErrorInfo {
  readonly code: RowErrorCode;
  readonly message: string;
  readonly httpStatus?: number;
}
export type Outcome<A, D = never> =
  | { readonly status: "answered"; readonly answer: A;
      readonly probability?: number; readonly detail?: D }
  | { readonly status: "unsure"; readonly probability: number; readonly detail?: D }
  | { readonly status: "failed"; readonly error: ErrorInfo };
export type Row<F extends FunctionName, A, D = never> = Outcome<A, D> & {
  readonly function: F;
  readonly position: number;
  readonly id?: string | number | bigint;
  readonly source: Source;
};
export type DecideRow = Row<"decide", boolean>;
export type ChooseRow<C extends string = string> = Row<"choose", C, LabelsDetail<C>>;
export type TagRow<L extends string = string> = Row<"tag", readonly L[], LabelsDetail<L>>;
export type ScoreRow = Row<"score", number, LevelsDetail>;
export type FilterRow = Row<"filter", boolean>;
export type RankRow = Row<"rank", number, LevelsDetail>;
export type FindRow = Row<"find", Span | null>;
export type RecognizeRow<K extends string = string> = Row<"recognize", readonly RecognizedName<K>[], RecognizeDetail>;
export type RelateRow<R extends string = string> = Row<"relate", readonly Relation<R>[], RelateDetail<R>>;
export interface Span { readonly text: string; readonly start: number; readonly end: number }
export interface NamedSpan<K extends string = string> extends Span { readonly kind: K }
export interface RecognizedName<K extends string = string> extends NamedSpan<K> { readonly probability: number }
export interface Relation<R extends string = string> {
  readonly source: NamedSpan;
  readonly target: NamedSpan;
  readonly relation: R;
  readonly probability: number;
  readonly symmetric?: boolean;
}
export interface RelationRule {
  readonly name: string;
  readonly source: string;
  readonly target: string;
  readonly phrase?: string;
  readonly symmetric?: boolean;
}
export interface LabelsDetail<L extends string = string> { readonly labels: Readonly<Record<L, number>> }
export interface LevelsDetail { readonly levels: readonly number[] }
export interface RecognizeDetail {
  readonly pieces: readonly (Span & { readonly tags: Readonly<Record<"BEGIN" | "INSIDE" | "END" | "SINGLE" | "OUT", number>> })[];
  readonly names: readonly {
    readonly start: number;
    readonly end: number;
    readonly kinds: Readonly<Record<string, number>>;
  }[];
}
export interface RelateDetail<R extends string = string> { readonly names: readonly RecognizedName[]; readonly pairs: readonly Relation<R>[] }

type QuestionFields = { readonly evidence?: string | KeyPath };
type DecisionFields = QuestionFields & { readonly threshold?: Threshold };
type NumericFields = QuestionFields & { readonly threshold?: number };
export type DecideQuestion = DecisionFields & { readonly function: "decide"; readonly question: string };
export type FilterQuestion = DecisionFields & { readonly function: "filter"; readonly question: string };
export type ChooseQuestion<C extends readonly string[] = readonly string[]> = DecisionFields & {
  readonly function: "choose"; readonly question: string; readonly choices: C;
};
export type TagQuestion<L extends readonly string[] = readonly string[]> = NumericFields & {
  readonly function: "tag"; readonly question: string; readonly labels: L;
};
export type ScoreQuestion = QuestionFields & { readonly function: "score"; readonly question: string; readonly levels: readonly string[] };
export type RankQuestion = QuestionFields & { readonly function: "rank"; readonly question: string; readonly levels: readonly string[]; readonly top?: number };
export type FindQuestion = NumericFields & { readonly function: "find"; readonly question: string };
export type RecognizeQuestion<K extends readonly string[] = readonly string[]> = NumericFields & { readonly function: "recognize"; readonly kinds: K };
export type RelateQuestion<R extends readonly RelationRule[] = readonly RelationRule[]> = NumericFields & { readonly function: "relate"; readonly rules: R };
export type ToolboxQuestion = DecideQuestion | FilterQuestion | ChooseQuestion | TagQuestion
  | ScoreQuestion | RankQuestion | FindQuestion | RecognizeQuestion | RelateQuestion;
export type Toolbox = Readonly<Record<string, ToolboxQuestion>>;
/** Load saved question toolbox JSON through shared engine admission. */
export function parseToolbox(json: string): Toolbox;
/** Save a toolbox for another SDK or the command. */
export function serializeToolbox(toolbox: Toolbox): string;

export type AnnotationQuestion = Exclude<ToolboxQuestion, RankQuestion>;
export type AnnotationQuestions = Readonly<Record<string, AnnotationQuestion>>;
export type EntryOf<Q extends AnnotationQuestion> =
  Q extends ChooseQuestion<infer C> ? Outcome<C[number], LabelsDetail<C[number]>> & { readonly function: "choose" } :
  Q extends TagQuestion<infer L> ? Outcome<readonly L[number][], LabelsDetail<L[number]>> & { readonly function: "tag" } :
  Q extends RecognizeQuestion<infer K> ? Outcome<readonly RecognizedName<K[number]>[], RecognizeDetail> & { readonly function: "recognize" } :
  Q extends RelateQuestion<infer R> ? Outcome<readonly Relation<R[number]["name"]>[], RelateDetail<R[number]["name"]>> & { readonly function: "relate" } :
  Q extends DecideQuestion | FilterQuestion ? Outcome<boolean> & { readonly function: Q["function"] } :
  Q extends ScoreQuestion ? Outcome<number, LevelsDetail> & { readonly function: Q["function"] } :
  Q extends FindQuestion ? Outcome<Span | null> & { readonly function: "find" } : never;
export type AnnotationOf<T extends AnnotationQuestions> = { readonly [N in keyof T]: EntryOf<T[N]> };
export type AnnotateRow<T extends AnnotationQuestions = AnnotationQuestions> = Row<"annotate", AnnotationOf<T>>;
export type DeepReadonly<T> = T extends object
  ? { readonly [P in keyof T]: DeepReadonly<T[P]> } : T;
export function defineToolbox<const T extends Toolbox>(value: T): DeepReadonly<T>;

export type FunctionName = "decide" | "choose" | "tag" | "score" | "filter" | "rank" | "find" | "recognize" | "relate" | "annotate";
export type AnyRow = DecideRow | ChooseRow | TagRow | ScoreRow | FilterRow | RankRow | FindRow | RecognizeRow | RelateRow | AnnotateRow;
export type SignalOptions = Pick<CallOptions, "signal" | "name">;
export interface RowStream<R> extends AsyncIterableIterator<R>, AsyncDisposable { return(): Promise<IteratorResult<R>>; toArray(): Promise<R[]>; [Symbol.asyncDispose](): Promise<void>; }
export interface FilterItem<T> { readonly item: T; readonly row: FilterRow; readonly kept: boolean; }
export function decide(question: string, record: RecordInput, options?: DecisionOptions): Promise<DecideRow>;
export function decide(question: DecideQuestion, record: RecordInput, options?: SignalOptions): Promise<DecideRow>;
export function decideStream(question: string, records: Records, options?: DecisionOptions): RowStream<DecideRow>;
export function decideStream(question: DecideQuestion, records: Records, options?: SignalOptions): RowStream<DecideRow>;
export function choose<const C extends readonly string[]>(question: string, record: RecordInput, options: ChooseOptions<C>): Promise<ChooseRow<C[number]>>;
export function choose<const C extends readonly string[]>(question: ChooseQuestion<C>, record: RecordInput, options?: SignalOptions): Promise<ChooseRow<C[number]>>;
export function chooseStream<const C extends readonly string[]>(question: string, records: Records, options: ChooseOptions<C>): RowStream<ChooseRow<C[number]>>;
export function chooseStream<const C extends readonly string[]>(question: ChooseQuestion<C>, records: Records, options?: SignalOptions): RowStream<ChooseRow<C[number]>>;
export function tag<const L extends readonly string[]>(question: string, record: RecordInput, options: TagOptions<L>): Promise<TagRow<L[number]>>;
export function tag<const L extends readonly string[]>(question: TagQuestion<L>, record: RecordInput, options?: SignalOptions): Promise<TagRow<L[number]>>;
export function tagStream<const L extends readonly string[]>(question: string, records: Records, options: TagOptions<L>): RowStream<TagRow<L[number]>>;
export function tagStream<const L extends readonly string[]>(question: TagQuestion<L>, records: Records, options?: SignalOptions): RowStream<TagRow<L[number]>>;
export function score(question: string, record: RecordInput, options: ScoreOptions): Promise<ScoreRow>;
export function score(question: ScoreQuestion, record: RecordInput, options?: SignalOptions): Promise<ScoreRow>;
export function scoreStream(question: string, records: Records, options: ScoreOptions): RowStream<ScoreRow>;
export function scoreStream(question: ScoreQuestion, records: Records, options?: SignalOptions): RowStream<ScoreRow>;
export function find(question: string, record: RecordInput, options?: NumericOptions): Promise<FindRow>;
export function find(question: FindQuestion, record: RecordInput, options?: SignalOptions): Promise<FindRow>;
export function findStream(question: string, records: Records, options?: NumericOptions): RowStream<FindRow>;
export function findStream(question: FindQuestion, records: Records, options?: SignalOptions): RowStream<FindRow>;
export function recognize<const K extends readonly string[]>(question: K, record: RecordInput, options?: NumericOptions): Promise<RecognizeRow<K[number]>>;
export function recognize<const K extends readonly string[]>(question: RecognizeQuestion<K>, record: RecordInput, options?: SignalOptions): Promise<RecognizeRow<K[number]>>;
export function recognizeStream<const K extends readonly string[]>(question: K, records: Records, options?: NumericOptions): RowStream<RecognizeRow<K[number]>>;
export function recognizeStream<const K extends readonly string[]>(question: RecognizeQuestion<K>, records: Records, options?: SignalOptions): RowStream<RecognizeRow<K[number]>>;
export function relate<const R extends readonly RelationRule[]>(question: R, record: RecordInput, options?: NumericOptions): Promise<RelateRow<R[number]["name"]>>;
export function relate<const R extends readonly RelationRule[]>(question: RelateQuestion<R>, record: RecordInput, options?: SignalOptions): Promise<RelateRow<R[number]["name"]>>;
export function relateStream<const R extends readonly RelationRule[]>(question: R, records: Records, options?: NumericOptions): RowStream<RelateRow<R[number]["name"]>>;
export function relateStream<const R extends readonly RelationRule[]>(question: RelateQuestion<R>, records: Records, options?: SignalOptions): RowStream<RelateRow<R[number]["name"]>>;
export function annotate<const T extends AnnotationQuestions>(question: T, record: RecordInput, options?: CallOptions): Promise<AnnotateRow<T>>;
export function annotateStream<const T extends AnnotationQuestions>(question: T, records: Records, options?: CallOptions): RowStream<AnnotateRow<T>>;
export function filterStream<T extends RecordInput>(question: string, records: Records<T>, options?: DecisionOptions): RowStream<FilterItem<T>>;
export function filterStream<T extends RecordInput>(question: FilterQuestion, records: Records<T>, options?: SignalOptions): RowStream<FilterItem<T>>;
export function rankStream(question: string, records: Records, options: RankOptions): RowStream<RankRow>;
export function rankStream(question: RankQuestion, records: Records, options?: SignalOptions): RowStream<RankRow>;

export declare class ThinkThen implements AsyncDisposable {
 constructor(options?: ClientOptions);
 close(): Promise<void>;
 [Symbol.asyncDispose](): Promise<void>;
 readonly decide: typeof decide;
 readonly decideStream: typeof decideStream;
 readonly choose: typeof choose;
 readonly chooseStream: typeof chooseStream;
 readonly tag: typeof tag;
 readonly tagStream: typeof tagStream;
 readonly score: typeof score;
 readonly scoreStream: typeof scoreStream;
 readonly find: typeof find;
 readonly findStream: typeof findStream;
 readonly recognize: typeof recognize;
 readonly recognizeStream: typeof recognizeStream;
 readonly relate: typeof relate;
 readonly relateStream: typeof relateStream;
 readonly annotate: typeof annotate;
 readonly annotateStream: typeof annotateStream;
 readonly filterStream: typeof filterStream;
 readonly rankStream: typeof rankStream;
}
export { ERROR_CODES } from "./errors.js";
import type { ErrorCode, RowErrorCode, UsageCode, BackendCode } from "./errors.js";
export type { ErrorCode, RowErrorCode, UsageCode, BackendCode } from "./errors.js";
export declare class ThinkThenError extends Error { constructor(error: { readonly code: ErrorCode; readonly message: string; readonly httpStatus?: number }, options?: ErrorOptions); readonly code: ErrorCode; readonly httpStatus?: number; }
export declare class UsageError extends ThinkThenError { constructor(error: { readonly code: UsageCode; readonly message: string; readonly httpStatus?: number }, options?: ErrorOptions); readonly code: UsageCode; }
export declare class BackendError extends ThinkThenError { constructor(error: { readonly code: BackendCode; readonly message: string; readonly httpStatus?: number }, options?: ErrorOptions); readonly code: BackendCode; }
export declare class CancelledError extends ThinkThenError { constructor(error: { readonly code: "local.cancelled"; readonly message: string }, options?: ErrorOptions); readonly code: "local.cancelled"; }

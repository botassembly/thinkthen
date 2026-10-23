/** JSON data, with readonly containers on returned rows. */
export type JsonValue = null | boolean | number | string | readonly JsonValue[] | JsonObject;
export interface JsonObject { readonly [key: string]: JsonValue }

/** A string supplies text; every other value supplies a JSON record. */
export type RecordInput = null | boolean | number | string | object;
export type Records = Iterable<RecordInput> | AsyncIterable<RecordInput>;
export type Question<T> = T | { readonly type: "inline"; readonly value: T }
  | { readonly type: "toolbox"; readonly value: JsonObject; readonly name?: string }
  | { readonly type: "reference"; readonly name: string };
export type Threshold = number | `${number}:${number}`;

export interface ClientOptions {
  readonly mode?: "embedded" | "proxy";
  readonly keySource?: string;
  readonly adapter?: string;
  readonly url?: string;
  readonly model?: string;
  readonly batch?: number;
}
export interface CallOptions {
  readonly on?: string;
  readonly signal?: AbortSignal;
}
export interface QuestionOptions extends CallOptions { readonly threshold?: Threshold }
export interface ChooseOptions extends QuestionOptions { readonly options?: readonly string[] }
export interface TagOptions extends QuestionOptions { readonly labels?: readonly string[] }
export interface ScoreOptions extends CallOptions { readonly levels?: readonly string[] }
export interface RankOptions extends ScoreOptions { readonly top?: number }

/** Answer span offsets count UTF-16 code units, with an exclusive end, for String.slice. */
export interface Span {
  readonly text: string;
  readonly start: number;
  readonly end: number;
}
export interface NamedSpan extends Span { readonly kind: string }
export interface RecognizedName extends NamedSpan { readonly p: number }
export interface Relation {
  readonly source: NamedSpan;
  readonly target: NamedSpan;
  readonly relation: string;
  readonly p: number;
  readonly either?: boolean;
}
export interface RelationRule {
  readonly name: string;
  readonly source: string;
  readonly target: string;
  readonly reads?: string;
  readonly either?: boolean;
}
export interface ErrorInfo { readonly code: string; readonly message: string }
export type Outcome<A> =
  | { readonly status: "answered"; readonly answer: A; readonly p?: number; readonly detail?: JsonValue }
  | { readonly status: "unsure"; readonly answer: null; readonly p?: number; readonly detail?: JsonValue }
  | { readonly status: "failed"; readonly error: ErrorInfo };
export interface Annotation { readonly [question: string]: Outcome<Answer> }
export type Answer = boolean | string | number | readonly string[] | Span | null
  | readonly RecognizedName[] | readonly Relation[] | Annotation;
export type Row<A = Answer> = Outcome<A> & { readonly id: JsonValue };

/** Admission and call failures preserve the engine's code and message. */
export declare class ThinkThenError extends Error { constructor(error: ErrorInfo); readonly code: string }
export declare class UsageError extends ThinkThenError {}
export declare class BackendError extends ThinkThenError {}
export declare class LocalError extends ThinkThenError {}

/** Question text stays literal, including text ending in .json. */
export declare function decide(question: Question<string>, record: RecordInput, options?: QuestionOptions): Promise<Row<boolean>>;
export declare function decideStream(question: Question<string>, records: Records, options?: QuestionOptions): AsyncIterable<Row<boolean>>;
export declare function choose(question: Question<string>, record: RecordInput, options?: ChooseOptions): Promise<Row<string>>;
export declare function chooseStream(question: Question<string>, records: Records, options?: ChooseOptions): AsyncIterable<Row<string>>;
export declare function tag(question: Question<string>, record: RecordInput, options?: TagOptions): Promise<Row<readonly string[]>>;
export declare function tagStream(question: Question<string>, records: Records, options?: TagOptions): AsyncIterable<Row<readonly string[]>>;
export declare function score(question: Question<string>, record: RecordInput, options?: ScoreOptions): Promise<Row<number>>;
export declare function scoreStream(question: Question<string>, records: Records, options?: ScoreOptions): AsyncIterable<Row<number>>;
/** Yield the original records kept by the engine, unchanged and in input order. Failures throw. */
export declare function filterStream<T extends RecordInput>(question: Question<string>, records: Iterable<T> | AsyncIterable<T>, options?: QuestionOptions): AsyncIterable<T>;
/** Rank emits rows in rank order, preserving input order for ties. */
export declare function rankStream(question: Question<string>, records: Records, options?: RankOptions): AsyncIterable<Row<number>>;
export declare function find(question: Question<string>, record: RecordInput, options?: QuestionOptions): Promise<Row<Span | null>>;
export declare function findStream(question: Question<string>, records: Records, options?: QuestionOptions): AsyncIterable<Row<Span | null>>;
/** A toolbox is a plain object using the toolbox JSON grammar. */
export declare function annotate(toolbox: JsonObject, record: RecordInput, options?: CallOptions): Promise<Row<Annotation>>;
export declare function annotateStream(toolbox: JsonObject, records: Records, options?: CallOptions): AsyncIterable<Row<Annotation>>;
/** An empty kinds array recognizes names without restricting their kind. */
export declare function recognize(kinds: Question<readonly string[]>, record: RecordInput, options?: QuestionOptions): Promise<Row<readonly RecognizedName[]>>;
export declare function recognizeStream(kinds: Question<readonly string[]>, records: Records, options?: QuestionOptions): AsyncIterable<Row<readonly RecognizedName[]>>;
export declare function relate(rules: Question<readonly RelationRule[]>, record: RecordInput, options?: QuestionOptions): Promise<Row<readonly Relation[]>>;
export declare function relateStream(rules: Question<readonly RelationRule[]>, records: Records, options?: QuestionOptions): AsyncIterable<Row<readonly Relation[]>>;

/** One client reuses its backend connections and settings across calls. */
export declare class ThinkThen {
  constructor(options?: ClientOptions);
  readonly decide: typeof decide;
  readonly decideStream: typeof decideStream;
  readonly choose: typeof choose;
  readonly chooseStream: typeof chooseStream;
  readonly tag: typeof tag;
  readonly tagStream: typeof tagStream;
  readonly score: typeof score;
  readonly scoreStream: typeof scoreStream;
  readonly filterStream: typeof filterStream;
  readonly rankStream: typeof rankStream;
  readonly find: typeof find;
  readonly findStream: typeof findStream;
  readonly annotate: typeof annotate;
  readonly annotateStream: typeof annotateStream;
  readonly recognize: typeof recognize;
  readonly recognizeStream: typeof recognizeStream;
  readonly relate: typeof relate;
  readonly relateStream: typeof relateStream;
}

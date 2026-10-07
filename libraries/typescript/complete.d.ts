import type * as C from './_complete.js';
export * from './_complete.js';
export interface QuestionSource {
  readonly role: 'atomic' | 'dynamic' | 'rank' | 'set' | 'find' | 'recognize' | 'relate';
  readonly body?: C.JsonValue;
  readonly raw?: string;
  readonly path?: string;
  readonly name?: string;
  readonly reference?: string;
  readonly none?: boolean;
}
export interface Content { readonly kind: 'text' | 'json' | 'images'; readonly value?: C.JsonValue; }
export interface Image { readonly media: 'image/png' | 'image/jpeg'; readonly bytes: readonly number[]; }
export interface Item {
  readonly content: Content;
  readonly images?: readonly Image[];
  readonly context?: Content;
  readonly options?: readonly { readonly name: string; readonly description?: C.Description }[];
}
export type Input = { readonly kind: 'records'; readonly records: readonly Item[] }
  | { readonly kind: 'files'; readonly paths: readonly string[]; readonly options: {
    readonly reading: { readonly unit: 'line' | 'window' | 'file'; readonly window?: number };
    readonly media: 'text' | 'image'; }; readonly jsonl?: boolean };
export interface Controls { readonly signal?: globalThis.AbortSignal; readonly deadlineMs?: number; readonly attempts?: boolean; readonly context?: C.QuestionText; }
export interface Completed<R> { readonly results: readonly R[]; readonly facts: C.Facts; readonly ordinals: readonly (number | null)[]; readonly inputs: readonly C.NativeInput[]; }
export class Functions {
  decideBatch(q:QuestionSource,input:Input,controls?:Controls):Batch<C.DecideResult>;
  chooseBatch(q:QuestionSource,input:Input,controls?:Controls):Batch<C.ChooseResult>;
  tagBatch(q:QuestionSource,input:Input,controls?:Controls):Batch<C.TagResult>;
  scoreBatch(q:QuestionSource,input:Input,controls?:Controls):Batch<C.ScoreResult>;
  filterBatch(q:QuestionSource,input:Input,controls?:Controls):Batch<C.FilterResult>;
  annotateBatch(q:QuestionSource,input:Input,controls?:Controls):Batch<C.AnnotateResult>;
  decide(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.DecideResult>>;
  choose(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.ChooseResult>>;
  tag(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.TagResult>>;
  score(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.ScoreResult>>;
  filter(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.FilterResult>>;
  rank(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.RankResult>>;
  find(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.FindResult>>;
  annotate(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.AnnotateResult>>;
  recognize(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.RecognizeResult>>;
  relate(q: QuestionSource, input: Input, controls?: Controls): Promise<Completed<C.RelateResult>>;
}

export interface BatchRow<R> {readonly result:R;readonly ordinal:number;readonly input:C.NativeInput;}
export class Batch<R> implements AsyncIterableIterator<BatchRow<R>> {
  readonly facts?:C.Facts;
  next():Promise<IteratorResult<BatchRow<R>>>;
  return():Promise<IteratorResult<BatchRow<R>>>;
  cancel():void;
  [Symbol.asyncIterator]():AsyncIterableIterator<BatchRow<R>>;
}

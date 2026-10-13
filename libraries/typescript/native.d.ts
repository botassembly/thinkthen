import type * as R from './results_generated.js';
export * as Results from './results_generated.js';
export type {EngineSettings, RequestOptions, RequestDefinition} from './results_generated.js';
export interface UsageStatus {
 readonly state: R.NativeUsagePersistence;
 readonly advice?: string;
}
export type Question = string | R.RequestDefinition | QuestionFile;
export interface QuestionFile { readonly _questionFile: unique symbol; }
export interface Source { readonly _source: unique symbol; }
export interface Item { readonly _item: unique symbol; }
export type CompleteInput = R.JsonValue | Source | Item | readonly (R.JsonValue | Item)[];
export type Input = R.JsonValue | Source | Item | readonly (R.JsonValue | Item)[] | Iterable<R.JsonValue | Item> | AsyncIterable<R.JsonValue | Item>;
export interface ImageBytes { readonly kind: "bytes"; readonly bytes: Uint8Array | string; readonly media?: string; }
export interface ItemFields { readonly images?: readonly ImageBytes[]; readonly context?: R.JsonValue; readonly options?: readonly {readonly name:string; readonly description?:R.JsonValue}[]; }
export interface Controls extends R.RequestOptions { readonly signal?: AbortSignal; }
export interface Completed extends R.AnswerReaders {}
export class Completed<T = R.NativeDecideResult | R.NativeChooseResult | R.NativeTagResult | R.NativeScoreResult | R.NativeFilterResult | R.NativeRankResult | R.NativeFindResult | R.NativeAnnotateResult | R.NativeRecognizeResult | R.NativeRelateResult> {
 readonly results: readonly T[];
 readonly terminal: R.NativeSessionPacketTerminal;
 readonly facts: R.NativeFacts | undefined;
}
export class Operation implements AsyncIterableIterator<R.NativeSessionPacket> {
 readonly terminal?: R.NativeSessionPacketTerminal;
 next():Promise<IteratorResult<R.NativeSessionPacket>>;
 result():Promise<Completed>;
 cancel():void;
 close():void;
 return():Promise<IteratorResult<R.NativeSessionPacket>>;
 [Symbol.asyncIterator]():AsyncIterableIterator<R.NativeSessionPacket>;
}
export interface Client {
 decideAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["decideAnswer"]>>;
 chooseAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["chooseAnswer"]>>;
 tagAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["tagAnswer"]>>;
 scoreAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["scoreAnswer"]>>;
 filterAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["filterAnswer"]>>;
 rankAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["rankAnswer"]>>;
 findAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["findAnswer"]>>;
 annotateAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["annotateAnswer"]>>;
 recognizeAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["recognizeAnswer"]>>;
 relateAnswer(q:Question,i:Input,c?:Controls):Promise<ReturnType<R.AnswerReaders["relateAnswer"]>>;
}
export class Client {
 constructor(settings?:R.EngineSettings);
 usagePersistence():UsageStatus;
 finishUsageStatus():UsageStatus;
 static files(paths:readonly string[],reading?:Readonly<Record<string,R.JsonValue>>,media?:string,framing?:string):Source;
 static item(value:R.JsonValue|undefined,fields?:ItemFields):Item;
 static questionFile(path:string):QuestionFile;
 static questionName(name:string):QuestionFile;
 static questionReference(reference:string):QuestionFile;
 plan(verb:string,question:Question,input:CompleteInput,controls?:Controls):R.NativePlan;
 start(verb:string,question:Question,input:Input,controls?:Controls):Operation;
 close():void;
 decide(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeDecideResult>>;
 choose(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeChooseResult>>;
 tag(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeTagResult>>;
 score(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeScoreResult>>;
 filter(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeFilterResult>>;
 rank(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeRankResult>>;
 find(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeFindResult>>;
 annotate(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeAnnotateResult>>;
 recognize(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeRecognizeResult>>;
 relate(q:Question,i:Input,c?:Controls):Promise<Completed<R.NativeRelateResult>>;
}

export class ClientError extends Error {
 constructor(kind:ClientError["kind"],message:string,retryable?:boolean);
 readonly kind:'usage'|'backend'|'local'|'cancelled'|'deadline'|'defect';
 readonly retryable:boolean;
 readonly complete?:R.NativeCallError;
 readonly facts?:R.NativeFacts;
 readonly terminal?:R.NativeSessionPacketTerminal;
 readonly results?:Completed['results'];
}

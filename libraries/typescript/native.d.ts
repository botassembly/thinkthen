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
export class Result<T = R.NativeDecideResult | R.NativeChooseResult | R.NativeTagResult | R.NativeScoreResult | R.NativeFilterResult | R.NativeRankResult | R.NativeFindResult | R.NativeAnnotateResult | R.NativeRecognizeResult | R.NativeRelateResult, V = R.JsonValue, P = number | null | readonly (number | null | readonly number[])[]> {
 readonly value: V;
 readonly probability: P;
 readonly results: readonly T[];
 readonly terminal: R.NativeSessionPacketTerminal;
 readonly facts: R.NativeFacts | undefined;
}
export class Operation implements AsyncIterableIterator<R.NativeSessionPacket> {
 readonly terminal?: R.NativeSessionPacketTerminal;
 next():Promise<IteratorResult<R.NativeSessionPacket>>;
 result():Promise<Result>;
 cancel():void;
 close():void;
 return():Promise<IteratorResult<R.NativeSessionPacket>>;
 [Symbol.asyncIterator]():AsyncIterableIterator<R.NativeSessionPacket>;
}
export interface Client extends R.NamedCalls {}
export class Client {
 constructor(settings?:R.EngineSettings);
 usagePersistence():UsageStatus;
 finishUsageStatus():UsageStatus;
 static files(paths:readonly string[],reading?:Readonly<Record<string,R.JsonValue>>,media?:string,framing?:string):Source;
 static item(value:R.JsonValue|undefined,fields?:ItemFields):Item;
 static recognizeQuestion(kinds:NonNullable<R.RecognizeQuestion["kinds"]>,options?:Omit<R.RecognizeQuestion,"kinds">):R.RecognizeRequestDefinition;
 static relateQuestion(relations:R.RelateQuestion["relations"],options?:Omit<R.RelateQuestion,"relations">):R.RelateDefinition;
 static questionFile(path:string):QuestionFile;
 static questionName(name:string):QuestionFile;
 static questionReference(reference:string):QuestionFile;
 plan(verb:string,question:Question,input:CompleteInput,controls?:Controls):R.NativePlan;
 start(verb:string,question:Question,input:Input,controls?:Controls):Operation;
 close():void;

}

export class ClientError extends Error {
 constructor(kind:ClientError["kind"],message:string,retryable?:boolean);
 readonly kind:'usage'|'backend'|'local'|'cancelled'|'deadline'|'defect';
 readonly retryable:boolean;
 readonly complete?:R.NativeCallError;
 readonly facts?:R.NativeFacts;
 readonly terminal?:R.NativeSessionPacketTerminal;
 readonly results?:Result['results'];
}

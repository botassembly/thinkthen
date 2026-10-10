import type * as R from './results_generated.js';
export * as Results from './results_generated.js';
export type Question = string | R.JsonValue | QuestionFile;
export interface QuestionFile { readonly _questionFile?: never; }
export interface Source { readonly _source?: never; }
export interface Item { readonly _item?: never; }
export type Input = R.JsonValue | Source | Item | readonly (R.JsonValue | Item)[] | Iterable<R.JsonValue | Item> | AsyncIterable<R.JsonValue | Item>;
export interface Controls { readonly signal?: AbortSignal; readonly [key:string]: unknown; }
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
export class Client {
 constructor(settings?:Readonly<Record<string,R.JsonValue>>);
 static files(paths:readonly string[],reading?:Readonly<Record<string,R.JsonValue>>,media?:string):Source;
 static item(value:R.JsonValue,fields?:Readonly<Record<string,R.JsonValue>>):Item;
 static questionFile(path:string):QuestionFile;
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
 readonly kind:'usage'|'backend'|'local'|'cancelled'|'deadline'|'defect';
 readonly code:number;
 readonly retryable:boolean;
 readonly complete?:R.NativeCallError;
 readonly facts?:R.NativeFacts;
 readonly terminal?:R.NativeSessionPacketTerminal;
 readonly results?:Completed['results'];
}

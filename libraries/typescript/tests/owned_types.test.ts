import {Client,ClientError,Results,type UsageStatus, type Completed, type Question, type Input} from 'thinkthen';
function usage(owner:Client) {
 const status:UsageStatus=owner.usagePersistence();
 const state:Results.NativeUsagePersistence=owner.finishUsageStatus().state;
 const advice:string|undefined=status.advice;
 // @ts-expect-error live status is immutable
 status.state='written';
 console.log(state,advice);
}
async function named(client:Client) {
 const done=await client.decide('Question?',false);
 const row:Results.NativeDecideResult=done.results[0];
 const id:string=row.answer_id;
 const original:Results.JsonValue|undefined=row.input;
 const value:Results.JsonValue=row.value;
 const facts:Results.NativeFacts|undefined=done.facts;
 const terminal:Results.NativeSessionPacketTerminal=done.terminal;
 if(terminal.has('failure')) console.log(terminal.failure?.error.kind);
 console.log(id,original,value,facts?.call_id);
}
function failure(error:unknown){if(error instanceof ClientError) {const facts:Results.NativeFacts|undefined=error.facts;console.log(error.complete?.error.kind,facts?.call_id,error.results?.length);}}
void [named,failure,usage];

const image=Client.item(undefined,{images:[{kind:"bytes",bytes:new Uint8Array([137,80,78,71]),media:"image/png"}]});
void image;

async function allNamed(client:Client,question:Question,input:Input) {
 const decide:Completed<Results.NativeDecideResult>=await client.decide(question,input);
 const choose:Completed<Results.NativeChooseResult>=await client.choose(question,input);
 const tag:Completed<Results.NativeTagResult>=await client.tag(question,input);
 const score:Completed<Results.NativeScoreResult>=await client.score(question,input);
 const filter:Completed<Results.NativeFilterResult>=await client.filter(question,input);
 const rank:Completed<Results.NativeRankResult>=await client.rank(question,input);
 const find:Completed<Results.NativeFindResult>=await client.find(question,input);
 const annotate:Completed<Results.NativeAnnotateResult>=await client.annotate(question,input);
 const recognize:Completed<Results.NativeRecognizeResult>=await client.recognize(question,input);
 const relate:Completed<Results.NativeRelateResult>=await client.relate(question,input);
 void [decide,choose,tag,score,filter,rank,find,annotate,recognize,relate];
}
void allNamed;

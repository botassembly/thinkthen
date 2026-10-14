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

async function oneLine(client:Client) {
 const decide:boolean|null=await client.decideAnswer('Question?','text');
 const choose:string|null=await client.chooseAnswer({choose:'Which?',options:['yes','no']},'text');
 const tag:readonly string[]=(await client.tag({tag:'Which?',labels:['yes']},'text')).tagAnswer();
 const score:number|Results.NativeNumber|null=await client.scoreAnswer({score:'How?',levels:['low','high']},'text');
 const rank:readonly (number|Results.NativeNumber)[]=await client.rankAnswer('Question?',['first','second']);
 const filter:readonly Results.JsonValue[]=await client.filterAnswer('Question?',['first',null]);
 void [decide,choose,tag,score,rank,filter];
}
void oneLine;

function typedControls() {
 const client=new Client({cache:false,timeout:1000,max_retries:2});
 const settings:import('thinkthen').EngineSettings={cache:false,refresh_cache:false};
 const controls:import('thinkthen').Controls={attempts:true,deadline_ms:-1,signal:new AbortController().signal};
 const question:import('thinkthen').RequestDefinition={choose:'Which?',options:['first','second']};
 void client.choose(question,'text',controls);
 // @ts-expect-error unknown engine setting
 new Client({typo:true});
 // @ts-expect-error timeout uses a number
 new Client({timeout:'slow'});
 // @ts-expect-error unknown call control
 void client.decide('Question?','text',{typo:true});
 // @ts-expect-error attempts uses a boolean
 void client.decide('Question?','text',{attempts:'yes'});
 // @ts-expect-error null is not an optional control
 void client.decide('Question?','text',{attempts:null});
 // @ts-expect-error authored question wording uses text
 const invalid:Question={choose:false};
 // @ts-expect-error input option arrays are immutable
 controls.field?.push('/body');
 // @ts-expect-error plain object cannot impersonate a file question
 const file:import('thinkthen').QuestionFile={};
 // @ts-expect-error one authored question selects one function
 const mixed:Question={decide:'Question?',choose:'Which?'};
 const recognize:Question=Client.recognizeQuestion({person:null},{mode:"whole"});
 const relate:Question=Client.relateQuestion([{name:'knows',source:'person',target:'person',either:false}]);
 void [settings,invalid,file,mixed,recognize,relate];client.close();
}
void typedControls;

function typedPlan(client:Client) {
 const plan:Results.NativePlan=client.plan('decide','Question?',['text']);
 const records:number|Results.NativeNumber=plan.records;
 const body:string|null=plan.first_body_utf8;
 // @ts-expect-error a preview requires a complete input
 client.plan('decide','Question?',(function*(){yield 'text';})());
 void [records,body];
}
void typedPlan;

// @ts-expect-error transport decoding stays inside the bridge
const privateParse:typeof Results.parse=undefined;
// @ts-expect-error callers receive typed packets from operations
const privatePacket:typeof Results.packet=undefined;

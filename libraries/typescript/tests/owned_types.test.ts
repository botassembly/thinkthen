import {Client,ClientError,Engine,Results,type UsageStatus} from 'thinkthen';
function usage(owner:Client|Engine) {
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

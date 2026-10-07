import * as tt from 'thinkthen';
import type * as C from '../_complete.js';
import type { QuestionSource, Input, Completed } from '../complete.js';
// The compiler has no @types/node dependency. Read the arbitrary fixture via native Node.
declare function require(name: string): { readFileSync(fd: number, encoding: string): string };
const document = JSON.parse(require('node:fs').readFileSync(0, 'utf8')) as {
  verb: string; question: QuestionSource; input: Input; settings: tt.EngineOptions; cancel?: boolean; deadline_ms?: number; incremental?: boolean; held_cancel?: boolean; shared_context?:string;
};
async function main() {
  const prefix:import("../complete.js").BatchRow<C.DecideResult>[]=[];
  try {
    const engine = new tt.Engine(document.settings);
    const signal = new AbortController();
    if (document.cancel) signal.abort();
    if(document.held_cancel) setTimeout(()=>signal.abort(),150);
    const control = {signal:signal.signal, deadlineMs:document.deadline_ms, attempts:true,...(document.shared_context==null?{}:{context:document.shared_context})};
    let done: Completed<C.Result>;
    const q=document.question, input=document.input;
    if(document.incremental) {
      const batch=engine.complete.decideBatch(q,input,control);
      for await(const row of batch) prefix.push(row);
      if(batch.facts===undefined) throw new Error('missing native facts');
      done={results:prefix.map(r=>r.result),facts:batch.facts,ordinals:prefix.map(r=>r.ordinal),inputs:prefix.map(r=>r.input)};
    } else switch (document.verb) {
      case 'decide': { const v=await engine.complete.decide(q,input,control); const p:number|undefined=v.results[0]?.answer.probability; done=v; break; }
      case 'choose': { const v=await engine.complete.choose(q,input,control); const p:Readonly<Record<string,number>>|undefined=v.results[0]?.answer.probabilities; done=v; break; }
      case 'tag': { const v=await engine.complete.tag(q,input,control); const p:Readonly<Record<string,number>>|undefined=v.results[0]?.answer.probabilities; done=v; break; }
      case 'score': { const v=await engine.complete.score(q,input,control); const p:Readonly<Record<string,number>>|undefined=v.results[0]?.answer.probabilities; done=v; break; }
      case 'filter': done=await engine.complete.filter(q,input,control); break;
      case 'rank': { const v=await engine.complete.rank(q,input,control); const place:number|undefined=v.results[0]?.value; done=v; break; }
      case 'find': done=await engine.complete.find(q,input,control); break;
      case 'annotate': { const v=await engine.complete.annotate(q,input,control); const a:C.AnnotationEntry|undefined=v.results[0]?.answers['ready']; done=v; break; }
      case 'recognize': { const v=await engine.complete.recognize(q,input,control); const end:number|undefined=v.results[0]?.value.entities[0]?.end; done=v; break; }
      case 'relate': { const v=await engine.complete.relate(q,input,control); const f:string|undefined=v.results[0]?.value[0]?.source.file; done=v; break; }
      default: throw new Error('unknown function');
    }
    const id:C.CallId=done.facts.call_id;
    const answer:C.AnswerId|undefined=done.results[0]?.answer_id;
    const image:number|undefined=done.inputs[0]?.images[0]?.width;
    if(id.length!==64||answer!==undefined&&answer.length!==64) throw new Error('native identity');
    // Output uses the values whose known fields were compiled above.
    globalThis.console.log(JSON.stringify(done));
  } catch(error) {
    if(!(error instanceof tt.ThinkThenError)) throw error;
    globalThis.console.log(JSON.stringify({error:error.complete??{kind:error.kind,message:error.message,retryable:error.retryable},facts:error.complete?.facts??error.facts,completed:prefix.length?{results:prefix.map(r=>r.result),facts:error.complete?.facts,ordinals:prefix.map(r=>r.ordinal),inputs:prefix.map(r=>r.input)}:undefined}));
  }
}
void main();

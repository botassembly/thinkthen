import * as tt from 'thinkthen';
import { readSync } from 'node:fs';
function line() {
  const bytes=new Uint8Array(65536),decoder=new TextDecoder();let text='';
  for (;;) {const n=readSync(0,bytes,0,bytes.length,null);if(n===0)break;text+=decoder.decode(bytes.subarray(0,n),{stream:true});if(text.includes('\n'))break;}
  return text.trimEnd();
}
const document=JSON.parse(line());
const prefix=[];
try {
  const engine = new tt.Engine(document.settings);
  const controller = new AbortController();
  if (document.cancel) controller.abort();
  if(document.held_cancel) setTimeout(()=>controller.abort(),150);
  const methods = {
    decide: engine.complete.decide.bind(engine.complete), choose:engine.complete.choose.bind(engine.complete),
    tag:engine.complete.tag.bind(engine.complete), score:engine.complete.score.bind(engine.complete),
    filter:engine.complete.filter.bind(engine.complete), rank:engine.complete.rank.bind(engine.complete),
    find:engine.complete.find.bind(engine.complete), annotate:engine.complete.annotate.bind(engine.complete),
    recognize:engine.complete.recognize.bind(engine.complete), relate:engine.complete.relate.bind(engine.complete),
  };
  let done;
  const controls={signal:controller.signal,deadlineMs:document.deadline_ms,attempts:true,...(document.shared_context==null?{}:{context:document.shared_context})};
  if(document.incremental) {
    const batch=engine.complete[document.verb+'Batch'](document.question,document.input,controls);
    if(document.batch_probe) {console.log('ready');line();}
    for await (const row of batch) prefix.push(row);
    done={results:prefix.map(r=>r.result),facts:batch.facts,ordinals:prefix.map(r=>r.ordinal),inputs:prefix.map(r=>r.input)};
  } else done=await methods[document.verb](document.question,document.input,controls);
  if (done.facts.call_id.length !== 64 || done.results.some(r=>r.answer_id.length!==64)) throw new Error('native identity');
  for(const result of done.results) for(const member of result.members??[]) {
    if(typeof member.name!=='string'||member.result.answer_id.length!==64||member.result.value<=0||member.result.question.verb!=='decide'||member.result.answer.kind!=='yes_no'||!member.result.meta) throw new Error('native rank member');
  }
  console.log(JSON.stringify(done));
} catch (error) {
  if (!(error instanceof tt.ThinkThenError)) throw error;
  console.log(JSON.stringify({error:error.complete??{kind:error.kind,message:error.message,retryable:error.retryable},facts:error.complete?.facts??error.facts,completed:prefix.length?{results:prefix.map(r=>r.result),facts:error.complete?.facts,ordinals:prefix.map(r=>r.ordinal),inputs:prefix.map(r=>r.input)}:undefined}));
}

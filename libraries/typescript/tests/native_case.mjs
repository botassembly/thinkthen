import * as tt from 'thinkthen';
import { readFileSync } from 'node:fs';
const document = JSON.parse(readFileSync(0, 'utf8'));
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
    const batch=engine.complete.decideBatch(document.question,document.input,controls);
    for await (const row of batch) prefix.push(row);
    done={results:prefix.map(r=>r.result),facts:batch.facts,ordinals:prefix.map(r=>r.ordinal),inputs:prefix.map(r=>r.input)};
  } else done=await methods[document.verb](document.question,document.input,controls);
  if (done.facts.call_id.length !== 64 || done.results.some(r=>r.answer_id.length!==64)) throw new Error('native identity');
  console.log(JSON.stringify(done));
} catch (error) {
  if (!(error instanceof tt.ThinkThenError)) throw error;
  console.log(JSON.stringify({error:error.complete??{kind:error.kind,message:error.message,retryable:error.retryable},facts:error.complete?.facts??error.facts,completed:prefix.length?{results:prefix.map(r=>r.result),facts:error.complete?.facts,ordinals:prefix.map(r=>r.ordinal),inputs:prefix.map(r=>r.input)}:undefined}));
}

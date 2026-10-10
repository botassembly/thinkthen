import {Client, Results, type Input, type Question} from 'thinkthen';
async function typed(client:Client,q:Question,input:Input) {
 const done=await client.recognize(q,input);
 const row:Results.NativeRecognizeResult|undefined=done.results[0];
 console.log(row?.value.entities);
}
void typed;
// Execute the same fixture consumer after the compiler checks named calls.
import './native_case.mjs';

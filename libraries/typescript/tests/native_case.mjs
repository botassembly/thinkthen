import * as esm from 'thinkthen';
import {createRequire} from 'node:module';
const tt=process.env.THINKTHEN_OWNED_MODULE==='cjs'?createRequire(import.meta.url)('thinkthen'):esm;
import { readSync } from 'node:fs';
function line() {
 const bytes=new Uint8Array(65536),decoder=new TextDecoder();let text='';
 for (;;) {const n=readSync(0,bytes,0,bytes.length,null);if(n===0)break;text+=decoder.decode(bytes.subarray(0,n),{stream:true});if(text.includes('\n'))break;}
 return text.trimEnd();
}
const document=JSON.parse(line());
let client;
try {
 client=new tt.Client(document.settings);
 const selector=document.question;
 const question=selector.kind==='file'?tt.Client.questionFile(selector.path):selector.kind==='name'?tt.Client.questionName(selector.name):selector.kind==='reference'?tt.Client.questionReference(selector.reference):selector.value;
 const source=document.input;
 const input=source.kind==='source'?tt.Client.files(source.source.paths,source.source.reading,source.source.media,source.source.framing):source.items.map(item=>tt.Client.item(item.original?.kind==='text'?item.original.text:item.original?.value,Object.fromEntries(Object.entries({...item,images:(item.images??[]).map(image=>image.kind==='bytes'?{...image,bytes:Uint8Array.from(Buffer.from(image.bytes,'base64'))}:image)}).filter(([key])=>key!=='original'))));
 const controller=new AbortController();
 if(document.cancel)controller.abort();
 if(document.held_cancel)setTimeout(()=>controller.abort(),150);
 if(document.batch_probe){console.log('ready');line();}
 const done=await client.start(document.verb,question,document.incremental && Array.isArray(input)?input.values():input,{...document.options,signal:controller.signal}).result();
 console.log(JSON.stringify({native:true,results:done.results,facts:done.facts}));
} catch(error) {
 if(!(error instanceof tt.ClientError))throw error;
 console.log(JSON.stringify({native:true,error:error.complete?.error??{kind:error.kind,message:error.message,retryable:error.retryable},facts:error.facts,completed:error.results?.length?{native:true,results:error.results,facts:error.facts}:undefined}));
} finally {client?.close();}

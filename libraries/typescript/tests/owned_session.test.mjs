import {test} from 'node:test';
import assert from 'node:assert/strict';
import {startBackend,ask as backendAsk,child as backendChild,within,sleep,until,INDEX} from './backend.mjs';

const face=process.env.THINKTHEN_OWNED_MODULE || 'esm';
const framed=body=>`const api = ${face==='cjs'?`(await import('node:module')).createRequire(${JSON.stringify(INDEX)})('thinkthen')`:"tt"};\n${body.replace(/\btt\./g,'api.')}`;
const ask=async(backend,body,options)=>{
  const reply=await backendAsk(backend,framed(body),options);
  assert.equal(reply.error,undefined,JSON.stringify(reply.error));return reply;
};
const child=(backend,body,options)=>backendChild(backend,framed(body),options);

test('Engine and Client report owned written usage without changing call facts',async t=>{
  const backend=await startBackend(t);
  for(const owner of ['Engine','Client']) {
    const {value}=await ask(backend,`
      const client=new tt.${owner}({cache:false});
      const done=await client.decide('Question?','text');
      const earlier=JSON.stringify(done);
      const written=client.finishUsageStatus(),observed=client.usagePersistence();
      if(!Object.isFrozen(written) || !Object.isFrozen(observed))throw Error('mutable status');
      if(JSON.stringify(done)!==earlier)throw Error('changed historical facts');
      if(client.close) {
        client.close();client.close();
        for(const method of ['usagePersistence','finishUsageStatus']) {
          try{client[method]();throw Error('closed status accepted');}
          catch(error){if(!(error instanceof tt.ClientError) || error.kind!=='usage' || error.message!=='client is closed')throw error;}
        }
      }
      return [written,observed,${owner==='Client'?'done.results[0].value':'done.value'}];
    `);
    assert.deepEqual(value,[{state:'written'},{state:'written'},true]);
  }
  assert.equal(await backend.count(),2);
});

test('Engine and Client latch held usage writer failure with safe owned advice',{skip:process.platform!=='linux'},async t=>{
  const backend=await startBackend(t);
  for(const owner of ['Engine','Client']) {
    const {value}=await ask(backend,`
      const {spawn}=await import('node:child_process');
      const lock=spawn('python3',['-c','import fcntl,os,pathlib,sys; p=pathlib.Path(os.environ["XDG_STATE_HOME"])/"thinkthen"; p.mkdir(parents=True,mode=0o700); f=(p/".lock").open("w"); os.chmod(p/".lock",0o600); fcntl.flock(f,fcntl.LOCK_EX); print("held",flush=True); sys.stdin.read()'],{env:process.env,stdio:['pipe','pipe','inherit']});
      const exited=new Promise(resolve=>lock.once('exit',resolve));
      await new Promise((resolve,reject)=>{lock.stdout.once('data',resolve);lock.once('error',reject);lock.once('exit',()=>reject(Error('lock holder exited')));});
      try {
        const client=new tt.${owner}({cache:false});
        const done=await client.decide('Question?','text');
        const earlier=JSON.stringify(done),pending=client.usagePersistence();
        const failed=client.finishUsageStatus();
        const repeated=[client.usagePersistence(),client.finishUsageStatus()];
        if(JSON.stringify(done)!==earlier)throw Error('changed historical facts');
        if(client.close)client.close();
        if(!Object.isFrozen(failed) || typeof failed.advice!=='string')throw Error('unowned advice');
        return [pending,failed,repeated,${owner==='Client'?'done.results[0].value':'done.value'}];
      } finally {lock.stdin.end();await exited;}
    `);
    const failed={state:'failed',advice:'check the usage folder permissions and free space'};
    assert.deepEqual(value,[{state:'pending'},failed,[failed,failed],true]);
  }
  assert.equal(await backend.count(),2);
});

test('Engine and Client report disabled usage when the environment selects no storage',async t=>{
  const backend=await startBackend(t);
  const {value}=await ask(backend,`
    return [tt.Engine,tt.Client].map(Owner=>{
      const client=new Owner({cache:false});
      const states=[client.usagePersistence(),client.finishUsageStatus()];
      if(client.close)client.close();
      return states;
    });
  `,{env:{HOME:'',XDG_STATE_HOME:'',LOCALAPPDATA:''}});
  assert.deepEqual(value,[[{state:'disabled'},{state:'disabled'}],[{state:'disabled'},{state:'disabled'}]]);
  assert.equal(await backend.count(),0);
});

test('the ten Client functions retain generated values after close',async t=>{
  const backend=await startBackend(t);
  const {value}=await ask(backend,`
    const client=new tt.Client({cache:false});
    const asks={
      decide:[{decide:'Is it urgent?'},'text'],
      choose:[{choose:'Which team?',options:['billing','shipping']},'text'],
      tag:[{tag:'Which labels?',labels:['money','shipping']},'text'],
      score:[{score:'How urgent?',levels:['low','mid','high']},'text'],
      filter:[{decide:'Is it urgent?'},['first',{note:'second'}]],
      rank:['Is it urgent?',['first','second']],
      find:['Which line?',['first','second']],
      annotate:[{version:1,questions:{ok:{decide:'Is it urgent?'}}},['text']],
      recognize:[{version:1,recognize:{kinds:{person:null}}},'Ana Lima'],
      relate:[{version:1,relate:{relations:[{name:'knows',source:'person',target:'person',either:false}]}},[{name:'Ana',kind:'person'},{name:'Bob',kind:'person'}]]
    };
    const held=[];
    for(const [verb,[q,input]] of Object.entries(asks)) held.push([verb,await client[verb](q,input,{attempts:true})]);
    client.close();
    for(const [,done] of held) {
      if(!(done.facts instanceof tt.Results.NativeFacts) || !Object.isFrozen(done.facts))throw Error('untyped facts');
      for(const row of done.results) {
        if(!row.constructor.name.startsWith('Native') || row.answer_id.length!==64 || !Object.isFrozen(row))throw Error('untyped row');
        if(JSON.parse(JSON.stringify(row)).answer_id!==row.answer_id)throw Error('lost identity');
      }
    }
    if(!(held[0][1].results[0].answer instanceof tt.Results.NativeAnswerYesNo) || !(held[0][1].results[0].answer.probability>0))throw Error('lost decision probabilities');
    if(!(held[1][1].results[0].answer instanceof tt.Results.NativeAnswerChoice) || !Object.values(held[1][1].results[0].answer.probabilities).every(v=>typeof v==='number'))throw Error('lost choice probabilities');
    if(typeof held[0][1].facts.largest_request_bytes!=='number' || !(held[0][1].facts.input_tokens>0) || !held[0][1].facts.token_estimate_method || !(held[0][1].facts.usage_persistence instanceof tt.Results.NativePersistenceObservation))throw Error('lost observed cost facts');
    return held.map(([verb,done])=>[verb,done.results.length,done.facts.requests_sent]);
  `,{arm:'arm/full/capture'});
  assert.deepEqual(value.map(row=>row[0]),['decide','choose','tag','score','filter','rank','find','annotate','recognize','relate']);
  assert.ok(value.every(([,rows,sends])=>rows>0 && sends>0));
  assert.ok(await backend.count()>=10);
});

test('Client retains false, null, missing, extensions and physical positions',async t=>{
  const backend=await startBackend(t);
  const {value}=await ask(backend,`
    const client=new tt.Client({cache:false});
    const no=await client.decide({decide:'Question?',threshold:0.95},'text');
    if(no.facts.has('input_tokens') || no.facts.has('output_tokens'))throw Error('invented observed tokens');
    const nil=await client.decide({decide:'Question?',true:null},'text');
    const uncertain=await client.decide({decide:'Question?',threshold:'0.2:0.95'},'text');
    const {writeFileSync}=await import('node:fs');
    const path=process.env.HOME+'/input.txt';writeFileSync(path,'first\\n\\nthird\\n');
    const located=await client.decide('Question?',tt.Client.files([path]));
    const exactDone=await client.decide('Question?',tt.Results.parse('9007199254740993'));client.close();
    const raw=JSON.parse(JSON.stringify(no.terminal));raw.future={flag:false,empty:null};
    const extended=tt.Results.packet(raw);
    return {values:[no.results[0].value,nil.results[0].value,uncertain.results[0].value],presence:[no.results[0].question.has('true'),nil.results[0].question.has('true'),nil.results[0].question.true],positions:located.results.map(r=>[r.index,r.source.first_line,r.source.last_line,r.input]),future:extended.future,nativeExact:[exactDone.results[0].input.raw,JSON.stringify(exactDone.results[0].input)],exact:(()=>{const p=tt.Results.parse('9007199254740993');return [p.raw,JSON.stringify(p)];})(),typed:extended instanceof tt.Results.NativeSessionPacketTerminal};
  `);
  assert.deepEqual(value.values,[false,null,null]);assert.deepEqual(value.presence,[false,true,null]);
  assert.deepEqual(value.positions,[[0,1,1,'first'],[1,3,3,'third']]);
  assert.deepEqual(value.future,{flag:false,empty:null});assert.equal(value.typed,true);assert.deepEqual(value.exact,['9007199254740993','9007199254740993']);assert.deepEqual(value.nativeExact,['9007199254740993','9007199254740993']);
});

test('Client reports native deadline settlement and invalid admission sends nothing',async t=>{
  const backend=await startBackend(t);
  const {value}=await ask(backend,`
    const client=new tt.Client({cache:false});const seen=[];
    try{await client.decide('Question?','text',{deadline_ms:0});}catch(error){if(!(error instanceof tt.ClientError))throw Error('untyped error');seen.push([error.kind,error.complete.constructor.name,error.complete.has('value'),error.terminal.kind,error.results,error.facts.requests_sent]);}
    for(const controls of [{field:['bad pointer']},{context:null},{signal:AbortSignal.abort()}]) {
      try{await client.decide('Question?','text',controls);}catch(error){seen.push([error.kind,error.facts??null]);}
    }
    client.close();return seen;
  `);
  assert.equal(value[0][0],'deadline');assert.equal(value[0][1],'NativeCallError');assert.deepEqual(value[0].slice(2),[false,'terminal',[],0]);
  assert.deepEqual(value.slice(1),[['usage',null],['usage',null],['cancelled',null]]);assert.equal(await backend.count(),0);
});

test('Client cancellation closes its feed and settles while provider replies remain held',async t=>{
  const backend=await startBackend(t);
  const run=child(backend,`
    const stop=new AbortController(),client=new tt.Client({cache:false,max_requests:1});
    let reads=0,closed=false,ticks=0;const timer=setInterval(()=>ticks++,1);
    const feed={ [Symbol.iterator](){return this;},next(){return {done:false,value:'record '+reads++};},return(){closed=true;return {done:true};} };
    process.stdin.once('data',()=>{process.stdin.destroy();stop.abort();});
    try{await client.decide('Question?',feed,{signal:stop.signal,batch:1});}
    catch(error){const before=reads;await new Promise(r=>setTimeout(r,20));client.close();clearInterval(timer);return [error.kind,closed,reads===before,ticks>0,error.facts??null];}
  `,{arm:'arm/held',stdin:true});
  assert.equal(await backend.wait(1),1);run.proc.stdin.write('abort\n');
  const exited=await within(run.exited,30000);
  assert.ok(exited,'child closes while provider is held');assert.equal(exited.code,0);
  assert.deepEqual(run.lines[0].value.value,['cancelled',true,true,true,null]);
  assert.equal(await backend.count(),1);backend.release();await sleep(30);assert.equal(await backend.count(),1);
});


test('Client retains completed native source rows and failure facts',async t=>{
  const backend=await startBackend(t);
  const {value}=await ask(backend,`
    const {writeFileSync}=await import('node:fs');
    const root=process.env.HOME,paths=[root+'/one.txt',root+'/two.txt',root+'/bad.txt'];
    writeFileSync(paths[0],'first');writeFileSync(paths[1],'second');writeFileSync(paths[2],new Uint8Array([255]));
    const client=new tt.Client({cache:false});
    try{await client.decide('Question?',tt.Client.files(paths,{unit:'file'}),{attempts:true,batch:1});throw Error('failure became value');}
    catch(error){client.close();return {kind:error.kind,rows:error.results.map(r=>[r.index,r.input,r.source.file,r.meta.model,r.meta.origin,r.answer_id.length]),sends:error.facts.requests_sent,id:error.facts.call_id.length,complete:error.complete.constructor.name};}
  `,{arm:'arm/full/capture'});
  assert.equal(value.kind,'usage');assert.equal(value.complete,'NativeCallError');assert.equal(value.sends,2);assert.equal(value.id,64);
  assert.deepEqual(value.rows.map(r=>r.slice(0,2)),[[0,'first'],[1,'second']]);
  assert.ok(value.rows.every(r=>r[2].endsWith('.txt')&&typeof r[3]==='string'&&r[4]==='live'&&r[5]===64));
  assert.equal(await backend.count(),2);
});


test('Client releases a rejected feed while retaining a held request and failure facts',async t=>{
  const backend=await startBackend(t);
  const run=child(backend,`
    const client=new tt.Client({cache:false,throttle:2});let reads=0,closed=0;
    async function* inputs(){
      try {
        reads++;yield {body:'first'};
        await new Promise(resolve=>process.stdin.once('data',resolve));
        reads++;yield {};
        for(let later=0;later<2;later++){reads++;if(reads===3)line({pending:true});yield {body:'later'};}
      } finally {closed++;line({closed,reads});throw Error('producer cleanup failed');}
    }
    try{await client.decide('Question?',inputs(),{batch:2,field:['/body']});throw Error('failure became value');}
    catch(error){
      client.close();process.stdin.destroy();
      if(!(error instanceof tt.ClientError))throw error;
      return {kind:error.kind,closed,rows:error.results.map(row=>row.input),sends:error.facts.requests_sent,id:error.facts.call_id.length,complete:error.complete.constructor.name};
    }
  `,{arm:'arm/held',stdin:true});
  assert.equal(await backend.wait(1),1);
  run.proc.stdin.write('continue\n');
  try {
    assert.ok(await until(()=>run.lines.length>0,5000),'another descriptor is produced while the earlier request is held');
    assert.deepEqual(run.lines[0].value,{pending:true});
    assert.ok(await until(()=>run.lines.some(row=>row.value.closed===1),5000),'closed intake releases its producer before provider settlement');
    assert.equal(await backend.count(),1);
  } finally {backend.release();}
  const exited=await within(run.exited,5000);assert.ok(exited);assert.equal(exited.code,0);
  assert.equal(run.lines.filter(row=>row.value.closed===1).length,1);
  assert.deepEqual(run.lines.at(-1).value,{value:{kind:'usage',closed:1,rows:[{body:'first'}],sends:1,id:64,complete:'NativeCallError'}});
  assert.equal(await backend.count(),1);
});

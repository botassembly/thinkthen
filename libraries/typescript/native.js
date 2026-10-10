'use strict';
const addon=require('./loader.js');
const Results=require('./results_generated.js');
const {ThinkThenError}=require('./index.js');
class ClientError extends ThinkThenError {}
const FUNCTIONS=['decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'];
const sourceTag=Symbol('source'), itemTag=Symbol('item'), questionTag=Symbol('question');
// Conversion refuses values JSON would silently omit or alter. Rust admits the request.
function dump(value) {
  const seen=new Set();
  function check(v) {
    if(v===null || typeof v==='boolean' || v instanceof Results.NativeNumber) return;
    if(typeof v==='string' && v.isWellFormed()) return;
    if(typeof v==='number' && Number.isFinite(v)) return;
    if(typeof v!=='object' || seen.has(v) || (!Array.isArray(v) && ![Object.prototype,null].includes(Object.getPrototypeOf(v)))) throw new ClientError('usage','input cannot be converted to native JSON');
    seen.add(v);
    if(Reflect.ownKeys(v).some(k=>typeof k==='symbol')) throw new ClientError('usage','input cannot be converted to native JSON');
    for(const child of Array.isArray(v)?v:Object.values(v)) check(child);
    seen.delete(v);
  }
  check(value);return JSON.stringify(value);
}
function crossing(call) {
  try {return call();} catch(error) {
    try {const held=JSON.parse(error.message).err; if(held) throw new ClientError(held.kind,held.message,held.retryable);} catch(parsed) {if(parsed instanceof ThinkThenError) throw parsed;}
    throw error;
  }
}
const original=value=>typeof value==='string'?{kind:'text',text:value}:{kind:'json',value};
const descriptor=value=>value?.[itemTag] || {original:original(value)};
class Completed {
  constructor(results,terminal) {this.results=Object.freeze(results);this.terminal=terminal;Object.freeze(this);}
  get facts(){return this.terminal.facts;}
}
class Client {
  #engine; #operations=new Set(); #closed=false;
  constructor(settings={}) {this.#engine=crossing(()=>addon.requestEngine(dump(settings)));}
  usagePersistence() {
    if(this.#closed) throw new ClientError('usage','client is closed');
    return Object.freeze(crossing(()=>this.#engine.usagePersistence()));
  }
  finishUsageStatus() {
    if(this.#closed) throw new ClientError('usage','client is closed');
    return Object.freeze(crossing(()=>this.#engine.finishUsageStatus()));
  }
  static files(paths,reading={unit:'line'},media='text'){return Object.freeze({[sourceTag]:{paths,reading,media}});}
  static item(value,fields={}) {return Object.freeze({[itemTag]:{original:original(value),...fields}});}
  static questionFile(path){return Object.freeze({[questionTag]:{kind:'file',path}});}
  start(verb,question,input,controls={}) {
    if(this.#closed) throw new ClientError('usage','client is closed');
    const operation=new Operation(this.#engine,verb,question,input,controls,op=>this.#operations.delete(op));
    this.#operations.add(operation);return operation;
  }
  close(){this.#closed=true;for(const op of this.#operations) op.close();this.#engine=null;}
}
class Operation {
  #native; #iterator; #pending; #rows=[]; #remove; #signal; #abort; #closed=false; #busy=false; #done=false;
  terminal;
  constructor(engine,verb,question,input,controls,remove) {
    this.#remove=remove;
    const {signal,...options}=controls;this.#signal=signal;
    if(signal && !(signal instanceof AbortSignal)) throw new ClientError('usage','signal is an AbortSignal');
    if(signal?.aborted) throw new ClientError('cancelled','the call was cancelled');
    let selected;
    if(input?.[sourceTag]) selected={kind:'source',source:input[sourceTag]};
    else if(Array.isArray(input)) selected={kind:verb==='find'?'units':verb==='relate'?'entities':'records',items:input.map(descriptor)};
    else if(typeof input!=='string' && (input?.[Symbol.asyncIterator] || input?.[Symbol.iterator])) {
      this.#iterator=input[Symbol.asyncIterator]?.() || input[Symbol.iterator]();selected={kind:'feed',name:'javascript'};
    } else selected={kind:'records',items:[descriptor(input)]};
    const asked=question?.[questionTag] || (typeof question==='string'?{kind:'text',text:question}:{kind:'definition',value:question});
    try {this.#native=crossing(()=>addon.requestSession(engine,dump({schema:Results.REQUEST_VERSION,call:{function:verb,question:asked,input:selected,options}})));}
    catch(error){this.close();throw error;}
    this.#abort=()=>this.cancel();signal?.addEventListener('abort',this.#abort,{once:true});
  }
  [Symbol.asyncIterator](){return this;}
  #feed() {
    if(!this.#iterator || this.#pending) return;
    const iterator=this.#iterator;
    this.#pending=Promise.resolve().then(()=>iterator.next()).then(value=>{
      if(this.#closed) return;
      if(value.done){this.#native.finish(null);this.#iterator=undefined;}
      else this.#pending={text:dump({item:descriptor(value.value)})};
    }).catch(()=>{if(!this.#closed){this.#native.finish('{"kind":"invalid_input"}');this.#releaseProducer();}}).finally(()=>{if(this.#pending instanceof Promise)this.#pending=undefined;});
  }
  async next() {
    if(this.#done) return {done:true,value:undefined};
    if(this.#busy) throw new ClientError('usage','a native operation has one pending next');
    this.#busy=true;
    try {
      for(;;) {
        if(this.#closed) throw new ClientError('cancelled','the call was cancelled');
        const text=crossing(()=>this.#native.poll());
        if(text==='end'){this.#done=true;this.close();return {done:true,value:undefined};}
        if(text) {
          const packet=Results.packet(Results.parse(text));
          if(packet.kind==='terminal') {
            this.terminal=packet;this.#done=true;this.close();
            if(packet.has('failure')) {
              const failure=packet.failure,error=new ClientError(failure.error.kind,failure.error.message,failure.error.retryable);
              error.complete=failure;error.facts=failure.facts;error.results=Object.freeze(this.#rows);error.terminal=packet;throw error;
            }
            return {done:true,value:undefined};
          }
          if(packet.kind==='row') this.#rows.push(packet.value);
          if(packet.kind==='aggregate') this.#rows=Array.isArray(packet.value)?[...packet.value]:[packet.value];
          return {done:false,value:packet};
        }
        if(this.#pending?.text) {
          const status=crossing(()=>this.#native.push(this.#pending.text));
          if(status!=='full') this.#pending=undefined;
          if(status==='closed') this.#releaseProducer();
        }
        this.#feed();
        await new Promise(resolve=>setTimeout(resolve,1));
      }
    } finally {this.#busy=false;}
  }
  async result(){try {while(!(await this.next()).done){}return new Completed(this.#rows,this.terminal);} finally {this.close();}}
  cancel(){this.#native?.cancel();this.close();}
  #releaseProducer(){
    const iterator=this.#iterator;this.#iterator=undefined;
    try {iterator?.return?.()?.catch?.(()=>{});}
    catch { /* Cleanup retains the call failure. */ }
  }
  close(){
    if(this.#closed)return;this.#closed=true;
    this.#signal?.removeEventListener('abort',this.#abort);
    try {this.#releaseProducer();}
    finally {this.#native?.close();this.#remove?.(this);}
  }
  async return(){this.#done=true;this.close();return {done:true,value:undefined};}
}
for(const verb of FUNCTIONS) Object.defineProperty(Client.prototype,verb,{value:function(q,i,c){return this.start(verb,q,i,c).result();}});
module.exports={Client,ClientError,Operation,Completed,Results};

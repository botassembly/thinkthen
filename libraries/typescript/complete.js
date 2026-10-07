'use strict';
const carriers = require('./_complete.js');
// Only the native addon interprets question grammar or reads named inputs.
class Functions {
  #invoke;
  #batch;
  #opened;
  constructor(invoke,batch,opened) { this.#invoke = invoke; this.#batch=batch; this.#opened=opened; }
  async #call(verb, question, input, controls = {}) {
    const { attempts = false, ...call } = controls;
    const request = { verb, question, input, attempts };
    const completed = await this.#invoke(JSON.stringify(request), call);
    const rows = Array.isArray(completed.value.results) ? completed.value.results : [completed.value.results];
    const kind = verb[0].toUpperCase() + verb.slice(1) + 'Result';
    return Object.freeze({results:Object.freeze(rows.map(row => carriers.decode(kind,row))), facts:carriers.decode('Facts',completed.value.facts), ordinals:Object.freeze(completed.value.ordinals), inputs:Object.freeze(completed.value.inputs.map(v => carriers.decode('NativeInput',v)))});
  }
  #stream(verb,question,input,controls={}) {
    return new Batch(this.#batch(JSON.stringify({verb,question,input,attempts:controls.attempts===true}),controls),verb[0].toUpperCase()+verb.slice(1)+'Result',controls.signal,this.#opened);
  }
  decideBatch(q,i,c) { return this.#stream('decide',q,i,c); }
  chooseBatch(q,i,c) { return this.#stream('choose',q,i,c); }
  tagBatch(q,i,c) { return this.#stream('tag',q,i,c); }
  scoreBatch(q,i,c) { return this.#stream('score',q,i,c); }
  filterBatch(q,i,c) { return this.#stream('filter',q,i,c); }
  annotateBatch(q,i,c) { return this.#stream('annotate',q,i,c); }

  decide(q,i,c) { return this.#call('decide',q,i,c); }
  choose(q,i,c) { return this.#call('choose',q,i,c); }
  tag(q,i,c) { return this.#call('tag',q,i,c); }
  score(q,i,c) { return this.#call('score',q,i,c); }
  filter(q,i,c) { return this.#call('filter',q,i,c); }
  rank(q,i,c) { return this.#call('rank',q,i,c); }
  find(q,i,c) { return this.#call('find',q,i,c); }
  annotate(q,i,c) { return this.#call('annotate',q,i,c); }
  recognize(q,i,c) { return this.#call('recognize',q,i,c); }
  relate(q,i,c) { return this.#call('relate',q,i,c); }
}

class Batch {
  #native; #kind; #ended=false; #busy=false; #signal; #stop; #opened;
  facts;
  constructor(native,kind,signal,opened) {
    this.#native=native;this.#kind=kind;this.#signal=signal;this.#opened=opened;
    this.#stop=()=>native.cancel();
    signal?.addEventListener('abort',this.#stop,{once:true});
    if(signal?.aborted) native.cancel();
  }
  [Symbol.asyncIterator]() {return this;}
  async next() {
    if(this.#ended) return {done:true,value:undefined};
    if(this.#busy) throw new Error('a complete batch has one pending next');
    this.#busy=true;
    try {
      const event=JSON.parse(await new Promise((resolve)=>this.#native.pull(resolve)));
      if(event.row) return {done:false,value:Object.freeze({result:carriers.decode(this.#kind,event.row),ordinal:event.ordinal,input:carriers.decode('NativeInput',event.input)})};
      await this.return();
      if(event.error) {
        this.facts=event.error.facts ? carriers.decode('Facts',event.error.facts):undefined;
        this.#opened(JSON.stringify({err:{...event.error,native_complete:event.error}}));
      }
      this.facts=carriers.decode('Facts',event.facts);
      return {done:true,value:undefined};
    } finally {this.#busy=false;}
  }
  cancel() {this.#native.cancel();}
  async return() {this.#ended=true;this.#signal?.removeEventListener('abort',this.#stop);this.#native.close();return {done:true,value:undefined};}
}
module.exports = { Functions, Batch, ...carriers };

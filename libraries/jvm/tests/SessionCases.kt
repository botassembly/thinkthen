import thinkthen.SessionCases
import thinkthen.Inputs
import thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking
fun main(args: Array<String>): Unit = runBlocking {
    if(thinkthen.CaseBatch.run(args) { data -> main(arrayOf("complete", thinkthen.SessionCases.write(data))) }) return@runBlocking
    val data = SessionCases.frame(args)
    try {
        KotlinEngine(thinkthen.SessionCases.settings(thinkthen.SessionCases.parse(data["engine_settings"] as String))).use { engine ->
            val question=SessionCases.question(data); val input=SessionCases.input(data); val options=SessionCases.options(data)
            if(SessionCases.manual(data)) {
                val session=engine.startSession(SessionCases.request(data,question,SessionCases.sessionInput(data,input),options))
                val feed=object:SessionCases.Feed {
                    override fun push(d:Inputs.RequestSessionDescriptor)=session.tryPush(d)
                    override fun finish(f:Inputs.RequestReaderFailure?)=session.finish(f)
                    override fun cancel()=session.cancel()
                    override fun waitReady(input:Boolean){runBlocking { session.waitReady(input) }}
                    override fun read():thinkthen.OwnedSession.Read {val read=session.tryRead();return thinkthen.OwnedSession.Read(read.packet,read.ended)}
                    override fun close()=session.close()
                }
                println(thinkthen.SessionCases.write(SessionCases.normalize(SessionCases.drain(feed,data,input),data)))
                return@use
            }
            val call = when(input) {
                is Inputs.RequestInputEntities -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
                is Inputs.RequestInputFeed -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
                is Inputs.RequestInputJson -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
                is Inputs.RequestInputRecords -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
                is Inputs.RequestInputSource -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
                is Inputs.RequestInputText -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
                is Inputs.RequestInputUnits -> when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            }
            }
            // Read the owning language's facts before projecting the shared fixture shape.
            call.terminal().facts().state()
            val output = SessionCases.normalize(SessionCases.javaCall(call.packets().map { thinkthen.SessionCases.tree(it) },thinkthen.SessionCases.tree(call.terminal())),data).toMutableMap()
            if(data["call_result_contract"] == true) output["call_result"] = mapOf("value" to thinkthen.SessionCases.tree(call.value), "probability" to call.probability, "results" to call.results.map { thinkthen.SessionCases.tree(it) }, "facts" to thinkthen.SessionCases.tree(call.facts))
            println(thinkthen.SessionCases.write(output))
        }
    } catch(failure: SessionFailure) { println(thinkthen.SessionCases.write(SessionCases.normalize(failure.call(),data))) }
    catch(failure: NativeFailure) { println(SessionCases.failure(failure.code,failure.message ?: "")) }
}

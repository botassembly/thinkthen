import thinkthen.SessionCases
import thinkthen.Inputs
import thinkthen.Json
import thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking
fun main(args: Array<String>) = runBlocking {
    val data = SessionCases.frame(args)
    try {
        KotlinEngine(Inputs.EngineSettings.fromJson(Json.parse(data["engine_settings"] as String))).use { engine ->
            val question=SessionCases.question(data); val input=SessionCases.input(data); val options=SessionCases.options(data)
            if(SessionCases.manual(data)) {
                val session=engine.startSession(SessionCases.request(data,question,SessionCases.sessionInput(data,input),options))
                val feed=object:SessionCases.Feed {
                    override fun push(d:Inputs.RequestSessionDescriptor)=session.tryPush(d)
                    override fun finish(f:Inputs.RequestReaderFailure?)=session.finish(f)
                    override fun cancel()=session.cancel()
                    override fun read():thinkthen.OwnedSession.Read {val read=session.tryRead();return thinkthen.OwnedSession.Read(read.packet?.let {SessionCases.packet(it.json)},read.ended)}
                    override fun close()=session.close()
                }
                println(Json.write(SessionCases.normalize(SessionCases.drain(feed,data,input),data)))
                return@use
            }
            val call = try { when(data["verb"]) {
                "decide"->engine.decide(question,input,options); "choose"->engine.choose(question,input,options)
                "tag"->engine.tag(question,input,options); "score"->engine.score(question,input,options)
                "filter"->engine.filter(question,input,options); "rank"->engine.rank(question,input,options)
                "find"->engine.find(question,input,options); "annotate"->engine.annotate(question,input,options)
                "recognize"->engine.recognize(question,input,options); "relate"->engine.relate(question,input,options)
                else->error("unknown fixture function")
            } } catch(failure: SessionFailure) { failure.call }
            // Read the owning language's facts before projecting the shared fixture shape.
            call.terminal.facts?.requestsSent
            println(Json.write(SessionCases.normalize(SessionCases.javaCall(call.packets.map { it.json },call.terminal.json),data)))
        }
    } catch(failure: NativeFailure) { println(SessionCases.failure(failure.code,failure.message ?: "")) }
}

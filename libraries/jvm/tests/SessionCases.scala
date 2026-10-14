import thinkthen.{SessionCases as Fixtures, Inputs}
import thinkthen.scala.*
import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.jdk.CollectionConverters.*
object SessionCases {
  def main(args: Array[String]): Unit = {
    if(thinkthen.CaseBatch.run(args, data => main(Array("complete", thinkthen.SessionCases.write(data))))) return
    val data=Fixtures.frame(args)
    try {
      val engine=new ScalaEngine(thinkthen.SessionCases.settings(thinkthen.SessionCases.parse(data.get("engine_settings").asInstanceOf[String])))
      try {
        val question=Fixtures.question(data);val input=Fixtures.input(data);val options=Fixtures.options(data)
        if(Fixtures.manual(data)) {
          val session=engine.startSession(Fixtures.request(data,question,Fixtures.sessionInput(data,input),options))
          val feed=new Fixtures.Feed {
            def push(d:Inputs.RequestSessionDescriptor):Int=session.tryPush(d)
            def finish(f:Inputs.RequestReaderFailure):Unit=session.finish(Option(f))
            def cancel():Unit=session.cancel()
            def waitReady(input:Boolean):Unit={Await.result(session.waitReady(input),30.seconds);()}
            def read():thinkthen.OwnedSession.Read={val read=session.tryRead();new thinkthen.OwnedSession.Read(read.packet.orNull,read.ended)}
            def close():Unit=session.close()
          }
          println(thinkthen.SessionCases.write(Fixtures.normalize(Fixtures.drain(feed,data,input),data)));return
        }
        val pending=data.get("verb") match {
          case "decide"=>engine.decide(question,input,options);case "choose"=>engine.choose(question,input,options)
          case "tag"=>engine.tag(question,input,options);case "score"=>engine.score(question,input,options)
          case "filter"=>engine.filter(question,input,options);case "rank"=>engine.rank(question,input,options)
          case "find"=>engine.find(question,input,options);case "annotate"=>engine.annotate(question,input,options)
          case "recognize"=>engine.recognize(question,input,options);case "relate"=>engine.relate(question,input,options)
          case _=>throw new IllegalArgumentException("unknown fixture function")
        }
        val call=Await.result(pending.result,30.seconds)
        call.terminal().facts().state()
        val output=new java.util.LinkedHashMap[String,Object](Fixtures.normalize(Fixtures.javaCall(call.packets().map(thinkthen.SessionCases.tree(_)).asJava,thinkthen.SessionCases.tree(call.terminal())),data))
        if(data.get("call_result_contract") == true) {
          val result = call
           output.put("call_result", Map("value" -> result.value.asInstanceOf[Option[?]].getOrElse(null), "probability" -> result.probability.asInstanceOf[Option[?]].getOrElse(null), "results" -> result.results.map(thinkthen.SessionCases.tree(_)).asJava, "facts" -> thinkthen.SessionCases.tree(result.facts)).asJava)
        }
        println(thinkthen.SessionCases.write(output))
      } finally engine.close()
    } catch {case failure:SessionFailure=>println(thinkthen.SessionCases.write(Fixtures.normalize(failure.call(),data))); case failure:NativeFailure=>println(Fixtures.failure(failure.code,failure.getMessage))}
  }
}

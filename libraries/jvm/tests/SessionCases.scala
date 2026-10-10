import thinkthen.{SessionCases as Fixtures, Inputs, Json}
import thinkthen.scala.*
import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.jdk.CollectionConverters.*
object SessionCases {
  def main(args: Array[String]): Unit = {
    val data=Fixtures.frame(args)
    try {
      val engine=new ScalaEngine(Inputs.EngineSettings.fromJson(Json.parse(data.get("engine_settings").asInstanceOf[String])))
      try {
        val question=Fixtures.question(data);val input=Fixtures.input(data);val options=Fixtures.options(data)
        if(Fixtures.manual(data)) {
          val session=engine.startSession(Fixtures.request(data,question,Fixtures.sessionInput(data,input),options))
          val feed=new Fixtures.Feed {
            def push(d:Inputs.RequestSessionDescriptor):Int=session.tryPush(d)
            def finish(f:Inputs.RequestReaderFailure):Unit=session.finish(f)
            def cancel():Unit=session.cancel()
            def read():thinkthen.OwnedSession.Read={val read=session.tryRead();new thinkthen.OwnedSession.Read(read.packet.map(p=>Fixtures.packet(p.json)).orNull,read.ended)}
            def close():Unit=session.close()
          }
          println(Json.write(Fixtures.normalize(Fixtures.drain(feed,data,input),data)));return
        }
        val pending=data.get("verb") match {
          case "decide"=>engine.decide(question,input,options);case "choose"=>engine.choose(question,input,options)
          case "tag"=>engine.tag(question,input,options);case "score"=>engine.score(question,input,options)
          case "filter"=>engine.filter(question,input,options);case "rank"=>engine.rank(question,input,options)
          case "find"=>engine.find(question,input,options);case "annotate"=>engine.annotate(question,input,options)
          case "recognize"=>engine.recognize(question,input,options);case "relate"=>engine.relate(question,input,options)
          case _=>throw new IllegalArgumentException("unknown fixture function")
        }
        val call=try Await.result(pending.result,30.seconds) catch {case failure:SessionFailure=>failure.call}
        call.terminal.facts.map(_.requestsSent)
        println(Json.write(Fixtures.normalize(Fixtures.javaCall(call.packets.map(_.json).asJava,call.terminal.json),data)))
      } finally engine.close()
    } catch {case failure:NativeFailure=>println(Fixtures.failure(failure.code,failure.getMessage))}
  }
}

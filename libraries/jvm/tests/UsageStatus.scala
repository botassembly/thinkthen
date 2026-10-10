import thinkthen.Inputs
import thinkthen.scala.{ScalaEngine, UsagePersistence}
import java.nio.file.*
import scala.concurrent.Await
import scala.concurrent.duration.*

object ScalaUsageStatus {
  def main(args: Array[String]): Unit = {
    val engine = new ScalaEngine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0))
    if (args(0) == "disabled") {
      assert(engine.usagePersistence().state == UsagePersistence.State.Disabled)
      assert(engine.finishUsageStatus().state == UsagePersistence.State.Disabled)
      engine.close(); return
    }
    assert(engine.usagePersistence().state == UsagePersistence.State.Written)
    val call = Await.result(engine.decide(new Inputs.RequestQuestionText().text("Is it?"), new Inputs.RequestInputText().text("usage-scala-" + args(0))).result, 10.seconds)
    assert(call.terminal.facts.get.requestsSent == 1)
    assert(engine.usagePersistence().state == UsagePersistence.State.Pending)
    Files.writeString(Path.of(args(1), "pending"), "")
    if (args(0) == "written") while (!Files.exists(Path.of(args(1), "released"))) Thread.sleep(5)
    val status = engine.finishUsageStatus()
    assert(status.state == (if (args(0) == "written") UsagePersistence.State.Written else UsagePersistence.State.Failed))
    assert(engine.usagePersistence() == status)
    engine.close()
    assert(call.packets.collect { case row: thinkthen.scala.Results.SessionPacketDecideRow => row.value.value.value }.contains(true))
    assert(call.terminal.facts.get.requestsSent == 1)
    assert(status.advice == (if (args(0) == "written") None else Some("check the usage folder permissions and free space")))
    try { engine.usagePersistence(); assert(false, "closed engine observed") } catch { case _: IllegalStateException => () }
    try { engine.finishUsageStatus(); assert(false, "closed engine finalized") } catch { case _: IllegalStateException => () }
    println("SCALA_USAGE_PASS")
  }
}

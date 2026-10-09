import thinkthen.scala.ScalaEngine
import thinkthen.Presence
import scala.concurrent.Await
import scala.concurrent.duration.*
import java.nio.file.{Files, Path}

object SessionScala {
  def main(args: Array[String]): Unit = {
    val engine = new ScalaEngine(Map("cache" -> false, "max_retries" -> 0))
    try {
      val question = Map[String, Any]("kind" -> "text", "text" -> "Is it?")
      def input(text: String) = Map[String, Any]("kind" -> "text", "text" -> text)
      val held = engine.decide(question, input("hold-jvm-scala"))
      val limit = System.nanoTime() + 10.seconds.toNanos
      while (!Files.exists(Path.of("barrier/arrived-hold-jvm-scala"))) {
        assert(System.nanoTime() < limit, "provider did not arrive")
        Thread.sleep(5)
      }
      val independent = Await.result(engine.decide(question, input("session-scala-independent")).result, 10.seconds)
      assert(independent.terminal().facts().state() == Presence.State.VALUE)
      val started = System.nanoTime()
      assert(held.cancel())
      try { Await.result(held.result, 2.seconds); throw new AssertionError("Future succeeded after cancellation") }
      catch { case _: java.util.concurrent.CancellationException => () }
      assert(System.nanoTime() - started < 2.seconds.toNanos)
      assert(!Files.exists(Path.of("barrier/release-hold-jvm-scala")))
      assert(independent.terminal().facts().value().requestsSent().intValueExact() == 1)
    } finally engine.close()
    println("INSTALLED_JVM_SCALA_SESSION_PASS")
  }
}

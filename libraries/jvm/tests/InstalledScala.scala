import thinkthen.Door
import thinkthen.ResultEnvelope
import java.nio.file.{Files, Path}
import scala.concurrent.ExecutionContext

@main def installedScala(): Unit = {
  given ExecutionContext = ExecutionContext.global
  if (sys.env.get("TT_PORTABLE_BATCH").contains("1")) {
    val door = new Door("""{"cache":false,"throttle":1,"max_retries":0}""")
    try {
      val request = """{"decide":"Is it relevant?","records":["alpha","café-5544","omega","line 2907","tail"],"details":true,"call":{"batch":"max"}}"""
      val result = new ScalaFacade(door).call(request)
      assert(ResultEnvelope.value(result).startsWith("["))
      Files.writeString(Path.of("portable-result.json"), result)
      println("PORTABLE_BATCH_SCALA_PASS")
    } finally door.close()
    return
  }
  val door = new Door()
  try {
    val answer = new ScalaFacade(door).decide("Is it?", "release-scala")
    assert(answer.outcome() == 1 && answer.probability() == 0.9)
    println("INSTALLED_SCALA_PASS")
  } finally door.close()
}

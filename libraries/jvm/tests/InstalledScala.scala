import thinkthen.Door
import thinkthen.Json
import java.nio.file.{Files, Path}
import scala.concurrent.ExecutionContext

@main def installedScala(): Unit = {
  given ExecutionContext = ExecutionContext.global
  if (sys.env.get("TT_PORTABLE_BATCH").contains("1")) {
    val door = new Door("""{"cache":false,"throttle":1,"max_retries":0}""")
    try {
      val request = """{"decide":"Is it relevant?","records":["alpha","café-5544","omega","line 2907","tail"],"details":true,"call":{"batch":"max"}}"""
      val result = new ScalaFacade(door).call(request)
      assert(Json.parseObject(result).get("value").isInstanceOf[java.util.List[?]])
      Files.writeString(Path.of("portable-result.json"), result)
      println("PORTABLE_BATCH_SCALA_PASS")
    } finally door.close()
    return
  }
  val door = new Door()
  try {
    val answer = new ScalaFacade(door).decide("Is it?", "release-scala")
    assert(answer.value().outcome() == 1 && answer.value().probability() == 0.9 && answer.facts().get("records").asInstanceOf[Number].longValue == 1L && answer.facts().get("requests_sent").asInstanceOf[Number].longValue == 1L)
    println("INSTALLED_SCALA_PASS")
  } finally door.close()
}

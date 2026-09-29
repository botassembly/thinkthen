import thinkthen.Door
import scala.concurrent.ExecutionContext

@main def installedScala(): Unit = {
  given ExecutionContext = ExecutionContext.global
  val door = new Door()
  try {
    val answer = new ScalaFacade(door).decide("Is it?", "release-scala")
    assert(answer.outcome() == 1 && answer.probability() == 0.9)
    println("INSTALLED_SCALA_PASS")
  } finally door.close()
}

import thinkthen.Door
import scala.concurrent.ExecutionContext

@main def scalaTypeCase(request: String): Unit = {
  given ExecutionContext = ExecutionContext.global
  val door = new Door()
  try println(new ScalaFacade(door).call(request))
  catch { case failure: Door.NativeFailure =>
    println(s"{\"failed\":{\"kind\":\"${failure.failure.kind().name.toLowerCase}\",\"code\":${failure.failure.code()}}}")
  } finally door.close()
}

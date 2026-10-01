import java.nio.file.{Files, Path}
import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.Door
import thinkthen.Door.Outcome

@main def questionFile(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val refund = Files.readString(Path.of("refund.json"))
    val text = "I would like to return this " +
      "and get my money back.\n"
    val isRefund = tt.decide(refund, text).value()
    assert(Door.outcome(isRefund) == Outcome.YES)
  }

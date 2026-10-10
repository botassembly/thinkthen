// Generated from the C header constants. Do not edit.
package thinkthen.scala
final case class UsagePersistence(state: UsagePersistence.State, advice: Option[String])
object UsagePersistence {
enum State { case Disabled, Failed, Pending, Written }
private[scala] def read(value: thinkthen.UsagePersistence): UsagePersistence =
UsagePersistence(State.valueOf(value.state().name()), Option(value.advice()))
}

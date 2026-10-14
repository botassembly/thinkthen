// Generated from the shared Rust request graph. Do not edit.
package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import java.util.concurrent.CompletableFuture
import scala.jdk.CollectionConverters.*
abstract class ScalaNamedCalls(protected val namedEngine: Engine) {
protected def run(native: CompletableFuture[Engine.OwnedCall]): Call
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.decide(question, input, options.orNull))
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.decide(question, input, options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.choose(question, input, options.orNull))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.choose(question, input, options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.tag(question, input, options.orNull))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.tag(question, input, options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.score(question, input, options.orNull))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.score(question, input, options))
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.filter(question, input, options.orNull))
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.filter(question, input, options))
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.rank(question, input, options.orNull))
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.rank(question, input, options))
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.find(question, input, options.orNull))
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.find(question, input, options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.annotate(question, input, options.orNull))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.annotate(question, input, options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.recognize(question, input, options.orNull))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.recognize(question, input, options))
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Option[Inputs.RequestOptions] = None): Call = run(namedEngine.relate(question, input, options.orNull))
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.relate(question, input, options))
}
final class OwnedCall private[scala](private val native: Engine.OwnedCall) {
def packets(): List[thinkthen.Results.SessionPacket] = native.packets().asScala.toList
def terminal(): thinkthen.Results.SessionPacketTerminal = native.terminal()
def decide(): Option[Boolean] = Option(native.decide())
def choose(): Option[String] = Option(native.choose())
def score(): java.math.BigDecimal = native.score()
def tag(): List[String] = native.tag().asScala.toList
def filter(): List[Any] = native.filter().asScala.toList
def rank(): List[java.math.BigInteger] = native.rank().asScala.toList
def find(): thinkthen.Results.Find = native.find()
def annotate(): List[thinkthen.Results.Annotation] = native.annotate().asScala.toList
def recognize(): List[thinkthen.Results.Recognition] = native.recognize().asScala.toList
def relate(): thinkthen.Results.Relation = native.relate()
}

// Generated from the shared Rust request graph. Do not edit.
package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import java.util.concurrent.CompletableFuture
abstract class ScalaNamedCalls(protected val namedEngine: Engine) {
protected def run(native: CompletableFuture[Engine.OwnedCall]): Call
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = decide(question, input, null)
final def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.decide(question, input, options))
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = choose(question, input, null)
final def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.choose(question, input, options))
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = tag(question, input, null)
final def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.tag(question, input, options))
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = score(question, input, null)
final def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.score(question, input, options))
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = filter(question, input, null)
final def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.filter(question, input, options))
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = rank(question, input, null)
final def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.rank(question, input, options))
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = find(question, input, null)
final def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.find(question, input, options))
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = annotate(question, input, null)
final def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.annotate(question, input, options))
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = recognize(question, input, null)
final def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.recognize(question, input, options))
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = relate(question, input, null)
final def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(namedEngine.relate(question, input, options))
}

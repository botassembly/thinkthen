// Generated from the shared Rust request graph. Do not edit.
package thinkthen.kotlin
import thinkthen.Engine
import thinkthen.Inputs
import java.util.concurrent.CompletableFuture
abstract class KotlinNamedCalls(protected val namedEngine: Engine) {
protected abstract suspend fun await(start: () -> CompletableFuture<Engine.OwnedCall>): Engine.OwnedCall
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.decide(question, input, options) }
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.choose(question, input, options) }
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.tag(question, input, options) }
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.score(question, input, options) }
suspend fun filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.filter(question, input, options) }
suspend fun rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.rank(question, input, options) }
suspend fun find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.find(question, input, options) }
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.annotate(question, input, options) }
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.recognize(question, input, options) }
suspend fun relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { namedEngine.relate(question, input, options) }
}

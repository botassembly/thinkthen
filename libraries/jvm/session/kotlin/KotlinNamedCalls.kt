// Generated from the shared Rust request graph. Do not edit.
package thinkthen.kotlin
import thinkthen.Engine
import thinkthen.Inputs
import java.util.concurrent.CompletableFuture
abstract class KotlinNamedCalls(protected val namedEngine: Engine) {
protected abstract suspend fun await(start: () -> CompletableFuture<Engine.OwnedCall>): OwnedCall
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.decide(question, input, options) }
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.choose(question, input, options) }
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.tag(question, input, options) }
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.score(question, input, options) }
suspend fun filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.filter(question, input, options) }
suspend fun rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.rank(question, input, options) }
suspend fun find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.find(question, input, options) }
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.annotate(question, input, options) }
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.recognize(question, input, options) }
suspend fun relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { namedEngine.relate(question, input, options) }
}
class OwnedCall internal constructor(private val native: Engine.OwnedCall) {
val packets: List<thinkthen.Results.SessionPacket> get() = native.packets().toList()
val terminal: thinkthen.Results.SessionPacketTerminal get() = native.terminal()
fun packets() = packets
fun terminal() = terminal
fun decide(): Boolean? = native.decide()
fun choose(): String? = native.choose()
fun score(): java.math.BigDecimal = native.score()
fun tag(): List<String> = native.tag().toList()
fun filter(): List<Any?> = native.filter().toList()
fun rank(): List<java.math.BigInteger> = native.rank().toList()
fun find(): thinkthen.Results.Find = native.find()
fun annotate(): List<thinkthen.Results.Annotation> = native.annotate().toList()
fun recognize(): List<thinkthen.Results.Recognition> = native.recognize().toList()
fun relate(): thinkthen.Results.Relation = native.relate()
}

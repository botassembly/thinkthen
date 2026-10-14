// Generated from the shared Rust call graph. Do not edit.
package thinkthen.kotlin
import thinkthen.Engine
import thinkthen.Inputs
import thinkthen.Results
import java.util.concurrent.CompletableFuture
abstract class KotlinNamedCalls(protected val namedEngine: Engine) {
protected abstract suspend fun <T> await(start: () -> CompletableFuture<T>): T
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions? = null): CallResult<Any?,java.math.BigDecimal?,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value(), native.probability(), native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions? = null): CallResult<Any?,java.math.BigDecimal?,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value(), native.probability(), native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions? = null): CallResult<List<Any?>,List<java.math.BigDecimal?>,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions? = null): CallResult<List<Any?>,List<java.math.BigDecimal?>,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions? = null): CallResult<List<Any?>,List<java.math.BigDecimal?>,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions? = null): CallResult<List<Any?>,List<java.math.BigDecimal?>,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions? = null): CallResult<List<Any?>,List<java.math.BigDecimal?>,Results.AtomicDecideValue> {
val native = await { namedEngine.decide(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions? = null): CallResult<String?,java.math.BigDecimal?,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value(), native.probability(), native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions? = null): CallResult<String?,java.math.BigDecimal?,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value(), native.probability(), native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions? = null): CallResult<List<String?>,List<java.math.BigDecimal?>,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions? = null): CallResult<List<String?>,List<java.math.BigDecimal?>,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions? = null): CallResult<List<String?>,List<java.math.BigDecimal?>,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions? = null): CallResult<List<String?>,List<java.math.BigDecimal?>,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions? = null): CallResult<List<String?>,List<java.math.BigDecimal?>,Results.AtomicNullableString> {
val native = await { namedEngine.choose(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions? = null): CallResult<List<String>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions? = null): CallResult<List<String>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions? = null): CallResult<List<List<String>>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0.map { item1 -> item1 } }, null, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions? = null): CallResult<List<List<String>>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0.map { item1 -> item1 } }, null, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions? = null): CallResult<List<List<String>>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0.map { item1 -> item1 } }, null, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions? = null): CallResult<List<List<String>>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0.map { item1 -> item1 } }, null, native.results().toList(), native.facts())
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions? = null): CallResult<List<List<String>>,Nothing?,Results.AtomicArrayOfString> {
val native = await { namedEngine.tag(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0.map { item1 -> item1 } }, null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions? = null): CallResult<java.math.BigDecimal,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value(), null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions? = null): CallResult<java.math.BigDecimal,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value(), null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions? = null): CallResult<List<java.math.BigDecimal>,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions? = null): CallResult<List<java.math.BigDecimal>,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions? = null): CallResult<List<java.math.BigDecimal>,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions? = null): CallResult<List<java.math.BigDecimal>,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions? = null): CallResult<List<java.math.BigDecimal>,Nothing?,Results.AtomicDouble> {
val native = await { namedEngine.score(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<List<Any?>,List<java.math.BigDecimal?>,Results.AtomicBoolean> {
val native = await { namedEngine.filter(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<List<java.math.BigInteger>,List<java.math.BigDecimal?>,Results.AtomicNonZeroUsize> {
val native = await { namedEngine.rank(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<Any?,java.math.BigDecimal?,Results.Find> {
val native = await { namedEngine.find(question, input, options) }
return CallResult(native, native.value(), native.probability(), native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions? = null): CallResult<Results.AnnotatedRow,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value(), null, native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions? = null): CallResult<Results.AnnotatedRow,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value(), null, native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions? = null): CallResult<List<Results.AnnotatedRow>,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions? = null): CallResult<List<Results.AnnotatedRow>,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions? = null): CallResult<List<Results.AnnotatedRow>,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions? = null): CallResult<List<Results.AnnotatedRow>,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions? = null): CallResult<List<Results.AnnotatedRow>,Nothing?,Results.Annotation> {
val native = await { namedEngine.annotate(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, null, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputText, options: Inputs.RequestOptions? = null): CallResult<Results.Recognize,List<java.math.BigDecimal?>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value(), native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputJson, options: Inputs.RequestOptions? = null): CallResult<Results.Recognize,List<java.math.BigDecimal?>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value(), native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputRecords, options: Inputs.RequestOptions? = null): CallResult<List<Results.Recognize>,List<List<java.math.BigDecimal?>>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0.map { item1 -> item1 } }, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputUnits, options: Inputs.RequestOptions? = null): CallResult<List<Results.Recognize>,List<List<java.math.BigDecimal?>>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0.map { item1 -> item1 } }, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputEntities, options: Inputs.RequestOptions? = null): CallResult<List<Results.Recognize>,List<List<java.math.BigDecimal?>>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0.map { item1 -> item1 } }, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputSource, options: Inputs.RequestOptions? = null): CallResult<List<Results.Recognize>,List<List<java.math.BigDecimal?>>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0.map { item1 -> item1 } }, native.results().toList(), native.facts())
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInputFeed, options: Inputs.RequestOptions? = null): CallResult<List<Results.Recognize>,List<List<java.math.BigDecimal?>>,Results.Recognition> {
val native = await { namedEngine.recognize(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0.map { item1 -> item1 } }, native.results().toList(), native.facts())
}
suspend fun relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<List<Results.RelatedEntityEdge>,List<java.math.BigDecimal?>,Results.Relation> {
val native = await { namedEngine.relate(question, input, options) }
return CallResult(native, native.value().map { item0 -> item0 }, native.probability().map { item0 -> item0 }, native.results().toList(), native.facts())
}
suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.AtomicDecideValue> = when (input) {
is Inputs.RequestInputText -> decide(question, input, options)
is Inputs.RequestInputJson -> decide(question, input, options)
is Inputs.RequestInputRecords -> decide(question, input, options)
is Inputs.RequestInputUnits -> decide(question, input, options)
is Inputs.RequestInputEntities -> decide(question, input, options)
is Inputs.RequestInputSource -> decide(question, input, options)
is Inputs.RequestInputFeed -> decide(question, input, options)
}
suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.AtomicNullableString> = when (input) {
is Inputs.RequestInputText -> choose(question, input, options)
is Inputs.RequestInputJson -> choose(question, input, options)
is Inputs.RequestInputRecords -> choose(question, input, options)
is Inputs.RequestInputUnits -> choose(question, input, options)
is Inputs.RequestInputEntities -> choose(question, input, options)
is Inputs.RequestInputSource -> choose(question, input, options)
is Inputs.RequestInputFeed -> choose(question, input, options)
}
suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.AtomicArrayOfString> = when (input) {
is Inputs.RequestInputText -> tag(question, input, options)
is Inputs.RequestInputJson -> tag(question, input, options)
is Inputs.RequestInputRecords -> tag(question, input, options)
is Inputs.RequestInputUnits -> tag(question, input, options)
is Inputs.RequestInputEntities -> tag(question, input, options)
is Inputs.RequestInputSource -> tag(question, input, options)
is Inputs.RequestInputFeed -> tag(question, input, options)
}
suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.AtomicDouble> = when (input) {
is Inputs.RequestInputText -> score(question, input, options)
is Inputs.RequestInputJson -> score(question, input, options)
is Inputs.RequestInputRecords -> score(question, input, options)
is Inputs.RequestInputUnits -> score(question, input, options)
is Inputs.RequestInputEntities -> score(question, input, options)
is Inputs.RequestInputSource -> score(question, input, options)
is Inputs.RequestInputFeed -> score(question, input, options)
}
suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.Annotation> = when (input) {
is Inputs.RequestInputText -> annotate(question, input, options)
is Inputs.RequestInputJson -> annotate(question, input, options)
is Inputs.RequestInputRecords -> annotate(question, input, options)
is Inputs.RequestInputUnits -> annotate(question, input, options)
is Inputs.RequestInputEntities -> annotate(question, input, options)
is Inputs.RequestInputSource -> annotate(question, input, options)
is Inputs.RequestInputFeed -> annotate(question, input, options)
}
suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): CallResult<*,*,Results.Recognition> = when (input) {
is Inputs.RequestInputText -> recognize(question, input, options)
is Inputs.RequestInputJson -> recognize(question, input, options)
is Inputs.RequestInputRecords -> recognize(question, input, options)
is Inputs.RequestInputUnits -> recognize(question, input, options)
is Inputs.RequestInputEntities -> recognize(question, input, options)
is Inputs.RequestInputSource -> recognize(question, input, options)
is Inputs.RequestInputFeed -> recognize(question, input, options)
}
}
open class OwnedCall internal constructor(private val native: Engine.OwnedCall) {
val packets: List<Results.SessionPacket> get() = native.packets().toList()
val terminal: Results.SessionPacketTerminal get() = native.terminal()
fun packets() = packets
fun terminal() = terminal
}
class CallResult<V,P,R> internal constructor(native: Engine.OwnedCall, val value: V, val probability: P, val results: List<R>, val facts: Results.Facts) : OwnedCall(native)

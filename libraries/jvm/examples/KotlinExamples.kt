import examples.SampleInputs as S
import thinkthen.kotlin.KotlinEngine
/** Coroutine calls return native Kotlin answers. */
suspend fun runKotlinExamples(engine: KotlinEngine): List<Any?> {
    val answers = listOf(
    engine.decide(question = S.decide(), input = S.input("session-decide"), options = S.options()).value,
    engine.choose(S.choose(), S.input("session-choose"), S.options()).value,
    engine.tag(S.tag(), S.input("session-tag"), S.options()).value,
    engine.score(S.score(), S.input("session-score"), S.options()).value,
    engine.filter(S.decide(), S.input("session-filter"), S.options()).value,
    engine.rank(S.decide(), S.input("session-rank"), S.options()).value,
    engine.find(S.find(), S.units(), S.options()).value,
    engine.annotate(S.annotate(), S.input("session-annotate"), S.options()).value,
    engine.recognize(S.recognize(), S.input("Maria Chen"), S.options()).value,
    engine.relate(S.relate(), S.entities(), S.options()).value
)
    val edge = (answers.last() as List<*>).first() as thinkthen.Results.RelatedEntityEdge
    println(edge.sourceName() + " -> " + edge.targetName())
    return answers
}

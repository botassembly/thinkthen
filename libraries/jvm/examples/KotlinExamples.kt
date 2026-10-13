import examples.SampleInputs as S
import thinkthen.kotlin.KotlinEngine
/** Coroutine calls return the shared owned Java result. */
suspend fun runKotlinExamples(engine: KotlinEngine): List<Any?> {
    val answers = listOf(
    engine.decide(S.decide(), S.input("session-decide"), S.options()).decide(),
    engine.choose(S.choose(), S.input("session-choose"), S.options()).choose(),
    engine.tag(S.tag(), S.input("session-tag"), S.options()).tag(),
    engine.score(S.score(), S.input("session-score"), S.options()).score(),
    engine.filter(S.decide(), S.input("session-filter"), S.options()).filter(),
    engine.rank(S.decide(), S.input("session-rank"), S.options()).rank(),
    engine.find(S.find(), S.units(), S.options()).find(),
    engine.annotate(S.annotate(), S.input("session-annotate"), S.options()).annotate(),
    engine.recognize(S.recognize(), S.input("Maria Chen"), S.options()).recognize(),
    engine.relate(S.relate(), S.entities(), S.options()).relate()
)
    val edge = (answers.last() as thinkthen.Results.Relation).value().first()
    println(edge.sourceName() + " -> " + edge.targetName())
    return answers
}

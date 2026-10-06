import thinkthen.Door

fun main(args: Array<String>) {
    if (args.size == 1 && args[0] == "carriers") { kotlinCarrierChecks(); return }
    try {
        Door().use { door -> println(KotlinFacade(door).call(args[0])) }
    } catch (failure: Door.NativeFailure) {
        println("{\"failed\":{\"kind\":\"${failure.failure.kind().name.lowercase()}\",\"code\":${failure.failure.code()}}}")
    }
}


// Fixture decoding/static APIs; no native result or parity receipt.
private fun kotlinCarrierChecks() {
    val question = thinkthen.Requests.QuestionInput.questionFile("β.json")
    val source = thinkthen.Requests.InputSource.files(thinkthen.Complete.FileSource(listOf("β.png", "β.png"), thinkthen.Complete.SourceUnit.IMAGE_FILE, 0))
    val controls = thinkthen.Complete.CallControls(thinkthen.Complete.OptionalValue.absent(), thinkthen.Complete.OptionalValue.absent(), false, true)
    val requests = listOf(
        KotlinRequests.decide(question, source, controls),
        KotlinRequests.choose(question, source, controls),
        KotlinRequests.tag(question, source, controls),
        KotlinRequests.score(question, source, controls),
        KotlinRequests.filter(question, source, controls),
        KotlinRequests.rank(question, source, controls),
        KotlinRequests.find(question, source, controls),
        KotlinRequests.annotate(question, source, controls),
        KotlinRequests.recognize(question, source, controls),
        KotlinRequests.relate(question, source, controls)
    )
    check(requests.map { it.function() } == thinkthen.Complete.Function.entries.toList())
    check(source.files().value().paths().size == 2 && !question.question().present())
    val answer = thinkthen.CompleteReaders.atomic("""{"kind":"choice","pick":"β","probabilities":{"β":0.7,"a":0.3},"confidence":0}""")
    check(answer.probabilities()[0].name() == "β" && answer.confidence().present() && answer.confidence().value() == 0.0)
    val span = thinkthen.Complete.NameSpan(1, 3, thinkthen.Complete.OptionalValue.of(emptyList()), thinkthen.Complete.OptionalValue.absent())
    check(span.kinds().present() && !span.edges().present() && span.end() == 3L)
    println("{\"carriers\":\"pass\"}")
}

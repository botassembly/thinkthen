import dev.thinkthen.kotlin.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import kotlinx.serialization.json.*

enum class Topic(private val word: String) {
    BILLING("billing"), SHIPPING("shipping"), OTHER("other");
    override fun toString(): String = word
}
enum class Urgency(private val word: String) {
    ROUTINE("routine"), URGENT("urgent");
    override fun toString(): String = word
}

private data class MutableWord(var word: String) { override fun toString(): String = word }

private suspend fun expectFailure(code: ErrorCode, original: Any?, httpStatus: Int? = null, call: suspend () -> Any?): FailedException {
    val failure = try { call(); error("Scalar failure must throw at the call") }
        catch (failure: FailedException) { failure }
    check(failure.code == code && failure.httpStatus == httpStatus)
    val row = failure.rows.single()
    check(row.item === original && row.position == 1L && row.error.code == code)
    return failure
}

suspend fun main(args: Array<String>) {
    val scenario = args.single()
    try { ChooseQuestion("Which?", listOf(MutableWord("first"), MutableWord("second"))); error("Mutable choice domain was accepted") }
    catch (failure: UsageException) { check(failure.message.orEmpty().contains("strings or enum")) }
    try { TagQuestion("Which?", listOf(MutableWord("first"), MutableWord("second"))); error("Mutable tag domain was accepted") }
    catch (failure: UsageException) { check(failure.message.orEmpty().contains("strings or enum")) }
    ThinkThen { retry = RetryPolicy(maxRetries = 0) }.use { client ->
        if (scenario == "failed") {
            val original = linkedMapOf("id" to "failed-original", "text" to "body")
            val calls = listOf<suspend () -> Any?>(
                { client.decide("Ready?", original) },
                { client.choose("Topic?", original, Topic::class) },
                { client.tag("Topics?", original, Topic::class) },
                { client.score("Urgency?", original, Urgency::class) },
                { client.find("Which line?", original) },
                { client.recognize(original, listOf("person")) },
                { client.relate(original, listOf(RelationRule("person", "organization", "works_for"))) },
            )
            for (call in calls) {
                val failure = expectFailure(ErrorCode.BACKEND_STATUS, original, 503, call)
                check(failure.rows.single().id.toString().contains("failed-original"))
            }
            val questions = AnnotationQuestions(mapOf("refund" to DecideQuestion("Refund?")))
            val form = client.annotate(questions, original)
            check(form.isAnswered && form.answer().entries.values.all { it.isFailed })
            val asyncForm = coroutineScope { async { client.annotate(questions, original) }.await() }
            check(asyncForm.isAnswered && asyncForm.answer().entries.values.all { it.isFailed })
            val before = client.usage.attempts
            expectFailure(ErrorCode.RECORD_EMPTY, "") { client.annotate(questions, "") }
            expectFailure(ErrorCode.RECORD_EMPTY, "") { client.annotate(AnnotateQuestion(questions), "") }
            check(client.usage.attempts == before) { "Blank annotate must fail before transport" }
            val siblingStarted = CompletableDeferred<Unit>()
            val siblingStopped = CompletableDeferred<Unit>()
            expectFailure(ErrorCode.BACKEND_STATUS, original, 503) {
                coroutineScope {
                    launch {
                        try { siblingStarted.complete(Unit); awaitCancellation() }
                        finally { siblingStopped.complete(Unit) }
                    }
                    async { siblingStarted.await(); client.decide("Ready?", original) }.await()
                }
            }
            check(siblingStopped.isCompleted) { "Scalar failure must cancel and release structured siblings" }
        }
        val item = linkedMapOf("id" to "a1", "text" to "Please refund my order.")
        val question = DecideQuestion("Does the customer request a refund?", threshold = Threshold.band(0.2, 0.8), field = Pointer("/text"))
        val row = client.decideStream(question, listOf(item)).single()
        check(row.item === item && row.id == client.decideStream(question, listOf(item.toMap())).single().id)
        val equal = client.decideStream(question, listOf(item.toMap())).single()
        check(row == equal && row.hashCode() == equal.hashCode())
        val hash = row.hashCode()
        item["text"] = "caller changed this after the request"
        check(row == equal && row.hashCode() == hash && setOf(row, equal).size == 1)
        check(!row.equals(true) && !row.equals("true") && !row.equals(Topic.BILLING))
        check(!row.toJson().contains("caller changed"))
        if (row is Row.Unsure) {
            check(row.toString() == "unsure")
            try { row.answer(); error("Not sure answered") } catch (failure: UnsureException) { check(failure.row === row && failure.probability == row.probability) }
            check(row.throwIfFailed() === row)
        }
        if (row is Row.Failed) {
            check(row.toString().startsWith("failed: ${row.error.code.wireName}:"))
            try { row.answer(); error("Failure answered") } catch (failure: FailedException) { check(failure.rows.single() === row) }
        }
        if (!row.isFailed) {
            val scalar = client.decide(question, item.toMap())
            check(!scalar.isFailed && scalar.status == row.status)
        }
        val before = client.usage.attempts
        for (empty in listOf("", "  ")) {
            expectFailure(ErrorCode.RECORD_EMPTY, empty) { client.decide("Ready?", empty) }
        }
        val emptyMap = linkedMapOf<String, String>()
        expectFailure(ErrorCode.RECORD_EMPTY, emptyMap) { client.decide("Ready?", emptyMap) }
        expectFailure(ErrorCode.RECORD_MISSING, null) { client.decide("Ready?", null) }
        check(client.usage.attempts == before) { "Blank and null scalar inputs must fail before transport" }
        val missing = client.decideStream("Ready?", listOf<String?>(null)).single()
        check(missing.isFailed && missing.item == null && missing.probability == null)
        val questions = AnnotationQuestions(mapOf(
            "refund" to DecideQuestion("Does the customer request a refund?", threshold = Threshold.band(0.2, 0.8)),
            "topic" to ChooseQuestion("What is the message about?", Topic::class),
            "urgency" to ScoreQuestion("How urgent is the request?", Urgency::class),
        ))
        val formItem = linkedMapOf("id" to "form", "text" to "Please refund my order.")
        val form = client.annotate(questions, formItem)
        if (form.isAnswered) {
            val entries = form.answer()
            entries.entries.values.forEach { child -> check(child.item === formItem && child.position == form.position && child.id == form.id) }
            val failed = entries.entries.toSortedMap().values.filterIsInstance<Row.Failed<*>>()
            if (failed.isEmpty()) check(form.throwIfFailed() === form)
            else try { form.throwIfFailed(); error("Form hid failed child") } catch (failure: FailedException) {
                check(failure.rows.size == 1 && failure.rows.single() === failed.first())
            }
            val equalForm = client.annotate(questions, formItem.toMap())
            val formHash = form.hashCode()
            check(form == equalForm && formHash == equalForm.hashCode())
            formItem["text"] = "changed form item"
            check(form == equalForm && form.hashCode() == formHash)
            entries.choose("topic", Topic::class).let { if (it.isAnswered) check(it.answer() in Topic.entries) }
            try { entries.choose("refund"); error("Wrong entry selected") } catch (_: UsageException) { }
        }
        if (scenario == "yes" || scenario == "relations") {
            val word = "bill\tline\r\nlast"
            val printed = client.choose("Which?", "text", listOf(word, "other"))
            check(printed.answer() == word && printed.toString() == "bill line  last")
            check(Json.parseToJsonElement(printed.toJson()).jsonObject.getValue("answer").jsonPrimitive.content == word)
            val toolbox = Toolbox(mapOf("topic" to ChooseQuestion("What is the message about?", Topic::class), "tags" to TagQuestion("Which topics appear?", Topic::class)))
            check(client.choose(toolbox.choose("topic"), "text").answer() == "billing")
            check(client.choose(toolbox.choose("topic", Topic::class), "text").answer() == Topic.BILLING)
            check(client.tag(toolbox.tag("tags"), "text").answer() == listOf("billing", "shipping", "other"))
            check(client.tag(toolbox.tag("tags", Topic::class), "text").answer().all { it in Topic.entries })
            check(client.tag("Which topics appear?", "text", Topic::class).toString() == "billing\tshipping\tother")
            check(client.score("How urgent is the request?", "text", Urgency::class).toString() == "1")
            fun immutableTags(result: Row<List<Topic>, *>) {
                val hash = result.hashCode()
                val json = result.toJson()
                val values = result.answer()
                try { (values as MutableList<Topic>).clear(); error("Enum tags were mutable") }
                catch (_: UnsupportedOperationException) { }
                check(result.hashCode() == hash && result.toJson() == json && setOf(result).contains(result))
            }
            immutableTags(client.tag("Which topics appear?", "text", Topic::class))
            val loaded = Toolbox.fromJson(toolbox.toJson())
            immutableTags(client.tag(loaded.tag("tags", Topic::class), "text"))
            val tagsForm = client.annotate(AnnotationQuestions(mapOf("tags" to TagQuestion("Which topics appear?", Topic::class))), "text")
            immutableTags(tagsForm.answer().tag("tags", Topic::class))
            val ranked = client.rank("How urgent is the request?", listOf(item), Urgency::class, top = 1).toList().single()
            check(ranked.item === item && ranked.answer() == 1L && ranked.toString() == "1")
        }
        if (scenario == "refund-failed") {
            val upperBmp = "\uE000"
            val astral = "\uD800\uDC00"
            val order = listOf("10", "2", upperBmp, astral)
            val record = mapOf("id" to "order", "text" to "Please refund my order.")
            val ordered = client.annotate(AnnotationQuestions(mapOf(
                "2" to ChooseQuestion("Which?", listOf("billing", "shipping")),
                astral to DecideQuestion("Ready?"),
                "10" to ChooseQuestion("Which?", listOf("billing", "shipping")),
                upperBmp to DecideQuestion("Ready?"),
            )), record)
            val entries = ordered.answer()
            check(entries.entries.keys.toList() == order) { "Admitted engine question order changed" }
            val first = entries.decide(upperBmp)
            check(first is Row.Failed && entries.decide(astral).isFailed)
            try { ordered.throwIfFailed(); error("Form ignored failed children") }
            catch (failure: FailedException) {
                check(failure.rows.size == 1 && failure.rows.single() === first) { "First failure must follow engine UTF-8 name order" }
                check(failure.rows.single().item === record && first.position == ordered.position && first.id == ordered.id && first.source == ordered.source)
            }
            check(ordered.toString().split('\t').map { it.substringBefore('=') } == order) { "Printed form order differs from the engine" }
        }
        if (scenario == "relations") {
            val original = linkedMapOf("text" to "é🦀 Alex Morgan works for Example Group.")
            val names = client.recognize(original, listOf("person", "organization"), field = Pointer("/text"))
            val name = names.answer().first()
            check(original.getValue("text").substring(name.start, name.end) == name.text)
            check(name.text == "Alex Morgan" && name.start == 4)
            original["text"] = "changed source"
            val saved = Json.parseToJsonElement(names.toJson()).jsonObject.getValue("answer").jsonArray.first().jsonObject
            check(saved.getValue("start").jsonPrimitive.int == 3 && names.toString().contains("Alex Morgan (3..14)"))
            check(client.find("Which line states the requested action?", "Please refund my order.").toString() == "Please refund my order.")
            check(client.relate("Morgan works for Example Group.", listOf(RelationRule("person", "organization", "works_for"))).toString() == "Morgan --works_for--> Example Group")
        }
    }
    println("Kotlin ideal host contracts passed: $scenario")
}

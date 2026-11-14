package consumer

import dev.thinkthen.kotlin.*
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable

@Serializable private data class LargeRecord(val id: Int, var body: String)

// Rank retains every original while evaluating top, and a detached result keeps its own original alive.
fun main() = runBlocking {
    for (stream in listOf(false, true)) {
        var consumed = 0
        val originals = mutableListOf<LargeRecord>()
        val records = generateSequence { LargeRecord(++consumed, "x".repeat(1024)).also(originals::add) }.take(64)
        val ranked = ThinkThen { backend = Backend.STUB }.use { client ->
            val levels = listOf("low", "high")
            val ranks = if (stream) client.rank("Priority?", records.asFlow(), levels, top = 1) else client.rank("Priority?", records.asIterable(), levels, top = 1)
            ranks.toList().single()
        }
        check(consumed == 64 && ranked.answer() == 1L && ranked.item === originals[(ranked.position - 1).toInt()])
        val hash = ranked.hashCode()
        ranked.item.body = "changed after close"
        check(ranked.hashCode() == hash && ranked.item.body == "changed after close")
    }
}

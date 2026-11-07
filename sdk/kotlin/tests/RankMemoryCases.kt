package consumer

import dev.thinkthen.kotlin.*
import kotlinx.coroutines.flow.asFlow
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable

@Serializable private data class LargeRecord(val id: Int, val body: String)

// Run with a 64 MiB heap: the source below is 128 MiB, so holding each record until its row arrives runs out of memory.
fun main() = runBlocking {
    for (stream in listOf(false, true)) {
        var consumed = 0
        val records = generateSequence { LargeRecord(++consumed, "x".repeat(65536)) }.take(2048)
        ThinkThen { backend = Backend.STUB }.use { client ->
            val levels = listOf("low", "high")
            val ranks = if (stream) client.rank("Priority?", records.asFlow(), levels, top = 1) else client.rank("Priority?", records.asIterable(), levels, top = 1)
            val ranked = ranks.toList().single()
            check(consumed == 2048 && ranked.answer() == 1L && ranked.item.id.toLong() == ranked.position && ranked.item.body.length == 65536)
        }
    }
}

package consumer

import io.thinkthen.kotlin.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

// Feature: a finite or streaming collection starts one driver across 1,000 rows.
suspend fun streamThreads() {
    ThinkThen { backend = Backend.Stub; batch = 1 }.use { client ->
        val records = List(1000) { "alpha" }
        StreamThreadAudit().use {
            check(client.decideStream("Ready?", records).count() == 1000)
        }
        StreamThreadAudit().use {
            check(client.decideStream("Ready?", records.asFlow()).count() == 1000)
        }
        // Hold each first row until all streams are active, then complete all calls.
        coroutineScope {
            val release = CompletableDeferred<Unit>()
            var arrived = 0
            val streams = List(100) {
                async {
                    client.decideStream("Ready?", flowOf("alpha", "beta")).onEach {
                        if (++arrived == 100) release.complete(Unit)
                        release.await()
                    }.count()
                }
            }
            check(streams.awaitAll().all { it == 2 })
        }
    }
}

fun main() = runBlocking { streamThreads() }

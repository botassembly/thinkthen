package consumer

import java.lang.management.ManagementFactory
import java.nio.file.Path
import java.util.ArrayDeque
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit

// Diagnostic silence threshold only: a dump never stops or fails a consumer case.
internal object FlowCapture {
    private const val STALL_SECONDS = 30L
    private const val EVENT_LIMIT = 64
    private val events = ArrayDeque<String>()
    private var lastEvent = System.nanoTime()
    private var sequence = 0L
    private var dumped = -1L

    @Synchronized fun event(name: String) {
        if (events.size == EVENT_LIMIT) events.removeFirst()
        events.addLast("${++sequence}: $name")
        lastEvent = System.nanoTime()
    }

    suspend fun case(name: String, body: suspend () -> Unit) {
        println("Flow case started: $name")
        event("case $name started")
        body()
        event("case $name finished")
        println("Flow case finished: $name")
    }

    fun run(args: Array<String>, body: () -> Unit) {
        if (!java.lang.Boolean.getBoolean("thinkthen.flow.capture")) {
            // The pinned coroutine core already includes its probe agent. Keep it
            // in the test process, without adding a runtime library dependency.
            val core = Path.of(Class.forName("kotlinx.coroutines.Job").protectionDomain.codeSource.location.toURI())
            val java = Path.of(System.getProperty("java.home"), "bin", "java").toString()
            val command = listOf(java) + ManagementFactory.getRuntimeMXBean().inputArguments +
                listOf("-javaagent:$core", "-Dthinkthen.flow.capture=true", "-cp", System.getProperty("java.class.path"), "consumer.FlowCasesKt") + args
            kotlin.system.exitProcess(ProcessBuilder(command).inheritIO().start().waitFor())
        }
        val probes = Class.forName("kotlinx.coroutines.debug.internal.DebugProbesImpl")
        val instance = probes.getField("INSTANCE").get(null)
        check(probes.getMethod("isInstalled\$kotlinx_coroutines_debug").invoke(instance) == true) { "Coroutine probes are not installed" }
        val monitor = Executors.newSingleThreadScheduledExecutor { task ->
            Thread(task, "flow-stall-capture").apply { isDaemon = true }
        }
        monitor.scheduleAtFixedRate({
            val snapshot = synchronized(this) {
                if (System.nanoTime() - lastEvent >= TimeUnit.SECONDS.toNanos(STALL_SECONDS) && dumped != sequence) {
                    dumped = sequence
                    events.toList()
                } else null
            }
            if (snapshot != null) {
                System.err.println("Flow consumer has no new event; diagnostic capture follows (execution continues)")
                snapshot.forEach(System.err::println)
                probes.getMethod("dumpCoroutines", java.io.PrintStream::class.java).invoke(instance, System.err)
                System.err.flush()
            }
        }, STALL_SECONDS, STALL_SECONDS, TimeUnit.SECONDS)
        try { body() } finally { monitor.shutdownNow() }
    }
}

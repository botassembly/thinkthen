import thinkthen.Inputs
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.UsagePersistence
import java.nio.file.*
import kotlinx.coroutines.runBlocking

fun main(args: Array<String>) = runBlocking {
    val engine = KotlinEngine(Inputs.EngineSettings().cache(Inputs.CacheDocument(false)).maxRetries(0))
    if (args[0] == "disabled") {
        check(engine.usagePersistence().state == UsagePersistence.State.Disabled)
        check(engine.finishUsageStatus().state == UsagePersistence.State.Disabled)
        engine.close(); return@runBlocking
    }
    check(engine.usagePersistence().state == UsagePersistence.State.Written)
    val call = engine.decide(Inputs.RequestQuestionText().text("Is it?"), Inputs.RequestInputText().text("usage-kotlin-" + args[0]))
    check(call.terminal.facts!!.requestsSent.intValueExact() == 1)
    check(engine.usagePersistence().state == UsagePersistence.State.Pending)
    Files.writeString(Path.of(args[1], "pending"), "")
    if (args[0] == "written") while (!Files.exists(Path.of(args[1], "released"))) Thread.sleep(5)
    val status = engine.finishUsageStatus()
    check(status.state == if (args[0] == "written") UsagePersistence.State.Written else UsagePersistence.State.Failed)
    check(engine.usagePersistence() == status)
    engine.close()
    check(call.packets.filterIsInstance<thinkthen.kotlin.Results.SessionPacketDecideRow>().any { it.value.value.value == true })
    check(call.terminal.facts!!.requestsSent.intValueExact() == 1)
    check(if (args[0] == "written") status.advice == null else status.advice == "check the usage folder permissions and free space")
    check(runCatching { engine.usagePersistence() }.exceptionOrNull()?.message == "Engine is closed")
    check(runCatching { engine.finishUsageStatus() }.exceptionOrNull()?.message == "Engine is closed")
    println("KOTLIN_USAGE_PASS")
}

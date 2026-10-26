import consumer.stubCases
import kotlinx.coroutines.runBlocking
import java.nio.file.Path
fun main(args: Array<String>) = runBlocking { stubCases(Path.of(args.first())) }

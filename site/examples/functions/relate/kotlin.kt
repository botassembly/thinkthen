import thinkthen.Inputs
import thinkthen.Inputs.RequestInputRecords
import thinkthen.Inputs.RequestItem
import thinkthen.Inputs.RequestOriginalJson
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results.SessionPacketRelateAggregate

suspend fun main() = KotlinEngine().use { tt ->
    val sings = mapOf("name" to "sings",
        "source" to "singer", "target" to "song",
        "either" to false)
    val ask = RequestQuestionDefinition().value(
        Inputs.RequestDefinitionFieldsRelateVersion
            .fromJson(mapOf("version" to 1, "relate" to
                mapOf("relations" to listOf(sings)))))
    val paul = mapOf(
        "name" to "Paul McCartney", "kind" to "singer")
    val ringo = mapOf(
        "name" to "Ringo Starr", "kind" to "singer")
    val yesterday = mapOf(
        "name" to "Yesterday", "kind" to "song")
    val garden = mapOf(
        "name" to "Octopus's Garden", "kind" to "song")
    val names = RequestInputRecords().items(
        listOf(paul, ringo, yesterday, garden).map { one ->
            RequestItem().original(
                RequestOriginalJson().value(one))
        })
    val whoSings = tt.relate(ask, names).packets
        .filterIsInstance<SessionPacketRelateAggregate>()
        .first().value.value
    val pairs = whoSings.map { edge ->
        edge.source.json to edge.target.json
    }
    check(pairs == listOf(
        paul to yesterday,
        ringo to garden,
    ))
}

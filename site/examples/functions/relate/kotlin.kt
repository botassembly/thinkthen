import thinkthen.Inputs
import thinkthen.Inputs.RequestInputRecords
import thinkthen.Inputs.RequestItem
import thinkthen.Inputs.RequestOriginalJson
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results.SessionPacketRelateAggregate

suspend fun main() = KotlinEngine().use { tt ->
    val makes = mapOf("name" to "makes",
        "source" to "company", "target" to "product",
        "either" to false)
    val ask = RequestQuestionDefinition().value(
        Inputs.RequestDefinitionFieldsRelateVersion
            .fromJson(mapOf("version" to 1, "relate" to
                mapOf("relations" to listOf(makes)))))
    val apple = mapOf(
        "name" to "Apple", "kind" to "company")
    val amazon = mapOf(
        "name" to "Amazon", "kind" to "company")
    val iphone = mapOf(
        "name" to "iPhone", "kind" to "product")
    val kindle = mapOf(
        "name" to "Kindle", "kind" to "product")
    val names = RequestInputRecords().items(
        listOf(apple, amazon, iphone, kindle).map { one ->
            RequestItem().original(
                RequestOriginalJson().value(one))
        })
    val whoMakes = tt.relate(ask, names).packets
        .filterIsInstance<SessionPacketRelateAggregate>()
        .first().value.value
    val pairs = whoMakes.map { edge ->
        edge.source.json to edge.target.json
    }
    check(pairs == listOf(
        apple to iphone,
        amazon to kindle,
    ))
}

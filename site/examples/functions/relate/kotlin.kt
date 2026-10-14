import thinkthen.Inputs
import thinkthen.Inputs.RequestInputRecords
import thinkthen.Inputs.RequestItem
import thinkthen.Inputs.RequestOriginalJson
import thinkthen.Inputs.RequestQuestionDefinition
import thinkthen.kotlin.KotlinEngine

suspend fun main() = KotlinEngine().use { tt ->
    val makes = Inputs.AuthoredRelation()
        .name("makes").source("company")
        .target("product").either(false)
    val ask = RequestQuestionDefinition().value(
        Inputs.RequestDefinitionAuthoredRelate()
            .relate(Inputs.RelateQuestion()
                .relations(listOf(makes))))
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
    val whoMakes = tt.relate(ask, names).relate().value()
    val pairs = whoMakes.map { edge ->
        edge.sourceName() to edge.targetName()
    }
    check(pairs == listOf(
        "Apple" to "iPhone",
        "Amazon" to "Kindle",
    ))
}

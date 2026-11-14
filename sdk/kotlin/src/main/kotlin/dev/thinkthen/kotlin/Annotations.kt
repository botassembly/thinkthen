package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.Domain
import dev.thinkthen.kotlin.internal.mapAnswer

import dev.thinkthen.kotlin.internal.immutableList
import dev.thinkthen.kotlin.internal.immutableMap
import dev.thinkthen.kotlin.internal.missing
import dev.thinkthen.kotlin.internal.questionNameOrder
import kotlin.reflect.KClass

/** Named ordinary result rows, retaining the form's original record and metadata. */
public class Annotations internal constructor(entries: Map<String, Row<*, *>>) {
    public val entries: Map<String, Row<*, *>> = immutableMap(entries)
    @Suppress("UNCHECKED_CAST")
    private fun <A> select(name: String, function: FunctionName): Row<A, Any?> {
        val row = entries[name]
        if (row == null || row.function != function) missing(function.name.lowercase(), "annotation", name, entries.keys)
        return row as Row<A, Any?>
    }
    public fun decide(name: String): Row<Boolean, Any?> = select(name, FunctionName.DECIDE)
    public fun choose(name: String): Row<String, Any?> = select(name, FunctionName.CHOOSE)
    public fun <E : Enum<E>> choose(name: String, type: KClass<E>): Row<E, Any?> {
        val domain = Domain.enums(type)
        return select<String>(name, FunctionName.CHOOSE).mapAnswer(domain::read) { kotlinx.serialization.json.JsonPrimitive(domain.word(it)) }
    }
    public fun tag(name: String): Row<List<String>, Any?> = select(name, FunctionName.TAG)
    public fun <E : Enum<E>> tag(name: String, type: KClass<E>): Row<List<E>, Any?> {
        val domain = Domain.enums(type)
        return select<List<String>>(name, FunctionName.TAG).mapAnswer({ values -> immutableList(values.map(domain::read)) }) { values -> kotlinx.serialization.json.JsonArray(values.map { kotlinx.serialization.json.JsonPrimitive(domain.word(it)) }) }
    }
    public fun score(name: String): Row<Double, Any?> = select(name, FunctionName.SCORE)
    public fun find(name: String): Row<Span?, Any?> = select(name, FunctionName.FIND)
    public fun recognize(name: String): Row<List<Entity>, Any?> = select(name, FunctionName.RECOGNIZE)
    public fun relate(name: String): Row<List<Relation>, Any?> = select(name, FunctionName.RELATE)
    override fun equals(other: Any?): Boolean = other is Annotations && entries == other.entries
    override fun hashCode(): Int = entries.hashCode()
    override fun toString(): String = entries.toSortedMap(questionNameOrder).entries.joinToString("\n") { (name, row) -> "$name\t$row" }
}

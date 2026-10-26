package io.thinkthen.kotlin

import kotlinx.serialization.Serializable
import java.util.Collections

internal fun <T> immutableList(values: List<T>): List<T> = Collections.unmodifiableList(ArrayList(values))
internal fun <K,V> immutableMap(values: Map<K,V>): Map<K,V> = Collections.unmodifiableMap(LinkedHashMap(values))
@Serializable class LabelDetail private constructor(val labels: Map<String,Double>) {
    companion object { internal fun of(values: Map<String,Double>): LabelDetail = LabelDetail(immutableMap(values)) }
    override fun equals(other: Any?): Boolean = other is LabelDetail && labels == other.labels
    override fun hashCode(): Int = labels.hashCode()
}
@Serializable class LevelDetail private constructor(val levels: List<Double>) {
    companion object { internal fun of(values: List<Double>): LevelDetail = LevelDetail(immutableList(values)) }
    override fun equals(other: Any?): Boolean = other is LevelDetail && levels == other.levels
    override fun hashCode(): Int = levels.hashCode()
}
@Serializable class RelationDetail private constructor(val names: List<Entity>, val pairs: List<Relation>) {
    companion object { internal fun of(names: List<Entity>, pairs: List<Relation>): RelationDetail = RelationDetail(immutableList(names),immutableList(pairs)) }
    override fun equals(other: Any?): Boolean = other is RelationDetail && names == other.names && pairs == other.pairs
    override fun hashCode(): Int = 31 * names.hashCode() + pairs.hashCode()
}
@Serializable(with = PieceDetailSerializer::class) class PieceDetail internal constructor(val span: Span, tags: Map<BoundaryTag,Double>) {
    val tags: Map<BoundaryTag,Double> = immutableMap(tags)
    override fun equals(other: Any?): Boolean = other is PieceDetail && span == other.span && tags == other.tags
    override fun hashCode(): Int = 31 * span.hashCode() + tags.hashCode()
}
@Serializable class KindDetail private constructor(val start: Int, val end: Int, val kinds: Map<String,Double>) {
    companion object { internal fun of(start: Int, end: Int, kinds: Map<String,Double>): KindDetail = KindDetail(start,end,immutableMap(kinds)) }
    override fun equals(other: Any?): Boolean = other is KindDetail && start == other.start && end == other.end && kinds == other.kinds
    override fun hashCode(): Int = 31 * (31 * start + end) + kinds.hashCode()
}
@Serializable class RecognizeDetail private constructor(val pieces: List<PieceDetail>, val names: List<KindDetail>) {
    companion object { internal fun of(pieces: List<PieceDetail>, names: List<KindDetail>): RecognizeDetail = RecognizeDetail(immutableList(pieces),immutableList(names)) }
    override fun equals(other: Any?): Boolean = other is RecognizeDetail && pieces == other.pieces && names == other.names
    override fun hashCode(): Int = 31 * pieces.hashCode() + names.hashCode()
}

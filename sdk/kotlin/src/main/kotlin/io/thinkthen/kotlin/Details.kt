package io.thinkthen.kotlin

import kotlinx.serialization.Serializable
import java.util.Collections

internal fun <T> immutableList(values: List<T>): List<T> = Collections.unmodifiableList(ArrayList(values))
internal fun <K,V> immutableMap(values: Map<K,V>): Map<K,V> = Collections.unmodifiableMap(LinkedHashMap(values))
@Serializable public class LabelDetail private constructor(public val labels: Map<String,Double>) {
    public companion object { internal fun of(values: Map<String,Double>): LabelDetail = LabelDetail(immutableMap(values)) }
    override fun toString(): String = "LabelDetail(labels=$labels)"
    override fun equals(other: Any?): Boolean = other is LabelDetail && labels == other.labels
    override fun hashCode(): Int = labels.hashCode()
}
@Serializable public class LevelDetail private constructor(public val levels: List<Double>) {
    public companion object { internal fun of(values: List<Double>): LevelDetail = LevelDetail(immutableList(values)) }
    override fun toString(): String = "LevelDetail(levels=$levels)"
    override fun equals(other: Any?): Boolean = other is LevelDetail && levels == other.levels
    override fun hashCode(): Int = levels.hashCode()
}
@Serializable public class RelationDetail private constructor(public val names: List<Entity>, public val pairs: List<Relation>) {
    public companion object { internal fun of(names: List<Entity>, pairs: List<Relation>): RelationDetail = RelationDetail(immutableList(names),immutableList(pairs)) }
    override fun toString(): String = "RelationDetail(names=$names, pairs=$pairs)"
    override fun equals(other: Any?): Boolean = other is RelationDetail && names == other.names && pairs == other.pairs
    override fun hashCode(): Int = 31 * names.hashCode() + pairs.hashCode()
}
@Serializable(with = PieceDetailSerializer::class) public class PieceDetail internal constructor(public val span: Span, tags: Map<BoundaryTag,Double>) {
    public val tags: Map<BoundaryTag,Double> = immutableMap(tags)
    override fun toString(): String = "PieceDetail(span=$span, tags=$tags)"
    override fun equals(other: Any?): Boolean = other is PieceDetail && span == other.span && tags == other.tags
    override fun hashCode(): Int = 31 * span.hashCode() + tags.hashCode()
}
@Serializable public class KindDetail private constructor(public val start: Int, public val end: Int, public val kinds: Map<String,Double>) {
    public companion object { internal fun of(start: Int, end: Int, kinds: Map<String,Double>): KindDetail = KindDetail(start,end,immutableMap(kinds)) }
    override fun toString(): String = "KindDetail(start=$start, end=$end, kinds=$kinds)"
    override fun equals(other: Any?): Boolean = other is KindDetail && start == other.start && end == other.end && kinds == other.kinds
    override fun hashCode(): Int = 31 * (31 * start + end) + kinds.hashCode()
}
@Serializable public class RecognizeDetail private constructor(public val pieces: List<PieceDetail>, public val names: List<KindDetail>) {
    public companion object { internal fun of(pieces: List<PieceDetail>, names: List<KindDetail>): RecognizeDetail = RecognizeDetail(immutableList(pieces),immutableList(names)) }
    override fun toString(): String = "RecognizeDetail(pieces=$pieces, names=$names)"
    override fun equals(other: Any?): Boolean = other is RecognizeDetail && pieces == other.pieces && names == other.names
    override fun hashCode(): Int = 31 * pieces.hashCode() + names.hashCode()
}

package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.PieceDetailSerializer
import dev.thinkthen.kotlin.internal.immutableList
import dev.thinkthen.kotlin.internal.immutableMap
import java.util.Objects
import kotlinx.serialization.Serializable

/** Function-specific detail, safe to inspect without reading an answer. */
public sealed interface Detail

/** Choose and tag detail: each label's probability, in the question's label order. */
@Serializable
public class LabelDetail private constructor(
    /** Each label's probability. */
    public val labels: Map<String, Double>,
) : Detail {
    /** Builds label detail inside the library. */
    public companion object {
        internal fun of(values: Map<String, Double>): LabelDetail = LabelDetail(immutableMap(values))
    }
    override fun toString(): String = "LabelDetail(labels=$labels)"
    override fun equals(other: Any?): Boolean = other is LabelDetail && labels == other.labels
    override fun hashCode(): Int = labels.hashCode()
}

/** ScoreQuestion and rank detail: each level's probability, in the question's level order. */
@Serializable
public class LevelDetail private constructor(
    /** Each level's probability. */
    public val levels: List<Double>,
) : Detail {
    /** Builds level detail inside the library. */
    public companion object {
        internal fun of(values: List<Double>): LevelDetail = LevelDetail(immutableList(values))
    }
    override fun toString(): String = "LevelDetail(levels=$levels)"
    override fun equals(other: Any?): Boolean = other is LevelDetail && levels == other.levels
    override fun hashCode(): Int = levels.hashCode()
}

/** RelateQuestion detail: the recognized names and every scored pair. */
@Serializable
public class RelationDetail private constructor(
    /** The names relate recognized. */
    public val names: List<Entity>,
    /** Every scored pair. */
    public val pairs: List<Relation>,
) : Detail {
    /** Builds relation detail inside the library. */
    public companion object {
        internal fun of(names: List<Entity>, pairs: List<Relation>): RelationDetail = RelationDetail(immutableList(names), immutableList(pairs))
    }
    override fun toString(): String = "RelationDetail(names=$names, pairs=$pairs)"
    override fun equals(other: Any?): Boolean = other is RelationDetail && names == other.names && pairs == other.pairs
    override fun hashCode(): Int = Objects.hash(names, pairs)
}

/** One recognition piece and its boundary tag probabilities. */
@Serializable(with = PieceDetailSerializer::class)
public class PieceDetail internal constructor(
    /** Where the piece is. */
    public val span: Span,
    tags: Map<BoundaryTag, Double>,
) {
    /** Each boundary tag's probability. */
    public val tags: Map<BoundaryTag, Double> = immutableMap(tags)
    override fun toString(): String = "PieceDetail(span=$span, tags=$tags)"
    override fun equals(other: Any?): Boolean = other is PieceDetail && span == other.span && tags == other.tags
    override fun hashCode(): Int = Objects.hash(span, tags)
}

/** One recognized name's kind probabilities, with UTF-16 offsets as in [Span]. */
@Serializable(with = dev.thinkthen.kotlin.internal.KindDetailSerializer::class)
public class KindDetail private constructor(internal val span: Span, kinds: Map<String, Double>) {
    public val text: String get() = span.text
    public val start: Int get() = span.start
    public val end: Int get() = span.end
    public val kinds: Map<String, Double> = immutableMap(kinds)
    public companion object {
        internal fun from(span: Span, kinds: Map<String, Double>): KindDetail = KindDetail(span, kinds)
        internal fun of(text: String, start: Int, end: Int, kinds: Map<String, Double>): KindDetail = KindDetail(Span(text, start, end), kinds)
    }
    override fun toString(): String = "KindDetail(text=$text, start=$start, end=$end, kinds=$kinds)"
    override fun equals(other: Any?): Boolean = other is KindDetail && span == other.span && kinds == other.kinds
    override fun hashCode(): Int = Objects.hash(span, kinds)
}

/** RecognizeQuestion detail: every piece and every name's kind probabilities. */
@Serializable
public class RecognizeDetail private constructor(
    /** Every piece. */
    public val pieces: List<PieceDetail>,
    /** Every name's kind probabilities. */
    public val names: List<KindDetail>,
) : Detail {
    /** Builds recognition detail inside the library. */
    public companion object {
        internal fun of(pieces: List<PieceDetail>, names: List<KindDetail>): RecognizeDetail = RecognizeDetail(immutableList(pieces), immutableList(names))
    }
    override fun toString(): String = "RecognizeDetail(pieces=$pieces, names=$names)"
    override fun equals(other: Any?): Boolean = other is RecognizeDetail && pieces == other.pieces && names == other.names
    override fun hashCode(): Int = Objects.hash(pieces, names)
}

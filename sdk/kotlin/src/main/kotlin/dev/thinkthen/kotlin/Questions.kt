package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.describe
import dev.thinkthen.kotlin.internal.described
import dev.thinkthen.kotlin.internal.immutableList
import java.util.Objects

/** A typed question whose answers have type [A] and detail [D]. Questions are immutable and compare by value. */
public sealed interface Question<out A, out D> {
    /** The execution name recorded in each row's [Source], or null. */
    public val name: String?
}

/** A question a [Toolbox] can hold. */
public sealed interface EntryQuestion<out A, out D> : Question<A, D>

/** A question an annotation can ask: every function except rank. */
public sealed interface AnnotationQuestion<out A, out D> : EntryQuestion<A, D>

/** A question that one-record calls such as [ask] accept: every function except filter and rank. */
public sealed interface SingleQuestion<out A, out D> : Question<A, D>

/**
 * A yes-or-no question.
 *
 * [yesMeans] and [noMeans] optionally say what each answer means, such as "The customer asks for money back."
 */
public class Decide(
    /** The question text. */
    public val question: String,
    /** When the answer counts; a band makes answers inside it not sure. */
    public val threshold: Threshold? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    /** What yes means, or null. */
    public val yesMeans: String? = null,
    /** What no means, or null. */
    public val noMeans: String? = null,
) : AnnotationQuestion<Boolean, Nothing>, SingleQuestion<Boolean, Nothing> {
    override fun equals(other: Any?): Boolean = other is Decide && question == other.question && threshold == other.threshold &&
        field == other.field && name == other.name && yesMeans == other.yesMeans && noMeans == other.noMeans
    override fun hashCode(): Int = Objects.hash(question, threshold, field, name, yesMeans, noMeans)
    override fun toString(): String = describe("Decide", "question" to question, "threshold" to threshold, "field" to field,
        "name" to name, "yesMeans" to yesMeans, "noMeans" to noMeans)
}

/**
 * Choose one of [choices].
 *
 * [descriptions] optionally maps a choice to what it means, such as `mapOf("billing" to "Invoices, fees and refunds.")`.
 */
public class Choose(
    /** The question text. */
    public val question: String,
    choices: List<String>,
    /** When the answer counts; a band makes answers inside it not sure. */
    public val threshold: Threshold? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    descriptions: Map<String, String> = emptyMap(),
) : AnnotationQuestion<String, LabelDetail>, SingleQuestion<String, LabelDetail> {
    /** The choices, in order. */
    public val choices: List<String> = immutableList(choices)

    /** What each described choice means. */
    public val descriptions: Map<String, String> = described(this.choices, descriptions)

    override fun equals(other: Any?): Boolean = other is Choose && question == other.question && choices == other.choices &&
        threshold == other.threshold && field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, choices, threshold, field, name, descriptions)
    override fun toString(): String = describe("Choose", "question" to question, "choices" to choices, "threshold" to threshold,
        "field" to field, "name" to name, "descriptions" to descriptions)
}

/** Every label from [labels] that fits. [descriptions] optionally maps a label to what it means. */
public class Tag(
    /** The question text. */
    public val question: String,
    labels: List<String>,
    /** The cut point for each label. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    descriptions: Map<String, String> = emptyMap(),
) : AnnotationQuestion<List<String>, LabelDetail>, SingleQuestion<List<String>, LabelDetail> {
    /** The labels, in order. */
    public val labels: List<String> = immutableList(labels)

    /** What each described label means. */
    public val descriptions: Map<String, String> = described(this.labels, descriptions)

    override fun equals(other: Any?): Boolean = other is Tag && question == other.question && labels == other.labels &&
        threshold == other.threshold && field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, labels, threshold, field, name, descriptions)
    override fun toString(): String = describe("Tag", "question" to question, "labels" to labels, "threshold" to threshold,
        "field" to field, "name" to name, "descriptions" to descriptions)
}

/** A score on the scale of [levels], lowest first. [descriptions] optionally maps a level to what it means. */
public class Score(
    /** The question text. */
    public val question: String,
    levels: List<String>,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    descriptions: Map<String, String> = emptyMap(),
) : AnnotationQuestion<Double, LevelDetail>, SingleQuestion<Double, LevelDetail> {
    /** The levels, lowest first. */
    public val levels: List<String> = immutableList(levels)

    /** What each described level means. */
    public val descriptions: Map<String, String> = described(this.levels, descriptions)

    override fun equals(other: Any?): Boolean = other is Score && question == other.question && levels == other.levels &&
        field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, levels, field, name, descriptions)
    override fun toString(): String = describe("Score", "question" to question, "levels" to levels, "field" to field,
        "name" to name, "descriptions" to descriptions)
}

/** Keep the records whose answer is yes. [yesMeans] and [noMeans] optionally say what each answer means. */
public class Filter(
    /** The question text. */
    public val question: String,
    /** When the answer counts; a band leaves records inside it out. */
    public val threshold: Threshold? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    /** What yes means, or null. */
    public val yesMeans: String? = null,
    /** What no means, or null. */
    public val noMeans: String? = null,
) : AnnotationQuestion<Boolean, Nothing> {
    override fun equals(other: Any?): Boolean = other is Filter && question == other.question && threshold == other.threshold &&
        field == other.field && name == other.name && yesMeans == other.yesMeans && noMeans == other.noMeans
    override fun hashCode(): Int = Objects.hash(question, threshold, field, name, yesMeans, noMeans)
    override fun toString(): String = describe("Filter", "question" to question, "threshold" to threshold, "field" to field,
        "name" to name, "yesMeans" to yesMeans, "noMeans" to noMeans)
}

/** Rank records on the scale of [levels], lowest first, keeping the [top] ranks when set. */
public class Rank(
    /** The question text. */
    public val question: String,
    levels: List<String>,
    /** How many ranks to keep, or null for every record. */
    public val top: Int? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    descriptions: Map<String, String> = emptyMap(),
) : EntryQuestion<Long, LevelDetail> {
    /** The levels, lowest first. */
    public val levels: List<String> = immutableList(levels)

    /** What each described level means. */
    public val descriptions: Map<String, String> = described(this.levels, descriptions)

    override fun equals(other: Any?): Boolean = other is Rank && question == other.question && levels == other.levels &&
        top == other.top && field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, levels, top, field, name, descriptions)
    override fun toString(): String = describe("Rank", "question" to question, "levels" to levels, "top" to top,
        "field" to field, "name" to name, "descriptions" to descriptions)
}

/** Find the line that answers the question. The answer is its [Span], or null when no line answers. */
public class Find(
    /** The question text. */
    public val question: String,
    /** The cut point. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : AnnotationQuestion<Span?, Nothing>, SingleQuestion<Span?, Nothing> {
    override fun equals(other: Any?): Boolean = other is Find && question == other.question && threshold == other.threshold &&
        field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(question, threshold, field, name)
    override fun toString(): String = describe("Find", "question" to question, "threshold" to threshold, "field" to field, "name" to name)
}

/** Recognize names of the given [kinds]. */
public class Recognize(
    kinds: List<String>,
    /** The cut point. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : AnnotationQuestion<List<Entity>, RecognizeDetail>, SingleQuestion<List<Entity>, RecognizeDetail> {
    /** The kinds, in order. */
    public val kinds: List<String> = immutableList(kinds)

    override fun equals(other: Any?): Boolean = other is Recognize && kinds == other.kinds && threshold == other.threshold &&
        field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(kinds, threshold, field, name)
    override fun toString(): String = describe("Recognize", "kinds" to kinds, "threshold" to threshold, "field" to field, "name" to name)
}

/** Find the relations that [rules] describe. */
public class Relate(
    rules: List<RelationRule>,
    /** The cut point. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : AnnotationQuestion<List<Relation>, RelationDetail>, SingleQuestion<List<Relation>, RelationDetail> {
    /** The rules, in order. */
    public val rules: List<RelationRule> = immutableList(rules)

    override fun equals(other: Any?): Boolean = other is Relate && rules == other.rules && threshold == other.threshold &&
        field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(rules, threshold, field, name)
    override fun toString(): String = describe("Relate", "rules" to rules, "threshold" to threshold, "field" to field, "name" to name)
}

/** Ask several named [questions] about one record. Pass it to [ask], or call [annotate]. */
public class Annotate(
    /** The named questions. */
    public val questions: AnnotationQuestions,
    /** The field inside a JSON record, for every question without its own. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : SingleQuestion<Annotations, Nothing> {
    override fun equals(other: Any?): Boolean = other is Annotate && questions == other.questions && field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(questions, field, name)
    override fun toString(): String = describe("Annotate", "questions" to questions.entries, "field" to field, "name" to name)
}

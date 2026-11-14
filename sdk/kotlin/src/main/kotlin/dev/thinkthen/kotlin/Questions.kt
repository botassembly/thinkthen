package dev.thinkthen.kotlin

import dev.thinkthen.kotlin.internal.Domain

import dev.thinkthen.kotlin.internal.describe
import dev.thinkthen.kotlin.internal.described
import dev.thinkthen.kotlin.internal.immutableList
import java.util.Objects
import kotlin.reflect.KClass

/** A typed question whose answers have type [A] and detail [D]. Questions are immutable and compare by value. */
public sealed interface Question<out A, out D> {
    /** The execution name recorded in each row's [Source], or null. */
    public val name: String?
}

/** A question a [Toolbox] can hold. */
public sealed interface EntryQuestion<out A, out D> : Question<A, D>

/** A question an annotation can ask: every function except filter and rank. */
public sealed interface AnnotationQuestion<out A, out D> : EntryQuestion<A, D>

/** A question that one-record calls such as [decide] accept: every function except filter and rank. */
public sealed interface SingleQuestion<out A, out D> : Question<A, D>

/**
 * A yes-or-no question.
 *
 * [yesMeans] and [noMeans] optionally say what each answer means, such as "The customer asks for money back."
 */
public class DecideQuestion(
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
    override fun equals(other: Any?): Boolean = other is DecideQuestion && question == other.question && threshold == other.threshold &&
        field == other.field && name == other.name && yesMeans == other.yesMeans && noMeans == other.noMeans
    override fun hashCode(): Int = Objects.hash(question, threshold, field, name, yesMeans, noMeans)
    override fun toString(): String = describe("DecideQuestion", "question" to question, "threshold" to threshold, "field" to field,
        "name" to name, "yesMeans" to yesMeans, "noMeans" to noMeans)
}

/**
 * ChooseQuestion one of [choices].
 *
 * [descriptions] optionally maps a choice to what it means, such as `mapOf("billing" to "Invoices, fees and refunds.")`.
 */
public class ChooseQuestion<E : Any>(
    /** The question text. */
    public val question: String,
    choices: List<E>,
    /** The cut point the best choice must reach; below it the answer is not sure. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    descriptions: Map<E, String> = emptyMap(),
) : AnnotationQuestion<E, LabelDetail>, SingleQuestion<E, LabelDetail> {
    /** The choices, in order. */
    internal val domain: Domain<E> = Domain(choices)
    public val choices: List<String> = domain.words

    /** What each described choice means. */
    public val descriptions: Map<String, String> = described(this.choices, descriptions.mapKeys { it.key.toString() })

    public constructor(question: String, type: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null,
        name: String? = null, descriptions: Map<E, String> = emptyMap()) :
        this(question, Domain.enums(type).values, threshold, field, name, descriptions)

    override fun equals(other: Any?): Boolean = other is ChooseQuestion<*> && question == other.question && choices == other.choices &&
        threshold == other.threshold && field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, choices, threshold, field, name, descriptions)
    override fun toString(): String = describe("ChooseQuestion", "question" to question, "choices" to choices, "threshold" to threshold,
        "field" to field, "name" to name, "descriptions" to descriptions)
}

/** Every label from [labels] that fits. [descriptions] optionally maps a label to what it means. */
public class TagQuestion<E : Any>(
    /** The question text. */
    public val question: String,
    labels: List<E>,
    /** The cut point for each label. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    descriptions: Map<E, String> = emptyMap(),
) : AnnotationQuestion<List<E>, LabelDetail>, SingleQuestion<List<E>, LabelDetail> {
    /** The labels, in order. */
    internal val domain: Domain<E> = Domain(labels)
    public val labels: List<String> = domain.words

    /** What each described label means. */
    public val descriptions: Map<String, String> = described(this.labels, descriptions.mapKeys { it.key.toString() })

    public constructor(question: String, type: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null,
        name: String? = null, descriptions: Map<E, String> = emptyMap()) :
        this(question, Domain.enums(type).values, threshold, field, name, descriptions)

    override fun equals(other: Any?): Boolean = other is TagQuestion<*> && question == other.question && labels == other.labels &&
        threshold == other.threshold && field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, labels, threshold, field, name, descriptions)
    override fun toString(): String = describe("TagQuestion", "question" to question, "labels" to labels, "threshold" to threshold,
        "field" to field, "name" to name, "descriptions" to descriptions)
}

/** A score on the scale of [levels], lowest first. [descriptions] optionally maps a level to what it means. */
public class ScoreQuestion(
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

    public constructor(question: String, type: KClass<out Enum<*>>, field: Pointer? = null, name: String? = null,
        descriptions: Map<out Enum<*>, String> = emptyMap()) : this(question, type.java.enumConstants.map { it.toString() }, field, name, descriptions.mapKeys { it.key.toString() })

    override fun equals(other: Any?): Boolean = other is ScoreQuestion && question == other.question && levels == other.levels &&
        field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, levels, field, name, descriptions)
    override fun toString(): String = describe("ScoreQuestion", "question" to question, "levels" to levels, "field" to field,
        "name" to name, "descriptions" to descriptions)
}

/** Keep the records whose answer is yes. [yesMeans] and [noMeans] optionally say what each answer means. */
public class FilterQuestion(
    /** The question text. */
    public val question: String,
    /** The cut point separating kept and rejected records. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
    /** What yes means, or null. */
    public val yesMeans: String? = null,
    /** What no means, or null. */
    public val noMeans: String? = null,
) : EntryQuestion<Boolean, Nothing> {
    override fun equals(other: Any?): Boolean = other is FilterQuestion && question == other.question && threshold == other.threshold &&
        field == other.field && name == other.name && yesMeans == other.yesMeans && noMeans == other.noMeans
    override fun hashCode(): Int = Objects.hash(question, threshold, field, name, yesMeans, noMeans)
    override fun toString(): String = describe("FilterQuestion", "question" to question, "threshold" to threshold, "field" to field,
        "name" to name, "yesMeans" to yesMeans, "noMeans" to noMeans)
}

/** RankQuestion records on the scale of [levels], lowest first, keeping the [top] ranks when set. */
public class RankQuestion(
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

    public constructor(question: String, type: KClass<out Enum<*>>, top: Int? = null, field: Pointer? = null, name: String? = null,
        descriptions: Map<out Enum<*>, String> = emptyMap()) : this(question, type.java.enumConstants.map { it.toString() }, top, field, name, descriptions.mapKeys { it.key.toString() })

    override fun equals(other: Any?): Boolean = other is RankQuestion && question == other.question && levels == other.levels &&
        top == other.top && field == other.field && name == other.name && descriptions == other.descriptions
    override fun hashCode(): Int = Objects.hash(question, levels, top, field, name, descriptions)
    override fun toString(): String = describe("RankQuestion", "question" to question, "levels" to levels, "top" to top,
        "field" to field, "name" to name, "descriptions" to descriptions)
}

/** FindQuestion the line that answers the question. The answer is its [Span], or null when no line answers. */
public class FindQuestion(
    /** The question text. */
    public val question: String,
    /** The cut point. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : AnnotationQuestion<Span?, Nothing>, SingleQuestion<Span?, Nothing> {
    override fun equals(other: Any?): Boolean = other is FindQuestion && question == other.question && threshold == other.threshold &&
        field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(question, threshold, field, name)
    override fun toString(): String = describe("FindQuestion", "question" to question, "threshold" to threshold, "field" to field, "name" to name)
}

/** RecognizeQuestion names of the given [kinds]. */
public class RecognizeQuestion(
    kinds: List<String>,
    /** The cut point. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : AnnotationQuestion<List<Entity>, RecognizeDetail>, SingleQuestion<List<Entity>, RecognizeDetail> {
    /** The kinds, in order. */
    public val kinds: List<String> = immutableList(kinds)

    override fun equals(other: Any?): Boolean = other is RecognizeQuestion && kinds == other.kinds && threshold == other.threshold &&
        field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(kinds, threshold, field, name)
    override fun toString(): String = describe("RecognizeQuestion", "kinds" to kinds, "threshold" to threshold, "field" to field, "name" to name)
}

/** FindQuestion the relations that [rules] describe. */
public class RelateQuestion(
    rules: List<RelationRule>,
    /** The cut point. */
    public val threshold: Threshold.Cut? = null,
    /** The field inside a JSON record. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : AnnotationQuestion<List<Relation>, RelationDetail>, SingleQuestion<List<Relation>, RelationDetail> {
    /** The rules, in order. */
    public val rules: List<RelationRule> = immutableList(rules)

    override fun equals(other: Any?): Boolean = other is RelateQuestion && rules == other.rules && threshold == other.threshold &&
        field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(rules, threshold, field, name)
    override fun toString(): String = describe("RelateQuestion", "rules" to rules, "threshold" to threshold, "field" to field, "name" to name)
}

/** Ask several named [questions] about one record. Pass it to [annotate]. */
public class AnnotateQuestion(
    /** The named questions. */
    public val questions: AnnotationQuestions,
    /** The field inside a JSON record, for every question without its own. */
    public val field: Pointer? = null,
    override val name: String? = null,
) : SingleQuestion<Annotations, Nothing> {
    override fun equals(other: Any?): Boolean = other is AnnotateQuestion && questions == other.questions && field == other.field && name == other.name
    override fun hashCode(): Int = Objects.hash(questions, field, name)
    override fun toString(): String = describe("AnnotateQuestion", "questions" to questions.entries, "field" to field, "name" to name)
}

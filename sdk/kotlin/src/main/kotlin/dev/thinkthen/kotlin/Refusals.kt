package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow
import kotlin.reflect.KClass

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Iterable<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Array<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Sequence<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Flow<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: DecideQuestion, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: DecideQuestion, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: DecideQuestion, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use decideStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: DecideQuestion, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Iterable<*>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Array<*>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Sequence<*>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Flow<*>, choices: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.choose(question: String, record: Iterable<*>, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.choose(question: String, record: Array<*>, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.choose(question: String, record: Sequence<*>, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.choose(question: String, record: Flow<*>, choices: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use chooseStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.choose(question: ChooseQuestion<E>, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Iterable<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Array<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Sequence<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Flow<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.tag(question: String, record: Iterable<*>, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.tag(question: String, record: Array<*>, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.tag(question: String, record: Sequence<*>, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.tag(question: String, record: Flow<*>, labels: KClass<E>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.tag(question: TagQuestion<E>, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.tag(question: TagQuestion<E>, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.tag(question: TagQuestion<E>, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use tagStream", level = DeprecationLevel.ERROR)
public fun <E : Any> ThinkThen.tag(question: TagQuestion<E>, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Iterable<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Array<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Sequence<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Flow<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.score(question: String, record: Iterable<*>, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.score(question: String, record: Array<*>, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.score(question: String, record: Sequence<*>, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun <E : Enum<E>> ThinkThen.score(question: String, record: Flow<*>, levels: KClass<E>, field: Pointer? = null, name: String? = null, descriptions: Map<E, String> = emptyMap()): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: ScoreQuestion, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: ScoreQuestion, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: ScoreQuestion, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use scoreStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.score(question: ScoreQuestion, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Iterable<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Array<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Sequence<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Flow<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: FindQuestion, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: FindQuestion, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: FindQuestion, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use findStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.find(question: FindQuestion, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Iterable<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Array<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Sequence<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Flow<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(question: RecognizeQuestion, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(question: RecognizeQuestion, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(question: RecognizeQuestion, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use recognizeStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.recognize(question: RecognizeQuestion, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Iterable<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Array<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Sequence<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Flow<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(question: RelateQuestion, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(question: RelateQuestion, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(question: RelateQuestion, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use relateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.relate(question: RelateQuestion, record: Flow<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Iterable<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Array<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Sequence<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Flow<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(question: AnnotateQuestion, record: Iterable<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(question: AnnotateQuestion, record: Array<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(question: AnnotateQuestion, record: Sequence<*>): Nothing = refusedMany()

/** Refuse many records before a one-record call can be made. */
@Deprecated("A one-record call takes one record; use annotateStream", level = DeprecationLevel.ERROR)
public fun ThinkThen.annotate(question: AnnotateQuestion, record: Flow<*>): Nothing = refusedMany()

private fun refusedMany(): Nothing = throw IllegalArgumentException("Use the function’s Stream form")

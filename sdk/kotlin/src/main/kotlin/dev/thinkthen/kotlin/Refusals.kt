package dev.thinkthen.kotlin

import kotlinx.coroutines.flow.Flow

// One-record calls refuse many records when the code is compiled. Each overload below is more specific
// than the generic record parameter, so a list, array, sequence or flow resolves here and the
// compiler reports the error with the many-record call to use instead.

/** Refuses a list or other collection as one record: pass many records to [decideStream]. */
@Deprecated("A one-record call takes one record; pass many records to decideStream", ReplaceWith("decideStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Iterable<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [decideStream]. */
@Deprecated("A one-record call takes one record; pass many records to decideStream", ReplaceWith("decideStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Array<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [decideStream]. */
@Deprecated("A one-record call takes one record; pass many records to decideStream", ReplaceWith("decideStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Sequence<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [decideStream]. */
@Deprecated("A one-record call takes one record; pass many records to decideStream", ReplaceWith("decideStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.decide(question: String, record: Flow<*>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, yesMeans: String? = null, noMeans: String? = null): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [chooseStream]. */
@Deprecated("A one-record call takes one record; pass many records to chooseStream", ReplaceWith("chooseStream(question, record, choices)"), DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Iterable<*>, choices: List<String>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [chooseStream]. */
@Deprecated("A one-record call takes one record; pass many records to chooseStream", ReplaceWith("chooseStream(question, record, choices)"), DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Array<*>, choices: List<String>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [chooseStream]. */
@Deprecated("A one-record call takes one record; pass many records to chooseStream", ReplaceWith("chooseStream(question, record, choices)"), DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Sequence<*>, choices: List<String>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [chooseStream]. */
@Deprecated("A one-record call takes one record; pass many records to chooseStream", ReplaceWith("chooseStream(question, record, choices)"), DeprecationLevel.ERROR)
public fun ThinkThen.choose(question: String, record: Flow<*>, choices: List<String>, threshold: Threshold? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [tagStream]. */
@Deprecated("A one-record call takes one record; pass many records to tagStream", ReplaceWith("tagStream(question, record, labels)"), DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Iterable<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [tagStream]. */
@Deprecated("A one-record call takes one record; pass many records to tagStream", ReplaceWith("tagStream(question, record, labels)"), DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Array<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [tagStream]. */
@Deprecated("A one-record call takes one record; pass many records to tagStream", ReplaceWith("tagStream(question, record, labels)"), DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Sequence<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [tagStream]. */
@Deprecated("A one-record call takes one record; pass many records to tagStream", ReplaceWith("tagStream(question, record, labels)"), DeprecationLevel.ERROR)
public fun ThinkThen.tag(question: String, record: Flow<*>, labels: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [scoreStream]. */
@Deprecated("A one-record call takes one record; pass many records to scoreStream", ReplaceWith("scoreStream(question, record, levels)"), DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Iterable<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [scoreStream]. */
@Deprecated("A one-record call takes one record; pass many records to scoreStream", ReplaceWith("scoreStream(question, record, levels)"), DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Array<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [scoreStream]. */
@Deprecated("A one-record call takes one record; pass many records to scoreStream", ReplaceWith("scoreStream(question, record, levels)"), DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Sequence<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [scoreStream]. */
@Deprecated("A one-record call takes one record; pass many records to scoreStream", ReplaceWith("scoreStream(question, record, levels)"), DeprecationLevel.ERROR)
public fun ThinkThen.score(question: String, record: Flow<*>, levels: List<String>, field: Pointer? = null, name: String? = null, descriptions: Map<String, String> = emptyMap()): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [findStream]. */
@Deprecated("A one-record call takes one record; pass many records to findStream", ReplaceWith("findStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Iterable<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [findStream]. */
@Deprecated("A one-record call takes one record; pass many records to findStream", ReplaceWith("findStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Array<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [findStream]. */
@Deprecated("A one-record call takes one record; pass many records to findStream", ReplaceWith("findStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Sequence<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [findStream]. */
@Deprecated("A one-record call takes one record; pass many records to findStream", ReplaceWith("findStream(question, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.find(question: String, record: Flow<*>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [recognizeStream]. */
@Deprecated("A one-record call takes one record; pass many records to recognizeStream", ReplaceWith("recognizeStream(record, kinds)"), DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Iterable<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [recognizeStream]. */
@Deprecated("A one-record call takes one record; pass many records to recognizeStream", ReplaceWith("recognizeStream(record, kinds)"), DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Array<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [recognizeStream]. */
@Deprecated("A one-record call takes one record; pass many records to recognizeStream", ReplaceWith("recognizeStream(record, kinds)"), DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Sequence<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [recognizeStream]. */
@Deprecated("A one-record call takes one record; pass many records to recognizeStream", ReplaceWith("recognizeStream(record, kinds)"), DeprecationLevel.ERROR)
public fun ThinkThen.recognize(record: Flow<*>, kinds: List<String>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [relateStream]. */
@Deprecated("A one-record call takes one record; pass many records to relateStream", ReplaceWith("relateStream(record, rules)"), DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Iterable<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [relateStream]. */
@Deprecated("A one-record call takes one record; pass many records to relateStream", ReplaceWith("relateStream(record, rules)"), DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Array<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [relateStream]. */
@Deprecated("A one-record call takes one record; pass many records to relateStream", ReplaceWith("relateStream(record, rules)"), DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Sequence<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [relateStream]. */
@Deprecated("A one-record call takes one record; pass many records to relateStream", ReplaceWith("relateStream(record, rules)"), DeprecationLevel.ERROR)
public fun ThinkThen.relate(record: Flow<*>, rules: List<RelationRule>, threshold: Threshold.Cut? = null, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [annotateStream]. */
@Deprecated("A one-record call takes one record; pass many records to annotateStream", ReplaceWith("annotateStream(questions, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Iterable<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [annotateStream]. */
@Deprecated("A one-record call takes one record; pass many records to annotateStream", ReplaceWith("annotateStream(questions, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Array<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [annotateStream]. */
@Deprecated("A one-record call takes one record; pass many records to annotateStream", ReplaceWith("annotateStream(questions, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Sequence<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [annotateStream]. */
@Deprecated("A one-record call takes one record; pass many records to annotateStream", ReplaceWith("annotateStream(questions, record)"), DeprecationLevel.ERROR)
public fun ThinkThen.annotate(questions: AnnotationQuestions, record: Flow<*>, field: Pointer? = null, name: String? = null): Nothing = refusedMany()

/** Refuses a list or other collection as one record: pass many records to [askStream]. */
@Deprecated("A one-record call takes one record; pass many records to askStream", ReplaceWith("askStream(question, record)"), DeprecationLevel.ERROR)
public fun <A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: Iterable<*>): Nothing = refusedMany()

/** Refuses an array as one record: pass many records to [askStream]. */
@Deprecated("A one-record call takes one record; pass many records to askStream", ReplaceWith("askStream(question, record)"), DeprecationLevel.ERROR)
public fun <A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: Array<*>): Nothing = refusedMany()

/** Refuses a sequence as one record: pass many records to [askStream]. */
@Deprecated("A one-record call takes one record; pass many records to askStream", ReplaceWith("askStream(question, record)"), DeprecationLevel.ERROR)
public fun <A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: Sequence<*>): Nothing = refusedMany()

/** Refuses a flow as one record: pass many records to [askStream]. */
@Deprecated("A one-record call takes one record; pass many records to askStream", ReplaceWith("askStream(question, record)"), DeprecationLevel.ERROR)
public fun <A, D> ThinkThen.ask(question: SingleQuestion<A, D>, record: Flow<*>): Nothing = refusedMany()

private fun refusedMany(): Nothing = throw IllegalArgumentException("A one-record call takes one record; use its Stream form")

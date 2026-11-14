package dev.thinkthen.kotlin.internal

import dev.thinkthen.kotlin.*

import dev.thinkthen.kotlin.internal.immutableList
import kotlin.reflect.KClass

/** Immutable snapshot of ordered model words and their language values. */
internal class Domain<E : Any>(values: List<E>) {
    init {
        // This adapter supports language strings and enum constants, never mutable caller label objects.
        if (values.any { it !is String && it !is Enum<*> })
            throw UsageException(ErrorCode.USAGE_ARGUMENT, "Choices must be strings or enum constants")
    }
    val values: List<E> = immutableList(values)
    val words: List<String> = immutableList(values.map { it.toString() })
    fun read(word: String): E = values.getOrNull(words.indexOf(word))
        ?: throw UsageException(ErrorCode.USAGE_ARGUMENT, "Unknown choice $word; expected $words")
    fun word(value: E): String = words[values.indexOf(value)]
    companion object {
        fun <E : Any> enums(type: KClass<E>): Domain<E> = Domain(type.java.enumConstants?.toList()
            ?: throw UsageException(ErrorCode.USAGE_ARGUMENT, "${type.simpleName} must be an enum"))
    }
}

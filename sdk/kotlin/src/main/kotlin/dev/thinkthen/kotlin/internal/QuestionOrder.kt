package dev.thinkthen.kotlin.internal

/** Rust's admitted question maps compare UTF-8 bytes, not JVM UTF-16 code units. */
internal val questionNameOrder: Comparator<String> = Comparator { left, right ->
    java.util.Arrays.compareUnsigned(left.toByteArray(Charsets.UTF_8), right.toByteArray(Charsets.UTF_8))
}

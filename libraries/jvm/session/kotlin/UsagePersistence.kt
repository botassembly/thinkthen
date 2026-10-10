// Generated from the C header constants. Do not edit.
package thinkthen.kotlin
data class UsagePersistence(val state: State, val advice: String?) {
enum class State { Disabled, Failed, Pending, Written }
companion object { internal fun read(value: thinkthen.UsagePersistence) =
UsagePersistence(State.valueOf(value.state().name), value.advice()) }
}

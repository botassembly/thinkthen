package thinkthen

// Optional distinguishes absent metadata from a present zero or empty value.
type Optional[T any] struct {
	Present bool
	Value   T
}

func Some[T any](value T) Optional[T] { return Optional[T]{true, value} }

<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

/** Content is withheld from normal diagnostics, including failures. */
abstract class Carrier implements \JsonSerializable
{
    public function __debugInfo(): array { return ['type' => static::class, 'content' => 'withheld']; }
    public function jsonSerialize(): mixed
    {
        $out = new \stdClass();
        foreach (get_object_vars($this) as $k => $v) {
            if ($v instanceof Optional) {
                if (!$v->present) continue;
                $v = $v->value;
            }
            $out->$k = $v;
        }
        return $out;
    }
}
/** @template T */
final class Optional
{
    /** @param T|null $value */
    public function __construct(public readonly bool $present = false, public readonly mixed $value = null) {}
    public function __debugInfo(): array { return ['present' => $this->present]; }
}
abstract class Identity extends Carrier
{
    public function __construct(public readonly string $value)
    {
        if (!preg_match('/\A[0-9a-f]{64}\z/D', $value)) Read::invalid();
    }
    public function jsonSerialize(): string { return $this->value; }
}
final class CallId extends Identity {}
final class SdkRequestId extends Identity {}
final class ObservationId extends Identity {}
final class FailureId extends Identity {}
final class AnswerId extends Identity {}
final class Digest extends Identity {}

/** Arbitrary structured authored content is intentionally JSON. */
final class Authored extends Carrier
{
    private function __construct(public readonly mixed $value) {}
    public static function fromJson(mixed $v): self
    {
        if ($v !== null && !is_string($v) && !is_array($v) && !$v instanceof \stdClass) Read::invalid();
        return new self(Read::json($v));
    }
    public function jsonSerialize(): mixed { return $this->value; }
}
final class Threshold extends Carrier
{
    private function __construct(public readonly ?float $cut, public readonly ?float $low,
                                 public readonly ?float $high, private readonly mixed $original) {}
    public static function fromJson(mixed $v): self
    {
        if ($v === null) return new self(null, null, null, null);
        if ((is_int($v) || is_float($v)) && is_finite((float)$v) && $v > 0 && $v <= 1)
            return new self((float)$v, null, null, $v);
        if (is_string($v) && preg_match('/\A(0(?:\.[0-9]+)?|1(?:\.0+)?):(0(?:\.[0-9]+)?|1(?:\.0+)?)\z/D', $v, $m)) {
            $low = (float)$m[1]; $high = (float)$m[2];
            if ($low < $high) return new self(null, $low, $high, $v);
        }
        Read::invalid();
    }
    public function jsonSerialize(): mixed { return $this->original; }
}
final class Batch extends Carrier
{
    private function __construct(public readonly ?int $records) {}
    public static function fromJson(mixed $v): self
    {
        if ($v === 'max') return new self(null);
        return new self(Read::integer($v, 1));
    }
    public function jsonSerialize(): int|string { return $this->records ?? 'max'; }
}
/** Numeric names are kept as strings, in authored order. */
final class Probability extends Carrier
{
    public function __construct(public readonly string $name, public readonly float $probability) {}
}
final class Probabilities extends Carrier
{
    /** @param list<Probability> $entries */
    private function __construct(public readonly array $entries) {}
    public static function fromJson(mixed $v): self
    {
        $out = [];
        foreach (Read::object($v) as $name => $value) $out[] = new Probability((string)$name, Read::number($value, true));
        return new self($out);
    }
    public function get(string $name): ?float
    {
        foreach ($this->entries as $entry) if ($entry->name === $name) return $entry->probability;
        return null;
    }
    public function jsonSerialize(): \stdClass
    {
        $out = new \stdClass();
        foreach ($this->entries as $entry) $out->{$entry->name} = $entry->probability;
        return $out;
    }
}
final class Pointers extends Carrier
{
    /** @param list<string> $values */
    private function __construct(public readonly array $values, public readonly bool $single) {}
    public static function fromJson(mixed $v): self
    {
        $values = is_string($v) ? [$v] : Read::value('[str]', $v);
        // Native grammar owns pointer validation, including escaping and extraction.
        return new self($values, is_string($v));
    }
    public function jsonSerialize(): array|string { return $this->single ? $this->values[0] : $this->values; }
}
final class Labels extends Carrier
{
    /** @param list<string>|null $names */
    private function __construct(public readonly ?array $names, public readonly ?Descriptions $descriptions) {}
    public static function fromJson(mixed $v): self
    {
        return is_array($v) ? new self(Read::value('[str]', $v), null) : new self(null, Descriptions::fromJson($v));
    }
    public function jsonSerialize(): array|Descriptions { return $this->names ?? $this->descriptions; }
}
abstract class ActionValue extends Carrier
{
    public static function fromJson(mixed $v): self
    {
        if ($v === null) return new Unresolved();
        if (is_bool($v)) return new BooleanValue($v);
        if (is_string($v)) return new LabelValue($v);
        if (is_int($v) || is_float($v)) return new NumberValue(Read::number($v));
        if (is_array($v)) return new LabelValues(Read::value('[str]', $v));
        return new FailedValue(FailedField::fromJson($v)->failed);
    }
}
final class Unresolved extends ActionValue { public function jsonSerialize(): mixed { return null; } }
final class BooleanValue extends ActionValue {
    public function __construct(public readonly bool $value) {}
    public function jsonSerialize(): bool { return $this->value; }
}
final class LabelValue extends ActionValue {
    public function __construct(public readonly string $value) {}
    public function jsonSerialize(): string { return $this->value; }
}
final class NumberValue extends ActionValue {
    public function __construct(public readonly float $value) {}
    public function jsonSerialize(): float { return $this->value; }
}
final class LabelValues extends ActionValue {
    /** @param list<string> $value */
    public function __construct(public readonly array $value) {}
    public function jsonSerialize(): array { return $this->value; }
}
final class FailedValue extends ActionValue {
    public function __construct(public readonly Failure $failed) {}
}
/** Arbitrary caller/author content, including Boolean and null, never a known engine object. */
final class JsonContent extends Carrier {
    private function __construct(public readonly mixed $value) {}
    public static function fromJson(mixed $v): self { return new self(Read::json($v)); }
    public function jsonSerialize(): mixed { return $this->value; }
}

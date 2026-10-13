<?php
declare(strict_types=1);
namespace ThinkThen\Results;

/** @template T */
final readonly class Presence
{
    /** @param T $value */
    public function __construct(public bool $present, public mixed $value) {}
}

/** An owned PHP value; unknown members remain available without affecting truth. */
class Node
{
    public function __construct(private readonly \stdClass $raw, private readonly array $values) {}
    public function has(string $name): bool { return property_exists($this->raw, $name); }
    public function field(string $name): Presence
    {
        $present = $this->has($name);
        $value = array_key_exists($name, $this->values) ? $this->values[$name] : ($present ? $this->raw->$name : null);
        return new Presence($present, $value);
    }
    public function __get(string $name): mixed { return $this->field($name)->value; }
    public function toObject(): \stdClass { return self::copy($this->raw); }
    private static function copy(mixed $value): mixed
    {
        if ($value instanceof self) return $value->toObject();
        if (is_array($value)) return array_map(self::copy(...), $value);
        if ($value instanceof \stdClass) {
            $copy = new \stdClass();
            foreach ($value as $name => $field) $copy->$name = self::copy($field);
            return $copy;
        }
        return $value;
    }
    public function __debugInfo(): array { return ['type' => static::class]; }
}

require_once __DIR__.'/results_generated.php';

final readonly class Completed
{
    use Answers;
    /** @param list<Node> $results */
    public function __construct(public array $results, public Node $terminal, public array $packets = []) {}
    public function facts(): ?Node { return $this->terminal->facts; }
}

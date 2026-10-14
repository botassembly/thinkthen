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
    public function __construct(private readonly array $values) {}
    public function has(string $name): bool { return array_key_exists($name, $this->values); }
    public function field(string $name): Presence
    {
        $present = $this->has($name);
        $value = $present ? $this->values[$name] : null;
        return new Presence($present, $value);
    }
    public function __get(string $name): mixed { return $this->field($name)->value; }
    public function __debugInfo(): array { return ['type' => static::class]; }
}

require_once __DIR__.'/results_generated.php';

final readonly class Completed
{
    use AnswerAccessors;
    /** @param list<Node> $results */
    public function __construct(public array $results, public SessionPacketTerminal $terminal, public array $packets = []) {}
    public function facts(): ?Facts { return $this->terminal->facts; }
}

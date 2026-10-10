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
    public function __construct(private readonly \stdClass $raw, private readonly array $values,
        private readonly ?string $wireJson = null) {}
    public function has(string $name): bool { return property_exists($this->raw, $name); }
    public function field(string $name): Presence
    {
        $present = $this->has($name);
        $value = array_key_exists($name, $this->values) ? $this->values[$name] : ($present ? $this->raw->$name : null);
        return new Presence($present, $value);
    }
    public function __get(string $name): mixed { return $this->field($name)->value; }
    public function toObject(): \stdClass { return unserialize(serialize($this->raw)); }
    /** The complete native packet bytes, including numbers outside PHP's integer range. */
    public function nativeJson(): ?string { return $this->wireJson; }
    public function __debugInfo(): array { return ['type' => static::class]; }
}

/** Schema-directed conversion; field inventories come only from the generator. */
final class Decoder
{
    private static ?array $graph = null;
    public static function packet(string $json): Node
    {
        self::$graph ??= json_decode(file_get_contents(__DIR__.'/graph_generated.json'), true, flags: JSON_THROW_ON_ERROR);
        return self::convert(json_decode($json, flags: JSON_THROW_ON_ERROR | JSON_BIGINT_AS_STRING),
            ['$ref' => '#/$defs/completesessionPacket'], $json);
    }
    private static function matches(\stdClass $value, array $tag): bool
    {
        [$mode, $field, $wanted] = $tag;
        return match ($mode) {
            'literal' => property_exists($value, $field) && $value->$field === $wanted,
            'literals' => self::literals($value, $wanted),
            'member' => property_exists($value, $field),
            'structure' => self::structure($value, $wanted),
            default => throw new \LogicException('unknown generated discriminator'),
        };
    }
    private static function literals(\stdClass $value, array $wanted): bool
    {
        foreach ($wanted as $field => $literal) if (!property_exists($value, $field) || $value->$field !== $literal) return false;
        return true;
    }
    private static function structure(\stdClass $value, array $wanted): bool
    {
        foreach ($wanted['required'] as $field) if (!property_exists($value, $field)) return false;
        foreach ($wanted['excluded'] as $field) if (property_exists($value, $field)) return false;
        return true;
    }
    private static function convert(mixed $value, array|bool $schema, ?string $wireJson = null): mixed
    {
        if ($value === null || $schema === true) return $value;
        if (isset($schema['$ref'])) $schema = self::$graph[substr($schema['$ref'], strlen('#/$defs/'))];
        if (is_array($schema['type'] ?? null)) {
            $kinds = array_values(array_filter($schema['type'], fn($kind) => $kind !== 'null'));
            if (count($kinds) === 1) $schema['type'] = $kinds[0];
        }
        if (isset($schema['anyOf'])) {
            $nonnull = array_values(array_filter($schema['anyOf'], fn($s) => is_array($s) && ($s['type'] ?? '') !== 'null'));
            if (count($nonnull) === 1) return self::convert($value, $nonnull[0]);
        }
        if (isset($schema['variants'])) {
            foreach ($schema['variants'] as [$child, $tag]) {
                if ($value instanceof \stdClass && self::matches($value, $tag)) return self::convert($value, self::$graph[$child], $wireJson);
            }
            throw new \LogicException('native result does not match its generated variants');
        }
        if (isset($schema['class'])) {
            $fields = [];
            foreach ($schema['properties'] as $name => $field) {
                if (property_exists($value, $name)) $fields[$name] = self::convert($value->$name, $field);
            }
            $class = __NAMESPACE__.'\\'.$schema['class'];
            return new $class($value, $fields, $wireJson);
        }
        if (($schema['type'] ?? null) === 'array') return array_map(fn($item) => self::convert($item, $schema['items']), $value);
        if (($schema['type'] ?? null) === 'object' && is_array($schema['additionalProperties'] ?? null)) {
            $result = [];
            foreach ((array)$value as $name => $item) $result[$name] = self::convert($item, $schema['additionalProperties']);
            return $result;
        }
        return $value;
    }
}

final readonly class Completed
{
    /** @param list<Node> $results */
    public function __construct(public array $results, public Node $terminal, public array $packets = []) {}
    public function facts(): ?Node { return $this->terminal->facts; }
}

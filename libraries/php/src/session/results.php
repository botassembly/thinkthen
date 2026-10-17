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

/**
 * @template T
 * @template P of float|array|null
 */
final readonly class Result
{
    /**
     * @param T $value
     * @param P $probability
     * @param list<Node> $results
     */
    public function __construct(public mixed $value, public float|array|null $probability,
        public array $results, public Facts $facts, public array $packets = []) {}
    public function __debugInfo(): array { return ['type' => self::class]; }
}

namespace ThinkThen\Internal;

/** Project owned receipts once; native admission and settlement stay authoritative. */
final class ResultBuilder
{
    public static function complete(string $function, bool $single, array $rows,
        \ThinkThen\Results\SessionPacketTerminal $terminal, array $packets): \ThinkThen\Results\Result
    {
        $shape = CallShapes::VALUES[$function];
        $values = []; $probabilities = [];
        $resolved = self::resolved($shape, $packets);
        foreach ($rows as $row) {
            $values[] = $row->{$shape['value_projection']};
            $probabilities[] = self::probability($function, $shape, $row, $resolved);
        }
        $scalar = $shape['call_shape'] === 'aggregate' || ($shape['call_shape'] === 'input' && $single);
        return new \ThinkThen\Results\Result($scalar ? ($values[0] ?? null) : $values,
            $shape['probability_mode'] === 'null' ? null : ($scalar ? ($probabilities[0] ?? null) : $probabilities),
            $rows, $terminal->facts, $packets);
    }
    private static function read(mixed $value, array $path): mixed
    {
        foreach ($path as $field) $value = $value?->$field;
        return $value;
    }
    private static function resolved(array $shape, array $packets): array
    {
        if (!isset($shape['resolution'])) return [];
        $route = $shape['resolution']; $resolved = [];
        foreach ($packets as $packet) {
            foreach ($route['guards'] as [$path, $literal]) {
                if (self::read($packet, $path) !== $literal) continue 2;
            }
            $resolved[self::read($packet, $route['index_path'])] = self::read($packet, $route['path']) !== null;
        }
        return $resolved;
    }
    private static function probability(string $function, array $shape, \ThinkThen\Results\Node $row, array $resolved): float|array|null
    {
        if ($shape['probability_mode'] === 'null') return null;
        if (isset($shape['resolution']) && !($resolved[$row->index] ?? false)) return null;
        if (isset($shape['selection_path']) && self::read($row, $shape['selection_path']) === null) return null;
        if ($shape['probability_mode'] === 'yes') return (float)self::read($row, $shape['probability_path']);
        if ($shape['probability_mode'] === 'picked') {
            $pick = self::read($row, $shape['pick_path']);
            $odds = self::read($row, $shape['probability_path']);
            return isset($odds->$pick) ? (float)$odds->$pick : null;
        }
        if ($function === 'rank') return $row->answer instanceof \ThinkThen\Results\AnswerYesNo ? (float)$row->answer->probability : null;
        if ($function === 'relate') return array_map(fn($edge) => (float)$edge->probability, $row->value);
        if ($row->value instanceof \ThinkThen\Results\RecognizeModeProposals) {
            return array_map(fn($proposal) => (float)$proposal->probability, $row->value->proposals);
        }
        return array_map(function($entity) use ($row) {
            foreach ($row->answer->proposals as $proposal) {
                if ($proposal->kept && $proposal->kind === $entity->kind
                    && $proposal->selected->start === $entity->start && $proposal->selected->end === $entity->end) {
                    return (float)$proposal->span_probability;
                }
            }
            throw new \ThinkThen\DefectFailure('recognized entity has no kept proposal');
        }, $row->value->entities);
    }
}

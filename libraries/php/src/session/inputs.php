<?php
declare(strict_types=1);
namespace ThinkThen\Inputs;

enum Omitted { case Value; }

/** Input documents retain present nulls and unknown fields for native admission. */
abstract class Document implements \JsonSerializable
{
    public function __construct(private readonly array $fields) {}
    public function jsonSerialize(): object { return (object)$this->fields; }
    public function __debugInfo(): array { return ['type' => static::class]; }
}

namespace ThinkThen\Internal;

/** Keep one validated JSON value unchanged for the native definition parser. */
final readonly class RawJson
{
    public function __construct(public string $json)
    {
        try { json_decode($json, flags: JSON_THROW_ON_ERROR); }
        catch (\JsonException $error) { throw new \ThinkThen\UsageFailure('question is not one JSON value', previous: $error); }
    }
    public function __debugInfo(): array { return ['type' => self::class]; }
}

final class Encoder
{
    private const DEPTH = 512;
    public static function encode(mixed $value): string
    {
        $arrays = [];
        try {
            $value = self::materialize($value, new \SplObjectStorage(), new \SplObjectStorage(), $arrays, 1);
            json_encode($value, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION, self::DEPTH);
        }
        catch (\JsonException $error) { throw new \ThinkThen\UsageFailure('input cannot be converted to native JSON', previous: $error); }
        return self::write($value);
    }
    /** Freeze serializer results before native JSON validation and writing. */
    private static function materialize(mixed &$value, \SplObjectStorage $objects,
        \SplObjectStorage $active, array &$arrays, int $depth): mixed
    {
        if ($depth > self::DEPTH) throw new \JsonException('maximum JSON depth exceeded');
        if ($value instanceof RawJson || $value instanceof \UnitEnum) return $value;
        if (is_object($value)) {
            if ($objects->contains($value)) return $objects[$value];
            if ($active->contains($value)) throw new \JsonException('recursive serializer result');
            if ($value instanceof \JsonSerializable) {
                $active->attach($value);
                try {
                    $serialized = $value->jsonSerialize();
                    if ($serialized === $value) $serialized = (object)get_object_vars($value);
                    return $objects[$value] = self::materialize($serialized, $objects, $active, $arrays, $depth);
                } finally { $active->detach($value); }
            }
            if (!$value instanceof \stdClass && (new \ReflectionObject($value))->isInternal()) {
                return $objects[$value] = new RawJson(json_encode($value, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION, self::DEPTH));
            }
            $result = new \stdClass();
            $objects[$value] = $result;
            foreach (get_object_vars($value) as $key => $child) {
                $result->$key = self::materialize($child, $objects, $active, $arrays, $depth + 1);
            }
            return $result;
        }
        if (!is_array($value)) return $value;
        $identity = \ReflectionReference::fromArrayElement([&$value], 0)->getId();
        if (isset($arrays[$identity])) throw new \JsonException('recursive array');
        $arrays[$identity] = true;
        try {
            $result = [];
            foreach ($value as $key => &$child) $result[$key] = self::materialize($child, $objects, $active, $arrays, $depth + 1);
            return $result;
        } finally { unset($arrays[$identity]); }
    }
    private static function write(mixed $value): string
    {
        if ($value instanceof RawJson) return $value->json;
        if (is_array($value) && array_is_list($value)) {
            return '['.implode(',', array_map(self::write(...), $value)).']';
        }
        if (is_array($value) || $value instanceof \stdClass) {
            $members = [];
            foreach ($value as $key => $child) $members[] = self::write((string)$key).':'.self::write($child);
            return '{'.implode(',', $members).'}';
        }
        return json_encode($value, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION, self::DEPTH);
    }
}

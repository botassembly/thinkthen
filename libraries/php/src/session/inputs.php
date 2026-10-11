<?php
declare(strict_types=1);
namespace ThinkThen\Inputs;

/** Input documents retain present nulls and unknown fields for native admission. */
abstract class Document implements \JsonSerializable
{
    public function __construct(private readonly array $fields) {}
    public function jsonSerialize(): object { return (object)$this->fields; }
    public function __debugInfo(): array { return ['type' => static::class]; }
}

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
    public static function encode(mixed $value): string
    {
        try { json_encode($value, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION); }
        catch (\JsonException $error) { throw new \ThinkThen\UsageFailure('input cannot be converted to native JSON', previous: $error); }
        return self::write($value);
    }
    private static function write(mixed $value): string
    {
        if ($value instanceof RawJson) return $value->json;
        if ($value instanceof \JsonSerializable) $value = $value->jsonSerialize();
        if (is_array($value) && array_is_list($value)) {
            return '['.implode(',', array_map(self::write(...), $value)).']';
        }
        if (is_array($value) || $value instanceof \stdClass) {
            $members = [];
            foreach ($value as $key => $child) $members[] = self::write((string)$key).':'.self::write($child);
            return '{'.implode(',', $members).'}';
        }
        try { return json_encode($value, JSON_THROW_ON_ERROR | JSON_PRESERVE_ZERO_FRACTION); }
        catch (\JsonException $error) { throw new \ThinkThen\UsageFailure('input cannot be converted to native JSON', previous: $error); }
    }
}

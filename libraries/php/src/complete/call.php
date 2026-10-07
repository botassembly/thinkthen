<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

/** @template T */
final class CompleteCall extends Carrier
{
    /** @param T $value @param Optional<list<Attempt>> $attempts */
    public function __construct(public readonly mixed $value, public readonly Facts $facts, public readonly Optional $attempts) {}
    /** @param callable(mixed):T $readValue @return self<T> */
    public static function fromJson(mixed $json, callable $readValue): self
    {
        $v=Read::object($json);
        if (!property_exists($v,'value') || !property_exists($v,'facts')) Read::invalid();
        return new self($readValue($v->value), Facts::fromJson($v->facts), Read::optional($v,'attempts','[Attempt]'));
    }
}
/** Copied reviewed C image view: native alone identifies/decodes media and dimensions. */
final class ImageView extends Carrier
{
    public function __construct(public readonly MediaType $media, public readonly string $bytes,
                                public readonly int $width, public readonly int $height,
                                public readonly Optional $filename = new Optional())
    {
        Read::integer($width,1,4294967295); Read::integer($height,1,4294967295);
        if ($filename->present) Read::value("str", $filename->value);
    }
    public function jsonSerialize(): mixed { throw new \LogicException('image views require the native complete adapter'); }
}

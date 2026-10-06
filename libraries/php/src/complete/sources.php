<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class Files extends Carrier implements Selection
{
    public function __construct(
        /** @var list<string> */
        public readonly array $paths,
        public readonly UnitKind $unit,
        /** @var Optional<int> */
        public readonly Optional $window = new Optional(),
        /** @var Optional<MediaKind> */
        public readonly Optional $media = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Files", $v, ["paths", "unit", "window", "media"], ["paths", "unit"]);
        return new self(
            Read::field($v, "paths", "[str]"),
            Read::field($v, "unit", "unit"),
            Read::optional($v, "window", "positive"),
            Read::optional($v, "media", "media"),
        );
    }
}

final class TextInput extends Carrier implements Selection
{
    public function __construct(
        public readonly Authored $text,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("TextInput", $v, ["text"], ["text"]);
        return new self(
            Read::field($v, "text", "text"),
        );
    }
}

final class RecordInput extends Carrier implements Selection
{
    public function __construct(
        /** @var list<mixed> */
        public readonly array $records,
        /** @var Optional<mixed> */
        public readonly Optional $context = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RecordInput", $v, ["records", "context"], ["records"]);
        return new self(
            Read::field($v, "records", "[json]"),
            Read::optional($v, "context", "json"),
        );
    }
}

final class CandidateInput extends Carrier implements Selection
{
    public function __construct(
        /** @var list<mixed> */
        public readonly array $units,
        /** @var Optional<mixed> */
        public readonly Optional $context = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("CandidateInput", $v, ["units", "context"], ["units"]);
        return new self(
            Read::field($v, "units", "[json]"),
            Read::optional($v, "context", "json"),
        );
    }
}

final class ImageBytes extends Carrier
{
    public function __construct(
        public readonly string $data,
        public readonly MediaType $media,
        /** @var Optional<string> */
        public readonly Optional $name = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ImageBytes", $v, ["data", "media", "name"], ["data", "media"]);
        return new self(
            Read::field($v, "data", "bytes"),
            Read::value("image_format", $v->media),
            Read::optional($v, "name", "str"),
        );
    }
}

final class ImageInput extends Carrier implements Selection
{
    public function __construct(
        /** @var list<ImageBytes> */
        public readonly array $images,
        /** @var Optional<mixed> */
        public readonly Optional $text = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ImageInput", $v, ["images", "text"], ["images"]);
        return new self(
            Read::field($v, "images", "[ImageBytes]"),
            Read::optional($v, "text", "json"),
        );
    }
}

final class Controls extends Carrier
{
    public function __construct(
        /** @var Optional<Batch> */
        public readonly Optional $batch = new Optional(),
        /** @var Optional<mixed> */
        public readonly Optional $context = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
        /** @var Optional<Threshold> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $top = new Optional(),
        /** @var Optional<bool> */
        public readonly Optional $none = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<bool> */
        public readonly Optional $attempts = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $deadline_ms = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Controls", $v, ["batch", "context", "on", "threshold", "top", "none", "model", "attempts", "deadline_ms"], []);
        return new self(
            Read::optional($v, "batch", "batch"),
            Read::optional($v, "context", "json"),
            Read::optional($v, "on", "str|[str]"),
            Read::optional($v, "threshold", "threshold"),
            Read::optional($v, "top", "positive"),
            Read::optional($v, "none", "bool"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "attempts", "bool"),
            Read::optional($v, "deadline_ms", "deadline"),
        );
    }
}

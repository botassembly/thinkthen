<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class YesNo extends Carrier implements AtomicAnswer
{
    public function __construct(
        public readonly string $kind,
        public readonly float $probability,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("YesNo", $v, ["kind", "probability"], ["kind", "probability"]);
        return new self(
            Read::field($v, "kind", "=yes_no"),
            Read::field($v, "probability", "probability"),
        );
    }
}

final class Choice extends Carrier implements AtomicAnswer
{
    public function __construct(
        public readonly string $kind,
        public readonly string $pick,
        public readonly Probabilities $probabilities,
        /** @var Optional<float> */
        public readonly Optional $confidence = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Choice", $v, ["kind", "pick", "probabilities", "confidence"], ["kind", "pick", "probabilities"]);
        return new self(
            Read::field($v, "kind", "=choice"),
            Read::field($v, "pick", "str"),
            Read::field($v, "probabilities", "{probability}"),
            Read::optional($v, "confidence", "probability"),
        );
    }
}

final class Tags extends Carrier implements AtomicAnswer
{
    public function __construct(
        public readonly string $kind,
        public readonly Probabilities $probabilities,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Tags", $v, ["kind", "probabilities"], ["kind", "probabilities"]);
        return new self(
            Read::field($v, "kind", "=tag"),
            Read::field($v, "probabilities", "{probability}"),
        );
    }
}

final class Score extends Carrier implements AtomicAnswer
{
    public function __construct(
        public readonly string $kind,
        public readonly string $level,
        public readonly Probabilities $probabilities,
        /** @var Optional<float> */
        public readonly Optional $confidence = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Score", $v, ["kind", "level", "probabilities", "confidence"], ["kind", "level", "probabilities"]);
        return new self(
            Read::field($v, "kind", "=score"),
            Read::field($v, "level", "str"),
            Read::field($v, "probabilities", "{probability}"),
            Read::optional($v, "confidence", "probability"),
        );
    }
}

final class FindAnswer extends Carrier implements AtomicAnswer
{
    public function __construct(
        public readonly string $kind,
        public readonly string $pick,
        public readonly Probabilities $probabilities,
        /** @var Optional<float> */
        public readonly Optional $confidence = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FindAnswer", $v, ["kind", "pick", "probabilities", "confidence"], ["kind", "pick", "probabilities"]);
        return new self(
            Read::field($v, "kind", "=find"),
            Read::field($v, "pick", "str"),
            Read::field($v, "probabilities", "{probability}"),
            Read::optional($v, "confidence", "probability"),
        );
    }
}

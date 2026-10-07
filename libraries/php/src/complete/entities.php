<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class Entity extends Carrier
{
    public function __construct(
        public readonly string $text,
        public readonly int $start,
        public readonly int $end,
        public readonly int $length,
        public readonly string $kind,
        public readonly float $strength,
        /** @var Optional<string> */
        public readonly Optional $file = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $first_line = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $last_line = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Entity", $v, ["text", "start", "end", "length", "kind", "strength", "file", "first_line", "last_line"], ["text", "start", "end", "length", "kind", "strength"]);
        return new self(
            Read::field($v, "text", "str"),
            Read::field($v, "start", "uint"),
            Read::field($v, "end", "uint"),
            Read::field($v, "length", "uint"),
            Read::field($v, "kind", "str"),
            Read::field($v, "strength", "probability"),
            Read::optional($v, "file", "str"),
            Read::optional($v, "first_line", "positive"),
            Read::optional($v, "last_line", "positive"),
        );
    }
}

final class Endpoint extends Carrier
{
    public function __construct(
        public readonly string $name,
        public readonly string $kind,
        /** @var Optional<mixed> */
        public readonly Optional $record = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $file = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $first_line = new Optional(),
        /** @var Optional<int> */
        public readonly Optional $last_line = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Endpoint", $v, ["name", "kind", "record", "file", "first_line", "last_line"], ["name", "kind"]);
        return new self(
            Read::field($v, "name", "str"),
            Read::field($v, "kind", "str"),
            Read::optional($v, "record", "json"),
            Read::optional($v, "file", "str"),
            Read::optional($v, "first_line", "positive"),
            Read::optional($v, "last_line", "positive"),
        );
    }
}

final class Edge extends Carrier
{
    public function __construct(
        public readonly string $relation,
        public readonly Endpoint $source,
        public readonly Endpoint $target,
        public readonly float $probability,
        /** @var Optional<bool> */
        public readonly Optional $either = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Edge", $v, ["relation", "source", "target", "probability", "either"], ["relation", "source", "target", "probability"]);
        return new self(
            Read::field($v, "relation", "str"),
            Read::field($v, "source", "Endpoint"),
            Read::field($v, "target", "Endpoint"),
            Read::field($v, "probability", "probability"),
            Read::optional($v, "either", "=true"),
        );
    }
}

final class EntityEdge extends Carrier
{
    public function __construct(
        public readonly string $relation,
        public readonly Entity $source,
        public readonly Entity $target,
        public readonly float $probability,
        /** @var Optional<bool> */
        public readonly Optional $either = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("EntityEdge", $v, ["relation", "source", "target", "probability", "either"], ["relation", "source", "target", "probability"]);
        return new self(
            Read::field($v, "relation", "str"),
            Read::field($v, "source", "Entity"),
            Read::field($v, "target", "Entity"),
            Read::field($v, "probability", "probability"),
            Read::optional($v, "either", "=true"),
        );
    }
}

final class Recognition extends Carrier
{
    public function __construct(
        /** @var list<Entity> */
        public readonly array $entities,
        /** @var Optional<list<EntityEdge>> */
        public readonly Optional $relations = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Recognition", $v, ["entities", "relations"], ["entities"]);
        return new self(
            Read::field($v, "entities", "[Entity]"),
            Read::optional($v, "relations", "[EntityEdge]"),
        );
    }
}

final class PieceOdds extends Carrier
{
    public function __construct(
        public readonly int $start,
        public readonly int $end,
        public readonly Probabilities $tags,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("PieceOdds", $v, ["start", "end", "tags"], ["start", "end", "tags"]);
        return new self(
            Read::field($v, "start", "uint"),
            Read::field($v, "end", "uint"),
            Read::field($v, "tags", "{probability}"),
        );
    }
}

final class NameOdds extends Carrier
{
    public function __construct(
        public readonly int $start,
        public readonly int $end,
        public readonly Probabilities|null $kinds,
        public readonly Probabilities|null $edges,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("NameOdds", $v, ["start", "end", "kinds", "edges"], ["start", "end", "kinds", "edges"]);
        return new self(
            Read::field($v, "start", "uint"),
            Read::field($v, "end", "uint"),
            Read::field($v, "kinds", "{probability}|null"),
            Read::field($v, "edges", "{probability}|null"),
        );
    }
}

final class Span extends Carrier
{
    public function __construct(
        public readonly int $start,
        public readonly int $end,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Span", $v, ["start", "end"], ["start", "end"]);
        return new self(
            Read::field($v, "start", "uint"),
            Read::field($v, "end", "uint"),
        );
    }
}

final class PairOdds extends Carrier
{
    public function __construct(
        public readonly string $relation,
        public readonly Span $source,
        public readonly Span $target,
        public readonly float $probability,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("PairOdds", $v, ["relation", "source", "target", "probability"], ["relation", "source", "target", "probability"]);
        return new self(
            Read::field($v, "relation", "str"),
            Read::field($v, "source", "Span"),
            Read::field($v, "target", "Span"),
            Read::field($v, "probability", "probability"),
        );
    }
}

final class RecognitionAnswer extends Carrier
{
    public function __construct(
        /** @var list<PieceOdds> */
        public readonly array $pieces,
        /** @var list<NameOdds> */
        public readonly array $names,
        /** @var list<PairOdds> */
        public readonly array $pairs,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RecognitionAnswer", $v, ["pieces", "names", "pairs"], ["pieces", "names", "pairs"]);
        return new self(
            Read::field($v, "pieces", "[PieceOdds]"),
            Read::field($v, "names", "[NameOdds]"),
            Read::field($v, "pairs", "[PairOdds]"),
        );
    }
}

<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class DecideQuestion extends Carrier implements AtomicQuestion
{
    public function __construct(
        public readonly string $verb,
        public readonly Authored $text,
        /** @var Optional<Authored> */
        public readonly Optional $true = new Optional(),
        /** @var Optional<Authored> */
        public readonly Optional $false = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("DecideQuestion", $v, ["verb", "text", "true", "false"], ["verb", "text"]);
        return new self(
            Read::field($v, "verb", "=decide"),
            Read::field($v, "text", "text"),
            Read::optional($v, "true", "description"),
            Read::optional($v, "false", "description"),
        );
    }
}

final class ChooseQuestion extends Carrier implements AtomicQuestion
{
    public function __construct(
        public readonly string $verb,
        public readonly Authored $text,
        /** @var list<string> */
        public readonly array $options,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ChooseQuestion", $v, ["verb", "text", "options"], ["verb", "text", "options"]);
        return new self(
            Read::field($v, "verb", "=choose"),
            Read::field($v, "text", "text"),
            Read::field($v, "options", "[str]"),
        );
    }
}

final class TagQuestion extends Carrier implements AtomicQuestion
{
    public function __construct(
        public readonly string $verb,
        public readonly Authored $text,
        /** @var list<string> */
        public readonly array $labels,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("TagQuestion", $v, ["verb", "text", "labels"], ["verb", "text", "labels"]);
        return new self(
            Read::field($v, "verb", "=tag"),
            Read::field($v, "text", "text"),
            Read::field($v, "labels", "[str]"),
        );
    }
}

final class ScoreQuestion extends Carrier implements AtomicQuestion
{
    public function __construct(
        public readonly string $verb,
        public readonly Authored $text,
        /** @var list<string> */
        public readonly array $levels,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ScoreQuestion", $v, ["verb", "text", "levels"], ["verb", "text", "levels"]);
        return new self(
            Read::field($v, "verb", "=score"),
            Read::field($v, "text", "text"),
            Read::field($v, "levels", "[str]"),
        );
    }
}

final class FindQuestion extends Carrier implements AtomicQuestion
{
    public function __construct(
        public readonly string $verb,
        public readonly Authored $text,
        public readonly bool $none,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FindQuestion", $v, ["verb", "text", "none"], ["verb", "text", "none"]);
        return new self(
            Read::field($v, "verb", "=find"),
            Read::field($v, "text", "text"),
            Read::field($v, "none", "bool"),
        );
    }
}

final class RelationRule extends Carrier
{
    public function __construct(
        public readonly string $name,
        public readonly string $source,
        public readonly string $target,
        public readonly string $reads,
        public readonly bool $either,
        /** @var Optional<bool> */
        public readonly Optional $single = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelationRule", $v, ["name", "source", "target", "reads", "either", "single"], ["name", "source", "target", "reads", "either"]);
        return new self(
            Read::field($v, "name", "str"),
            Read::field($v, "source", "str"),
            Read::field($v, "target", "str"),
            Read::field($v, "reads", "str"),
            Read::field($v, "either", "bool"),
            Read::optional($v, "single", "bool"),
        );
    }
}

final class RelateFields extends Carrier
{
    public function __construct(
        public readonly string $name,
        public readonly string $kind,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelateFields", $v, ["name", "kind"], ["name", "kind"]);
        return new self(
            Read::field($v, "name", "str"),
            Read::field($v, "kind", "str"),
        );
    }
}

final class RelateQuestion extends Carrier
{
    public function __construct(
        public readonly string $verb,
        public readonly RelateFields|null $fields,
        /** @var list<RelationRule> */
        public readonly array $relations,
        public readonly Threshold $threshold,
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelateQuestion", $v, ["verb", "fields", "relations", "threshold", "profile"], ["verb", "fields", "relations", "threshold"]);
        return new self(
            Read::field($v, "verb", "=relate"),
            Read::field($v, "fields", "RelateFields|null"),
            Read::field($v, "relations", "[RelationRule]"),
            Read::field($v, "threshold", "threshold"),
            Read::optional($v, "profile", "str"),
        );
    }
}

final class RecognizeQuestion extends Carrier
{
    public function __construct(
        public readonly string $verb,
        public readonly Descriptions $kinds,
        public readonly Threshold $threshold,
        public readonly Threshold $relation_threshold,
        /** @var Optional<list<RelationRule>> */
        public readonly Optional $relations = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $instructions = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $entity_definition = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RecognizeQuestion", $v, ["verb", "kinds", "relations", "threshold", "relation_threshold", "on", "profile", "instructions", "entity_definition"], ["verb", "kinds", "threshold", "relation_threshold"]);
        return new self(
            Read::field($v, "verb", "=recognize"),
            Read::field($v, "kinds", "{description}"),
            Read::field($v, "threshold", "threshold"),
            Read::field($v, "relation_threshold", "threshold"),
            Read::optional($v, "relations", "[RelationRule]"),
            Read::optional($v, "on", "str|[str]"),
            Read::optional($v, "profile", "str"),
            Read::optional($v, "instructions", "str"),
            Read::optional($v, "entity_definition", "str"),
        );
    }
}

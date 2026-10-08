<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class RecognitionPlan extends Carrier
{
    public function __construct(
        /** @var Optional<Labels> */
        public readonly Optional $kinds = new Optional(),
        /** @var Optional<list<PlanRule>> */
        public readonly Optional $relations = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $instructions = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $entity_definition = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RecognitionPlan", $v, ["kinds", "relations", "instructions", "entity_definition"], []);
        return new self(
            Read::optional($v, "kinds", "Labels"),
            Read::optional($v, "relations", "[PlanRule]"),
            Read::optional($v, "instructions", "str"),
            Read::optional($v, "entity_definition", "str"),
        );
    }
}

final class PlanRule extends Carrier
{
    public function __construct(
        public readonly string $name,
        /** @var Optional<string> */
        public readonly Optional $source = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $target = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $reads = new Optional(),
        /** @var Optional<bool> */
        public readonly Optional $either = new Optional(),
        /** @var Optional<bool> */
        public readonly Optional $single = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("PlanRule", $v, ["name", "source", "target", "reads", "either", "single"], ["name"]);
        return new self(
            Read::field($v, "name", "str"),
            Read::optional($v, "source", "str"),
            Read::optional($v, "target", "str"),
            Read::optional($v, "reads", "str"),
            Read::optional($v, "either", "bool"),
            Read::optional($v, "single", "bool"),
        );
    }
}

final class RecognitionSpec extends Carrier
{
    public function __construct(
        public readonly int $version,
        public readonly RecognitionPlan $recognize,
        /** @var Optional<float> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<float> */
        public readonly Optional $relation_threshold = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RecognitionSpec", $v, ["version", "recognize", "threshold", "relation_threshold", "model", "profile", "on"], ["version", "recognize"]);
        return new self(
            Read::field($v, "version", "one"),
            Read::field($v, "recognize", "RecognitionPlan"),
            Read::optional($v, "threshold", "probability"),
            Read::optional($v, "relation_threshold", "probability"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "profile", "str"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class RelationPlan extends Carrier
{
    public function __construct(
        /** @var list<PlanRule> */
        public readonly array $relations,
        /** @var Optional<RelateFields> */
        public readonly Optional $fields = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelationPlan", $v, ["relations", "fields"], ["relations"]);
        return new self(
            Read::field($v, "relations", "[PlanRule]"),
            Read::optional($v, "fields", "RelateFields"),
        );
    }
}

final class RelationSpec extends Carrier
{
    public function __construct(
        public readonly int $version,
        public readonly RelationPlan $relate,
        /** @var Optional<float> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelationSpec", $v, ["version", "relate", "threshold", "model", "profile"], ["version", "relate"]);
        return new self(
            Read::field($v, "version", "one"),
            Read::field($v, "relate", "RelationPlan"),
            Read::optional($v, "threshold", "probability"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "profile", "str"),
        );
    }
}

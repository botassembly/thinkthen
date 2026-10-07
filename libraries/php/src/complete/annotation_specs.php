<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class DecideMember extends Carrier implements AnnotationSpec
{
    public function __construct(
        public readonly Authored $decide,
        /** @var Optional<Authored> */
        public readonly Optional $true = new Optional(),
        /** @var Optional<Authored> */
        public readonly Optional $false = new Optional(),
        /** @var Optional<Threshold> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("DecideMember", $v, ["decide", "true", "false", "threshold", "on"], ["decide"]);
        return new self(
            Read::field($v, "decide", "text"),
            Read::optional($v, "true", "description"),
            Read::optional($v, "false", "description"),
            Read::optional($v, "threshold", "threshold"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class ChooseMember extends Carrier implements AnnotationSpec
{
    public function __construct(
        public readonly Authored $choose,
        public readonly Labels $options,
        /** @var Optional<float> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ChooseMember", $v, ["choose", "options", "threshold", "on"], ["choose", "options"]);
        return new self(
            Read::field($v, "choose", "text"),
            Read::field($v, "options", "Labels"),
            Read::optional($v, "threshold", "probability"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class TagMember extends Carrier implements AnnotationSpec
{
    public function __construct(
        public readonly Authored $tag,
        public readonly Labels $labels,
        /** @var Optional<float> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("TagMember", $v, ["tag", "labels", "threshold", "on"], ["tag", "labels"]);
        return new self(
            Read::field($v, "tag", "text"),
            Read::field($v, "labels", "Labels"),
            Read::optional($v, "threshold", "probability"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class ScoreMember extends Carrier implements AnnotationSpec
{
    public function __construct(
        public readonly Authored $score,
        public readonly Labels $levels,
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ScoreMember", $v, ["score", "levels", "on"], ["score", "levels"]);
        return new self(
            Read::field($v, "score", "text"),
            Read::field($v, "levels", "Labels"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

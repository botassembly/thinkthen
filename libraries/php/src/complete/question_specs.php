<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class DecideSpec extends Carrier implements QuestionSpec, RankSpec
{
    public function __construct(
        public readonly Authored $decide,
        /** @var Optional<Authored> */
        public readonly Optional $true = new Optional(),
        /** @var Optional<Authored> */
        public readonly Optional $false = new Optional(),
        /** @var Optional<Threshold> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
        /** @var Optional<Batch> */
        public readonly Optional $batch = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("DecideSpec", $v, ["decide", "true", "false", "threshold", "model", "profile", "batch", "on"], ["decide"]);
        return new self(
            Read::field($v, "decide", "text"),
            Read::optional($v, "true", "description"),
            Read::optional($v, "false", "description"),
            Read::optional($v, "threshold", "threshold"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "profile", "str"),
            Read::optional($v, "batch", "batch"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class ChooseSpec extends Carrier implements QuestionSpec
{
    public function __construct(
        public readonly Authored $choose,
        public readonly Labels $options,
        /** @var Optional<float> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
        /** @var Optional<Batch> */
        public readonly Optional $batch = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ChooseSpec", $v, ["choose", "options", "threshold", "model", "profile", "batch", "on"], ["choose", "options"]);
        return new self(
            Read::field($v, "choose", "text"),
            Read::field($v, "options", "Labels"),
            Read::optional($v, "threshold", "probability"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "profile", "str"),
            Read::optional($v, "batch", "batch"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class TagSpec extends Carrier implements QuestionSpec
{
    public function __construct(
        public readonly Authored $tag,
        public readonly Labels $labels,
        /** @var Optional<float> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
        /** @var Optional<Batch> */
        public readonly Optional $batch = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("TagSpec", $v, ["tag", "labels", "threshold", "model", "profile", "batch", "on"], ["tag", "labels"]);
        return new self(
            Read::field($v, "tag", "text"),
            Read::field($v, "labels", "Labels"),
            Read::optional($v, "threshold", "probability"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "profile", "str"),
            Read::optional($v, "batch", "batch"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class ScoreSpec extends Carrier implements QuestionSpec, RankSpec
{
    public function __construct(
        public readonly Authored $score,
        public readonly Labels $levels,
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
        /** @var Optional<Batch> */
        public readonly Optional $batch = new Optional(),
        /** @var Optional<Pointers> */
        public readonly Optional $on = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ScoreSpec", $v, ["score", "levels", "model", "profile", "batch", "on"], ["score", "levels"]);
        return new self(
            Read::field($v, "score", "text"),
            Read::field($v, "levels", "Labels"),
            Read::optional($v, "model", "str"),
            Read::optional($v, "profile", "str"),
            Read::optional($v, "batch", "batch"),
            Read::optional($v, "on", "str|[str]"),
        );
    }
}

final class FindSpec extends Carrier
{
    public function __construct(
        public readonly Authored $find,
        /** @var Optional<bool> */
        public readonly Optional $none = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $model = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FindSpec", $v, ["find", "none", "model"], ["find"]);
        return new self(
            Read::field($v, "find", "text"),
            Read::optional($v, "none", "bool"),
            Read::optional($v, "model", "str"),
        );
    }
}

final class QuestionFile extends Carrier implements RankSpec
{
    public function __construct(
        public readonly string $path,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("QuestionFile", $v, ["path"], ["path"]);
        return new self(
            Read::field($v, "path", "str"),
        );
    }
}

final class QuestionSet extends Carrier implements RankSpec
{
    public function __construct(
        public readonly int $version,
        public readonly QuestionMembers $questions,
        /** @var Optional<Batch> */
        public readonly Optional $batch = new Optional(),
        /** @var Optional<Threshold> */
        public readonly Optional $threshold = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $profile = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("QuestionSet", $v, ["version", "questions", "batch", "threshold", "profile"], ["version", "questions"]);
        return new self(
            Read::field($v, "version", "one"),
            Read::field($v, "questions", "{AnnotationSpec}"),
            Read::optional($v, "batch", "batch"),
            Read::optional($v, "threshold", "threshold"),
            Read::optional($v, "profile", "str"),
        );
    }
}

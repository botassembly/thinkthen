<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class DecideResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly DecisionValue $value,
        public readonly DecideQuestion $question,
        public readonly YesNo $answer,
        public readonly Threshold $threshold,
        /** @var Optional<mixed> */
        public readonly Optional $input = new Optional(),
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $input_file = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("DecideResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer", "threshold", "input", "position", "input_file"], ["schema", "answer_id", "meta", "value", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            readDecision($v->value, $v->question),
            Read::field($v, "question", "DecideQuestion"),
            Read::field($v, "answer", "YesNo"),
            Read::field($v, "threshold", "threshold"),
            Read::optional($v, "input", "json"),
            Read::optional($v, "position", "Position"),
            Read::optional($v, "input_file", "str"),
        );
    }
}

final class ChooseResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly string|null $value,
        public readonly ChooseQuestion $question,
        public readonly Choice $answer,
        public readonly Threshold $threshold,
        /** @var Optional<mixed> */
        public readonly Optional $input = new Optional(),
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $input_file = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ChooseResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer", "threshold", "input", "position", "input_file"], ["schema", "answer_id", "meta", "value", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "str|null"),
            Read::field($v, "question", "ChooseQuestion"),
            Read::field($v, "answer", "Choice"),
            Read::field($v, "threshold", "threshold"),
            Read::optional($v, "input", "json"),
            Read::optional($v, "position", "Position"),
            Read::optional($v, "input_file", "str"),
        );
    }
}

final class TagResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        /** @var list<string> */
        public readonly array $value,
        public readonly TagQuestion $question,
        public readonly Tags $answer,
        public readonly Threshold $threshold,
        /** @var Optional<mixed> */
        public readonly Optional $input = new Optional(),
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $input_file = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("TagResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer", "threshold", "input", "position", "input_file"], ["schema", "answer_id", "meta", "value", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "[str]"),
            Read::field($v, "question", "TagQuestion"),
            Read::field($v, "answer", "Tags"),
            Read::field($v, "threshold", "threshold"),
            Read::optional($v, "input", "json"),
            Read::optional($v, "position", "Position"),
            Read::optional($v, "input_file", "str"),
        );
    }
}

final class ScoreResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly float $value,
        public readonly ScoreQuestion $question,
        public readonly Score $answer,
        public readonly null $threshold,
        /** @var Optional<mixed> */
        public readonly Optional $input = new Optional(),
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
        /** @var Optional<string> */
        public readonly Optional $input_file = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("ScoreResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer", "threshold", "input", "position", "input_file"], ["schema", "answer_id", "meta", "value", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "number"),
            Read::field($v, "question", "ScoreQuestion"),
            Read::field($v, "answer", "Score"),
            Read::field($v, "threshold", "null"),
            Read::optional($v, "input", "json"),
            Read::optional($v, "position", "Position"),
            Read::optional($v, "input_file", "str"),
        );
    }
}

final class FilterResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly bool $value,
        public readonly mixed $input,
        public readonly DecideQuestion $question,
        public readonly YesNo $answer,
        public readonly Threshold $threshold,
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FilterResult", $v, ["schema", "answer_id", "meta", "value", "input", "question", "answer", "threshold", "position"], ["schema", "answer_id", "meta", "value", "input", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "bool"),
            Read::field($v, "input", "json"),
            Read::field($v, "question", "DecideQuestion"),
            Read::field($v, "answer", "YesNo"),
            Read::field($v, "threshold", "threshold"),
            Read::optional($v, "position", "Position"),
        );
    }
}

final class RankResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly int $value,
        public readonly mixed $input,
        public readonly AtomicQuestion $question,
        public readonly AtomicAnswer $answer,
        public readonly null $threshold,
        /** @var Optional<string> */
        public readonly Optional $question_name = new Optional(),
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RankResult", $v, ["schema", "answer_id", "meta", "value", "input", "question", "answer", "threshold", "question_name", "position"], ["schema", "answer_id", "meta", "value", "input", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "positive"),
            Read::field($v, "input", "json"),
            Read::field($v, "question", "AtomicQuestion"),
            Read::field($v, "answer", "AtomicAnswer"),
            Read::field($v, "threshold", "null"),
            Read::optional($v, "question_name", "str"),
            Read::optional($v, "position", "Position"),
        );
    }
}

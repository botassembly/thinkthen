<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class FindResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly mixed $value,
        public readonly FindQuestion $question,
        public readonly FindAnswer $answer,
        public readonly null $threshold,
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FindResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer", "threshold", "position"], ["schema", "answer_id", "meta", "value", "question", "answer", "threshold"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "json"),
            Read::field($v, "question", "FindQuestion"),
            Read::field($v, "answer", "FindAnswer"),
            Read::field($v, "threshold", "null"),
            Read::optional($v, "position", "Position"),
        );
    }
}

final class AnnotateResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly mixed $input,
        public readonly AnnotationValues $value,
        public readonly AnnotationMembers $answers,
        /** @var Optional<Position> */
        public readonly Optional $position = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("AnnotateResult", $v, ["schema", "answer_id", "meta", "input", "value", "answers", "position"], ["schema", "answer_id", "meta", "input", "value", "answers"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "input", "json"),
            readAnnotationValues($v->value, $v->answers),
            Read::field($v, "answers", "{AnnotationEntry}"),
            Read::optional($v, "position", "Position"),
        );
    }
}

final class RecognizeResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        public readonly Recognition $value,
        public readonly RecognizeQuestion $question,
        public readonly RecognitionAnswer $answer,
        /** @var Optional<mixed> */
        public readonly Optional $input = new Optional(),
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RecognizeResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer", "input"], ["schema", "answer_id", "meta", "value", "question", "answer"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "Recognition"),
            Read::field($v, "question", "RecognizeQuestion"),
            Read::field($v, "answer", "RecognitionAnswer"),
            Read::optional($v, "input", "json"),
        );
    }
}

final class RelateResult extends Carrier implements Result
{
    public function __construct(
        public readonly string $schema,
        public readonly AnswerId $answer_id,
        public readonly Meta $meta,
        /** @var list<Edge> */
        public readonly array $value,
        public readonly RelateQuestion $question,
        public readonly RelationAnswer $answer,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelateResult", $v, ["schema", "answer_id", "meta", "value", "question", "answer"], ["schema", "answer_id", "meta", "value", "question", "answer"]);
        return new self(
            Read::field($v, "schema", "=thinkthen.result/2"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "meta", "Meta"),
            Read::field($v, "value", "[Edge]"),
            Read::field($v, "question", "RelateQuestion"),
            Read::field($v, "answer", "RelationAnswer"),
        );
    }
}

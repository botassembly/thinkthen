<?php
declare(strict_types=1);
namespace ThinkThen\Complete;

final class Failure extends Carrier
{
    public function __construct(
        public readonly string $kind,
        public readonly CauseKind $cause,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("Failure", $v, ["kind", "cause"], ["kind", "cause"]);
        return new self(
            Read::field($v, "kind", "=backend"),
            Read::field($v, "cause", "cause"),
        );
    }
}

final class FailedField extends Carrier
{
    public function __construct(
        public readonly Failure $failed,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("FailedField", $v, ["failed"], ["failed"]);
        return new self(
            Read::field($v, "failed", "Failure"),
        );
    }
}

final class AnnotationSuccess extends Carrier implements AnnotationEntry
{
    public function __construct(
        public readonly AnswerId $answer_id,
        public readonly ActionValue $value,
        public readonly AtomicQuestion $question,
        public readonly AtomicAnswer $answer,
        public readonly Threshold $threshold,
        public readonly Digest $request,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("AnnotationSuccess", $v, ["answer_id", "value", "question", "answer", "threshold", "request"], ["answer_id", "value", "question", "answer", "threshold", "request"]);
        return new self(
            Read::field($v, "answer_id", "AnswerId"),
            readAction($v->value, $v->question),
            Read::field($v, "question", "AtomicQuestion"),
            Read::field($v, "answer", "AtomicAnswer"),
            Read::field($v, "threshold", "threshold"),
            Read::field($v, "request", "Digest"),
        );
    }
}

final class AnnotationFailure extends Carrier implements AnnotationEntry
{
    public function __construct(
        public readonly FailureId $failure_id,
        public readonly AtomicQuestion $question,
        public readonly Failure $failure,
        public readonly Digest $request,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("AnnotationFailure", $v, ["failure_id", "question", "failure", "request"], ["failure_id", "question", "failure", "request"]);
        return new self(
            Read::field($v, "failure_id", "FailureId"),
            Read::field($v, "question", "AtomicQuestion"),
            Read::field($v, "failure", "Failure"),
            Read::field($v, "request", "Digest"),
        );
    }
}

final class RelationSuccess extends Carrier implements RelationEntry
{
    public function __construct(
        public readonly string $relation,
        public readonly string $reads,
        public readonly MethodKind $method,
        public readonly DirectionKind $direction,
        public readonly Endpoint $source,
        public readonly Endpoint|null $target,
        public readonly Digest $request,
        public readonly AnswerId $answer_id,
        public readonly float $probability,
        public readonly bool $accepted,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelationSuccess", $v, ["relation", "reads", "method", "direction", "source", "target", "request", "answer_id", "probability", "accepted"], ["relation", "reads", "method", "direction", "source", "target", "request", "answer_id", "probability", "accepted"]);
        return new self(
            Read::field($v, "relation", "str"),
            Read::field($v, "reads", "str"),
            Read::field($v, "method", "method"),
            Read::field($v, "direction", "direction"),
            Read::field($v, "source", "Endpoint"),
            Read::field($v, "target", "Endpoint|null"),
            Read::field($v, "request", "Digest"),
            Read::field($v, "answer_id", "AnswerId"),
            Read::field($v, "probability", "probability"),
            Read::field($v, "accepted", "bool"),
        );
    }
}

final class RelationFailure extends Carrier implements RelationEntry
{
    public function __construct(
        public readonly string $relation,
        public readonly string $reads,
        public readonly MethodKind $method,
        public readonly DirectionKind $direction,
        public readonly Endpoint $source,
        public readonly Endpoint|null $target,
        public readonly Digest $request,
        public readonly FailureId $failure_id,
        public readonly Failure $failure,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelationFailure", $v, ["relation", "reads", "method", "direction", "source", "target", "request", "failure_id", "failure"], ["relation", "reads", "method", "direction", "source", "target", "request", "failure_id", "failure"]);
        return new self(
            Read::field($v, "relation", "str"),
            Read::field($v, "reads", "str"),
            Read::field($v, "method", "method"),
            Read::field($v, "direction", "direction"),
            Read::field($v, "source", "Endpoint"),
            Read::field($v, "target", "Endpoint|null"),
            Read::field($v, "request", "Digest"),
            Read::field($v, "failure_id", "FailureId"),
            Read::field($v, "failure", "Failure"),
        );
    }
}

final class RelationAnswer extends Carrier
{
    public function __construct(
        /** @var list<RelationEntry> */
        public readonly array $questions,
    ) {}
    public static function fromJson(mixed $value): self
    {
        $v = Read::object($value);
        Read::shape("RelationAnswer", $v, ["questions"], ["questions"]);
        return new self(
            Read::field($v, "questions", "[RelationEntry]"),
        );
    }
}

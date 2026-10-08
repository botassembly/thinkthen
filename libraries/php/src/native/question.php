<?php
declare(strict_types=1);
namespace ThinkThen\Native;

enum FunctionKind: int { case DECIDE=1; case CHOOSE=2; case TAG=3; case SCORE=4; case FILTER=5; case RANK=6; case FIND=7; case ANNOTATE=8; case RECOGNIZE=9; case RELATE=10; }
final class Rule
{
    private function __construct(public readonly int $kind,public readonly float $low=0,public readonly float $high=0) {}
    public static function missing(): self { return new self(0); }
    public static function none(): self { return new self(1); }
    public static function cut(float $value): self { return new self(2,$value); }
    public static function band(float $low,float $high): self { return new self(3,$low,$high); }
}
final class Member
{
    public function __construct(public readonly string $name,public readonly Question $question) {}
}
final class Relation
{
    public function __construct(public readonly string $name,public readonly string $source,public readonly string $target,
        public readonly ?string $reads=null,public readonly bool $either=false,public readonly bool $single=false) {}
}
enum PropertyKind: int { case STRING=1; case NUMBER=2; case BOOLEAN=3; case STRING_LIST=4; }
final class Property
{
    public function __construct(public readonly string $name,public readonly PropertyKind $kind) {}
}
final class Declaration
{
    /** @param list<Property> $properties @param list<string> $required */
    private function __construct(public readonly int $kind,public readonly array $properties=[],public readonly array $required=[]) {}
    public static function absent(): self { return new self(0); }
    public static function string(): self { return new self(1); }
    public static function object(array $properties,array $required=[]): self { return new self(2,$properties,$required); }
}
final class Author
{
    /** Exact unsigned wording version, as decimal text. */
    public function __construct(public readonly ?string $name=null,public readonly ?string $wordingVersion=null,
        public readonly ?Declaration $itemSchema=null,public readonly ?Declaration $contextSchema=null) {}
}
/** Counted typed question descriptor; native alone validates semantics. */
final class QuestionSpec
{
    /** @param list<Choice> $choices @param list<string> $on @param list<Member> $members
     * @param list<Choice> $kinds @param list<Relation> $relations */
    public function __construct(public readonly FunctionKind $kind,public readonly ?Content $text=null,
        public readonly ?Content $yes=null,public readonly ?Content $no=null,public readonly array $choices=[],
        public readonly ?Rule $threshold=null,public readonly ?Rule $relationThreshold=null,
        public readonly ?string $model=null,public readonly ?string $profile=null,public readonly ?int $batch=null,
        public readonly bool $batchMax=false,public readonly bool $none=false,public readonly array $on=[],
        public readonly array $members=[],public readonly array $kinds=[],public readonly array $relations=[],
        public readonly ?string $namePointer=null,public readonly ?string $kindPointer=null,public readonly ?Author $author=null,public readonly ?string $instructions=null,public readonly ?string $entityDefinition=null) {}
}

<?php
declare(strict_types=1);
namespace ThinkThen\Native;

/** Explicit text or arbitrary caller JSON; neither guesses paths or media. */
final class Content
{
    private function __construct(public readonly int $kind, public readonly string $bytes) {}
    public static function text(string $text): self { return new self(1,$text); }
    public static function json(mixed $value): self { return new self(2,json_encode($value,JSON_THROW_ON_ERROR|JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES)); }
}
enum LoaderRole: int { case ATOMIC=1; case SET=2; case DYNAMIC_CHOOSE=3; case RECOGNIZE=4; case RELATE=5; case RANK=6; case RANK_SET=7; case FIND=8; }
final class Question
{
    private function __construct(public readonly string $method, public readonly LoaderRole $role, public readonly string $value, public readonly ?Content $text=null, public readonly bool $none=false, public readonly ?string $name=null, public readonly ?int $wordingVersion=null, public readonly ?QuestionSpec $spec=null) {}
    public static function spec(QuestionSpec $spec): self { return new self("spec",LoaderRole::ATOMIC,"",spec:$spec); }
    public static function find(Content $text,bool $none=false,?string $name=null,?int $wordingVersion=null): self { return new self("new",LoaderRole::FIND,"",$text,$none,$name,$wordingVersion); }
    /** Saved grammar uses the native parser once, under an explicit role. */
    public static function saved(LoaderRole $role, string $json): self { return new self('parse',$role,$json); }
    public static function file(string $path): self { return new self('load',LoaderRole::ATOMIC,$path); }
    public static function named(LoaderRole $role,string $name): self { return new self('load_named',$role,$name); }
    public static function reference(LoaderRole $role,string $reference): self { return new self('load_reference',$role,$reference); }
}
final class Choice
{
    public function __construct(public readonly string $name, public readonly ?Content $description=null, public readonly ?float $weight=null) {}
}
final class Image
{
    public function __construct(public readonly string $bytes,public readonly int $media,public readonly ?string $filename=null) {}
}
final class Record
{
    /** @param list<Choice> $options @param list<Image> $images */
    public function __construct(public readonly ?Content $original,public readonly ?Content $context=null,
                                public readonly array $options=[],public readonly array $images=[]) {}
}
interface Source {}
final class Records implements Source
{
    /** @param list<Record> $records */
    public function __construct(public readonly array $records) {}
}
enum FileUnit: int { case LINE=1; case WINDOW=2; case FILE=3; case IMAGE=4; case JSONL=5; }
final class Files implements Source
{
    /** @param list<string> $paths */
    public function __construct(public readonly array $paths,public readonly FileUnit $unit=FileUnit::LINE,
                                public readonly int $window=0,public readonly bool $imageReader=false) {}
}
final class Controls
{
    public function __construct(public readonly int $deadlineMs=-1,public readonly ?Cancellation $cancel=null,
        public readonly ?Content $context=null,public readonly ?int $batch=null,
        public readonly bool $batchMax=false,public readonly bool $attempts=false) {}
}

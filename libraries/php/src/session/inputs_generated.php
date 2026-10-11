<?php
// Generated from the shared Rust request graph; do not edit.
declare(strict_types=1);
namespace ThinkThen\Inputs;
final class InputRequest extends Document
{
    public function __construct(string $schema, mixed $call, array $fields = [])
    {
        parent::__construct(['schema' => $schema, 'call' => $call, ...$fields]);
    }
}
final class InputRequestCallAnnotate extends Document
{
    public const TAGS = ['function' => 'annotate'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallChoose extends Document
{
    public const TAGS = ['function' => 'choose'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallDecide extends Document
{
    public const TAGS = ['function' => 'decide'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallFilter extends Document
{
    public const TAGS = ['function' => 'filter'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallFind extends Document
{
    public const TAGS = ['function' => 'find'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallRank extends Document
{
    public const TAGS = ['function' => 'rank'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallRecognize extends Document
{
    public const TAGS = ['function' => 'recognize'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallRelate extends Document
{
    public const TAGS = ['function' => 'relate'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallScore extends Document
{
    public const TAGS = ['function' => 'score'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestCallTag extends Document
{
    public const TAGS = ['function' => 'tag'];
    public function __construct(mixed $question, mixed $input, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'question' => $question, 'input' => $input, ...$fields]);
    }
}
final class InputRequestInputEntities extends Document
{
    public const TAGS = ['kind' => 'entities'];
    public function __construct(array $items, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'items' => $items, ...$fields]);
    }
}
final class InputRequestInputFeed extends Document
{
    public const TAGS = ['kind' => 'feed'];
    public function __construct(string $name, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'name' => $name, ...$fields]);
    }
}
final class InputRequestInputJson extends Document
{
    public const TAGS = ['kind' => 'json'];
    public function __construct(mixed $value, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'value' => $value, ...$fields]);
    }
}
final class InputRequestInputRecords extends Document
{
    public const TAGS = ['kind' => 'records'];
    public function __construct(array $items, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'items' => $items, ...$fields]);
    }
}
final class InputRequestInputSource extends Document
{
    public const TAGS = ['kind' => 'source'];
    public function __construct(mixed $source, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'source' => $source, ...$fields]);
    }
}
final class InputRequestInputText extends Document
{
    public const TAGS = ['kind' => 'text'];
    public function __construct(string $text, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'text' => $text, ...$fields]);
    }
}
final class InputRequestInputUnits extends Document
{
    public const TAGS = ['kind' => 'units'];
    public function __construct(array $items, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'items' => $items, ...$fields]);
    }
}
final class InputRequestItem extends Document
{
    public function __construct(array $fields = [])
    {
        parent::__construct($fields);
    }
}
final class InputRequestOptions extends Document
{
    public function __construct(array $fields = [])
    {
        parent::__construct($fields);
    }
}
final class InputRequestQuestion extends Document
{
}
final class InputRequestQuestionDefinition extends Document
{
    public const TAGS = ['kind' => 'definition'];
    public function __construct(mixed $value, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'value' => $value, ...$fields]);
    }
}
final class InputRequestQuestionFile extends Document
{
    public const TAGS = ['kind' => 'file'];
    public function __construct(string $path, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'path' => $path, ...$fields]);
    }
}
final class InputRequestQuestionName extends Document
{
    public const TAGS = ['kind' => 'name'];
    public function __construct(string $name, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'name' => $name, ...$fields]);
    }
}
final class InputRequestQuestionReference extends Document
{
    public const TAGS = ['kind' => 'reference'];
    public function __construct(string $reference, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'reference' => $reference, ...$fields]);
    }
}
final class InputRequestQuestionText extends Document
{
    public const TAGS = ['kind' => 'text'];
    public function __construct(string $text, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'text' => $text, ...$fields]);
    }
}
final class InputRequestReader extends Document
{
    public function __construct(array $fields = [])
    {
        parent::__construct($fields);
    }
}
final class InputRequestReaderFailureInvalidInput extends Document
{
    public const TAGS = ['kind' => 'invalid_input'];
    public function __construct(array $fields = [])
    {
        parent::__construct([...self::TAGS, ...$fields]);
    }
}
final class InputRequestReaderFailureIo extends Document
{
    public const TAGS = ['kind' => 'io'];
    public function __construct(array $fields = [])
    {
        parent::__construct([...self::TAGS, ...$fields]);
    }
}
final class InputRequestReaderFailureUtf8 extends Document
{
    public const TAGS = ['kind' => 'utf8'];
    public function __construct(array $fields = [])
    {
        parent::__construct([...self::TAGS, ...$fields]);
    }
}
final class InputRequestSessionDescriptor extends Document
{
    public function __construct(mixed $item, array $fields = [])
    {
        parent::__construct(['item' => $item, ...$fields]);
    }
}
final class InputRequestSource extends Document
{
    public function __construct(array $paths, array $fields = [])
    {
        parent::__construct(['paths' => $paths, ...$fields]);
    }
}

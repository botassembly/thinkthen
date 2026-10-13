<?php
// Generated from the shared Rust request graph; do not edit.
declare(strict_types=1);
namespace ThinkThen\Inputs;
final class InputAuthoredRelation extends Document
{
    public function __construct(string $name, array $fields = [])
    {
        parent::__construct(['name' => $name, ...$fields]);
    }
}
final class InputRecognizeDefinition extends Document
{
    public const TAGS = ['version' => 1];
    public function __construct(mixed $recognize, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'recognize' => $recognize, ...$fields]);
    }
}
final class InputRecognizeQuestion extends Document
{
    public function __construct(array $fields = [])
    {
        parent::__construct($fields);
    }
}
final class InputRelateQuestion extends Document
{
    public function __construct(array $relations, array $fields = [])
    {
        parent::__construct(['relations' => $relations, ...$fields]);
    }
}
final class InputRequestDefinitionAuthoredRelate extends Document
{
    public const TAGS = ['version' => 1];
    public function __construct(mixed $relate, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'relate' => $relate, ...$fields]);
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
final class InputRequestReaderFailureProducer extends Document
{
    public const TAGS = ['kind' => 'producer'];
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
namespace ThinkThen;
trait NamedCalls
{
    public function decide(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('decide', $question, $input, $options, $cancel); }
    public function choose(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('choose', $question, $input, $options, $cancel); }
    public function tag(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('tag', $question, $input, $options, $cancel); }
    public function score(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('score', $question, $input, $options, $cancel); }
    public function filter(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('filter', $question, $input, $options, $cancel); }
    public function rank(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('rank', $question, $input, $options, $cancel); }
    public function find(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('find', $question, $input, $options, $cancel); }
    public function annotate(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('annotate', $question, $input, $options, $cancel); }
    public function recognize(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('recognize', $question, $input, $options, $cancel); }
    public function relate(string|array|\stdClass|\ThinkThen\Inputs\Document|QuestionInput $question, mixed $input, array $options = [], ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('relate', $question, $input, $options, $cancel); }
}

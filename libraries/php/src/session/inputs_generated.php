<?php
// Generated from the shared Rust request graph; do not edit.
declare(strict_types=1);
namespace ThinkThen\Inputs;
enum Omitted { case Value; }
interface QuestionSource {}
interface QuestionDefinition {}
namespace ThinkThen\Inputs;
final class RelationRule extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly string $name, public readonly bool|Omitted $either = Omitted::Value, public readonly string|Omitted $reads = Omitted::Value, public readonly bool|Omitted $single = Omitted::Value, public readonly string|Omitted $source = Omitted::Value, public readonly string|Omitted $target = Omitted::Value)
    {
        parent::__construct([...($either === Omitted::Value ? [] : ['either' => $either]), 'name' => $name, ...($reads === Omitted::Value ? [] : ['reads' => $reads]), ...($single === Omitted::Value ? [] : ['single' => $single]), ...($source === Omitted::Value ? [] : ['source' => $source]), ...($target === Omitted::Value ? [] : ['target' => $target])]);
    }
}
namespace ThinkThen\Inputs;
final class Settings extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly string|Omitted $backend = Omitted::Value, public readonly string|Omitted $base_url = Omitted::Value, public readonly int|string|Omitted $batch = Omitted::Value, public readonly string|bool|Omitted $cache = Omitted::Value, public readonly int|null|Omitted $max_estimated_input_tokens_total = Omitted::Value, public readonly int|Omitted $max_request_bytes = Omitted::Value, public readonly int|null|Omitted $max_requests = Omitted::Value, public readonly int|null|Omitted $max_requests_total = Omitted::Value, public readonly int|Omitted $max_retries = Omitted::Value, public readonly string|Omitted $model = Omitted::Value, public readonly string|Omitted $profile = Omitted::Value, public readonly mixed $proxy = Omitted::Value, public readonly string|Omitted $record = Omitted::Value, public readonly bool|Omitted $refresh_cache = Omitted::Value, public readonly string|Omitted $replay = Omitted::Value, public readonly int|Omitted $throttle = Omitted::Value, public readonly int|Omitted $timeout = Omitted::Value, public readonly string|Omitted $usd_per_million_input = Omitted::Value, public readonly string|Omitted $usd_per_million_output = Omitted::Value)
    {
        parent::__construct([...($backend === Omitted::Value ? [] : ['backend' => $backend]), ...($base_url === Omitted::Value ? [] : ['base_url' => $base_url]), ...($batch === Omitted::Value ? [] : ['batch' => $batch]), ...($cache === Omitted::Value ? [] : ['cache' => $cache]), ...($max_estimated_input_tokens_total === Omitted::Value ? [] : ['max_estimated_input_tokens_total' => $max_estimated_input_tokens_total]), ...($max_request_bytes === Omitted::Value ? [] : ['max_request_bytes' => $max_request_bytes]), ...($max_requests === Omitted::Value ? [] : ['max_requests' => $max_requests]), ...($max_requests_total === Omitted::Value ? [] : ['max_requests_total' => $max_requests_total]), ...($max_retries === Omitted::Value ? [] : ['max_retries' => $max_retries]), ...($model === Omitted::Value ? [] : ['model' => $model]), ...($profile === Omitted::Value ? [] : ['profile' => $profile]), ...($proxy === Omitted::Value ? [] : ['proxy' => $proxy]), ...($record === Omitted::Value ? [] : ['record' => $record]), ...($refresh_cache === Omitted::Value ? [] : ['refresh_cache' => $refresh_cache]), ...($replay === Omitted::Value ? [] : ['replay' => $replay]), ...($throttle === Omitted::Value ? [] : ['throttle' => $throttle]), ...($timeout === Omitted::Value ? [] : ['timeout' => $timeout]), ...($usd_per_million_input === Omitted::Value ? [] : ['usd_per_million_input' => $usd_per_million_input]), ...($usd_per_million_output === Omitted::Value ? [] : ['usd_per_million_output' => $usd_per_million_output])]);
    }
}
namespace ThinkThen\Inputs;
final class RecognitionSeedSpan extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly int $end, public readonly int $start, public readonly string|Omitted $kind = Omitted::Value)
    {
        parent::__construct(['end' => $end, ...($kind === Omitted::Value ? [] : ['kind' => $kind]), 'start' => $start]);
    }
}
namespace ThinkThen\Inputs;
final class RecognitionStageContext extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly string|Omitted $boundary = Omitted::Value, public readonly string|Omitted $kind_edge = Omitted::Value, public readonly string|Omitted $relation = Omitted::Value)
    {
        parent::__construct([...($boundary === Omitted::Value ? [] : ['boundary' => $boundary]), ...($kind_edge === Omitted::Value ? [] : ['kind_edge' => $kind_edge]), ...($relation === Omitted::Value ? [] : ['relation' => $relation])]);
    }
}
namespace ThinkThen\Inputs;
final class RecognitionSpec extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly string|array|\stdClass|Omitted $entity_definition = Omitted::Value, public readonly string|array|\stdClass|Omitted $instructions = Omitted::Value, public readonly array|\stdClass|Omitted $kinds = Omitted::Value, public readonly string|Omitted $mode = Omitted::Value, public readonly array|Omitted $relations = Omitted::Value, public readonly int|Omitted $snippet_pieces = Omitted::Value, public readonly RecognitionStageContext|Omitted $stage_context = Omitted::Value)
    {
        parent::__construct([...($entity_definition === Omitted::Value ? [] : ['entity_definition' => $entity_definition]), ...($instructions === Omitted::Value ? [] : ['instructions' => $instructions]), ...($kinds === Omitted::Value ? [] : ['kinds' => $kinds]), ...($mode === Omitted::Value ? [] : ['mode' => $mode]), ...($relations === Omitted::Value ? [] : ['relations' => $relations]), ...($snippet_pieces === Omitted::Value ? [] : ['snippet_pieces' => $snippet_pieces]), ...($stage_context === Omitted::Value ? [] : ['stage_context' => $stage_context])]);
    }
}
namespace ThinkThen\Inputs;
final class RelationSpec extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly array $relations, public readonly array|\stdClass|Omitted $fields = Omitted::Value)
    {
        parent::__construct([...($fields === Omitted::Value ? [] : ['fields' => $fields]), 'relations' => $relations]);
    }
}
namespace ThinkThen\Inputs;
final class ChooseQuestion extends \ThinkThen\Internal\Document implements \ThinkThen\Inputs\QuestionDefinition
{
    public function __construct(public readonly string|array|\stdClass $choose, public readonly string|int|Omitted $batch = Omitted::Value, public readonly mixed $context_schema = Omitted::Value, public readonly mixed $item_schema = Omitted::Value, public readonly string|Omitted $model = Omitted::Value, public readonly string|Omitted $name = Omitted::Value, public readonly string|array|Omitted $on = Omitted::Value, public readonly array|\stdClass|Omitted $options = Omitted::Value, public readonly string|Omitted $profile = Omitted::Value, public readonly int|float|string|Omitted $threshold = Omitted::Value, public readonly int|Omitted $wording_version = Omitted::Value)
    {
        parent::__construct([...($batch === Omitted::Value ? [] : ['batch' => $batch]), 'choose' => $choose, ...($context_schema === Omitted::Value ? [] : ['context_schema' => $context_schema]), ...($item_schema === Omitted::Value ? [] : ['item_schema' => $item_schema]), ...($model === Omitted::Value ? [] : ['model' => $model]), ...($name === Omitted::Value ? [] : ['name' => $name]), ...($on === Omitted::Value ? [] : ['on' => $on]), ...($options === Omitted::Value ? [] : ['options' => $options]), ...($profile === Omitted::Value ? [] : ['profile' => $profile]), ...($threshold === Omitted::Value ? [] : ['threshold' => $threshold]), ...($wording_version === Omitted::Value ? [] : ['wording_version' => $wording_version])]);
    }
}
namespace ThinkThen\Inputs;
final class RelateQuestion extends \ThinkThen\Internal\Document implements \ThinkThen\Inputs\QuestionDefinition
{
    private const TAGS = ['version' => 1];
    public function __construct(public readonly RelationSpec $relate, public readonly mixed $context_schema = Omitted::Value, public readonly mixed $item_schema = Omitted::Value, public readonly string|Omitted $model = Omitted::Value, public readonly string|Omitted $name = Omitted::Value, public readonly string|Omitted $profile = Omitted::Value, public readonly int|float|string|Omitted $threshold = Omitted::Value, public readonly int|Omitted $wording_version = Omitted::Value)
    {
        parent::__construct([...self::TAGS, ...($context_schema === Omitted::Value ? [] : ['context_schema' => $context_schema]), ...($item_schema === Omitted::Value ? [] : ['item_schema' => $item_schema]), ...($model === Omitted::Value ? [] : ['model' => $model]), ...($name === Omitted::Value ? [] : ['name' => $name]), ...($profile === Omitted::Value ? [] : ['profile' => $profile]), 'relate' => $relate, ...($threshold === Omitted::Value ? [] : ['threshold' => $threshold]), ...($wording_version === Omitted::Value ? [] : ['wording_version' => $wording_version])]);
    }
}
namespace ThinkThen\Inputs;
final class RecognizeQuestion extends \ThinkThen\Internal\Document implements \ThinkThen\Inputs\QuestionDefinition
{
    private const TAGS = ['version' => 1];
    public function __construct(public readonly RecognitionSpec $recognize, public readonly mixed $context_schema = Omitted::Value, public readonly mixed $item_schema = Omitted::Value, public readonly string|Omitted $model = Omitted::Value, public readonly string|Omitted $name = Omitted::Value, public readonly string|array|Omitted $on = Omitted::Value, public readonly string|Omitted $profile = Omitted::Value, public readonly int|float|string|Omitted $relation_threshold = Omitted::Value, public readonly int|float|string|Omitted $threshold = Omitted::Value, public readonly int|Omitted $wording_version = Omitted::Value)
    {
        parent::__construct([...self::TAGS, ...($context_schema === Omitted::Value ? [] : ['context_schema' => $context_schema]), ...($item_schema === Omitted::Value ? [] : ['item_schema' => $item_schema]), ...($model === Omitted::Value ? [] : ['model' => $model]), ...($name === Omitted::Value ? [] : ['name' => $name]), ...($on === Omitted::Value ? [] : ['on' => $on]), ...($profile === Omitted::Value ? [] : ['profile' => $profile]), 'recognize' => $recognize, ...($relation_threshold === Omitted::Value ? [] : ['relation_threshold' => $relation_threshold]), ...($threshold === Omitted::Value ? [] : ['threshold' => $threshold]), ...($wording_version === Omitted::Value ? [] : ['wording_version' => $wording_version])]);
    }
}
namespace ThinkThen\Internal\Requests;
final class InputEntities extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'entities'];
    public function __construct(array $items, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'items' => $items, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class InputFeed extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'feed'];
    public function __construct(string $name, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'name' => $name, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class InputRecords extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'records'];
    public function __construct(array $items, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'items' => $items, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class InputSource extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'source'];
    public function __construct(Source $source, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'source' => $source, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class InputUnits extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'units'];
    public function __construct(array $items, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'items' => $items, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class Item extends \ThinkThen\Internal\Document
{
    public function __construct(array $fields = [])
    {
        parent::__construct($fields);
    }
}
namespace ThinkThen\Inputs;
final class Options extends \ThinkThen\Internal\Document
{
    public function __construct(public readonly bool|Omitted $attempts = Omitted::Value, public readonly int|string|Omitted $batch = Omitted::Value, public readonly string|Omitted $context = Omitted::Value, public readonly string|Omitted $context_field = Omitted::Value, public readonly int|Omitted $deadline_ms = Omitted::Value, public readonly array|Omitted $examples = Omitted::Value, public readonly string|Omitted $examples_field = Omitted::Value, public readonly array|Omitted $field = Omitted::Value, public readonly bool|Omitted $files_only = Omitted::Value, public readonly int|Omitted $max_requests_total = Omitted::Value, public readonly string|Omitted $mode = Omitted::Value, public readonly string|Omitted $model = Omitted::Value, public readonly bool|Omitted $none = Omitted::Value, public readonly string|Omitted $options_field = Omitted::Value, public readonly int|float|string|Omitted $relation_threshold = Omitted::Value, public readonly array|Omitted $seed_spans = Omitted::Value, public readonly string|Omitted $seed_spans_field = Omitted::Value, public readonly int|Omitted $snippet_pieces = Omitted::Value, public readonly RecognitionStageContext|Omitted $stage_context = Omitted::Value, public readonly int|float|string|Omitted $threshold = Omitted::Value, public readonly int|Omitted $top = Omitted::Value)
    {
        parent::__construct([...($attempts === Omitted::Value ? [] : ['attempts' => $attempts]), ...($batch === Omitted::Value ? [] : ['batch' => $batch]), ...($context === Omitted::Value ? [] : ['context' => $context]), ...($context_field === Omitted::Value ? [] : ['context_field' => $context_field]), ...($deadline_ms === Omitted::Value ? [] : ['deadline_ms' => $deadline_ms]), ...($examples === Omitted::Value ? [] : ['examples' => $examples]), ...($examples_field === Omitted::Value ? [] : ['examples_field' => $examples_field]), ...($field === Omitted::Value ? [] : ['field' => $field]), ...($files_only === Omitted::Value ? [] : ['files_only' => $files_only]), ...($max_requests_total === Omitted::Value ? [] : ['max_requests_total' => $max_requests_total]), ...($mode === Omitted::Value ? [] : ['mode' => $mode]), ...($model === Omitted::Value ? [] : ['model' => $model]), ...($none === Omitted::Value ? [] : ['none' => $none]), ...($options_field === Omitted::Value ? [] : ['options_field' => $options_field]), ...($relation_threshold === Omitted::Value ? [] : ['relation_threshold' => $relation_threshold]), ...($seed_spans === Omitted::Value ? [] : ['seed_spans' => $seed_spans]), ...($seed_spans_field === Omitted::Value ? [] : ['seed_spans_field' => $seed_spans_field]), ...($snippet_pieces === Omitted::Value ? [] : ['snippet_pieces' => $snippet_pieces]), ...($stage_context === Omitted::Value ? [] : ['stage_context' => $stage_context]), ...($threshold === Omitted::Value ? [] : ['threshold' => $threshold]), ...($top === Omitted::Value ? [] : ['top' => $top])]);
    }
}
namespace ThinkThen\Internal\Requests;
final class QuestionDefinition extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'definition'];
    public function __construct(mixed $value, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'value' => $value, ...$fields]);
    }
}
namespace ThinkThen\Inputs;
final class QuestionFile extends \ThinkThen\Internal\Document implements QuestionSource
{
    private const TAGS = ['kind' => 'file'];
    public function __construct(public readonly string $path)
    {
        parent::__construct([...self::TAGS, 'path' => $path]);
    }
}
namespace ThinkThen\Inputs;
final class QuestionName extends \ThinkThen\Internal\Document implements QuestionSource
{
    private const TAGS = ['kind' => 'name'];
    public function __construct(public readonly string $name)
    {
        parent::__construct([...self::TAGS, 'name' => $name]);
    }
}
namespace ThinkThen\Inputs;
final class QuestionReference extends \ThinkThen\Internal\Document implements QuestionSource
{
    private const TAGS = ['kind' => 'reference'];
    public function __construct(public readonly string $reference)
    {
        parent::__construct([...self::TAGS, 'reference' => $reference]);
    }
}
namespace ThinkThen\Internal\Requests;
final class QuestionText extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'text'];
    public function __construct(string $text, array $fields = [])
    {
        parent::__construct([...self::TAGS, 'text' => $text, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class Reader extends \ThinkThen\Internal\Document
{
    public function __construct(array $fields = [])
    {
        parent::__construct($fields);
    }
}
namespace ThinkThen\Internal\Requests;
final class ReaderFailureProducer extends \ThinkThen\Internal\Document
{
    public const TAGS = ['kind' => 'producer'];
    public function __construct(array $fields = [])
    {
        parent::__construct([...self::TAGS, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class SessionDescriptor extends \ThinkThen\Internal\Document
{
    public function __construct(Item $item, array $fields = [])
    {
        parent::__construct(['item' => $item, ...$fields]);
    }
}
namespace ThinkThen\Internal\Requests;
final class Source extends \ThinkThen\Internal\Document
{
    public function __construct(array $paths, array $fields = [])
    {
        parent::__construct(['paths' => $paths, ...$fields]);
    }
}
namespace ThinkThen;
trait NamedCalls
{
    public function decide(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('decide', $question, $input, $options, $cancel); }
    public function choose(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('choose', $question, $input, $options, $cancel); }
    public function tag(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('tag', $question, $input, $options, $cancel); }
    public function score(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('score', $question, $input, $options, $cancel); }
    public function filter(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('filter', $question, $input, $options, $cancel); }
    public function rank(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('rank', $question, $input, $options, $cancel); }
    public function find(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('find', $question, $input, $options, $cancel); }
    public function annotate(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('annotate', $question, $input, $options, $cancel); }
    public function recognize(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('recognize', $question, $input, $options, $cancel); }
    public function relate(string|array|\stdClass|\ThinkThen\Inputs\QuestionDefinition|\ThinkThen\Inputs\QuestionSource $question, mixed $input, \ThinkThen\Inputs\Options $options = new \ThinkThen\Inputs\Options(), ?Cancellation $cancel = null): \ThinkThen\Results\Completed { return $this->call('relate', $question, $input, $options, $cancel); }
}

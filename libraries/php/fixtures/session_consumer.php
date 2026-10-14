<?php
declare(strict_types=1);
require getenv('TT_AUTOLOAD');
use ThinkThen\Client;

/** Test adapter serialization is not a public library operation. */
function fixtureValue(mixed $value): mixed
{
    if ($value instanceof ThinkThen\Results\Node) {
        $value = (object)(new ReflectionProperty(ThinkThen\Results\Node::class, 'values'))->getValue($value);
    }
    if ($value instanceof stdClass) {
        $result = new stdClass();
        foreach ($value as $name => $field) $result->$name = fixtureValue($field);
        return $result;
    }
    return is_array($value) ? array_map(fixtureValue(...), $value) : $value;
}

function check(bool $ok, string $message): void { if (!$ok) throw new RuntimeException($message); }
$client = new Client(['cache' => false, 'batch' => 1, 'max_retries' => 0]);
$asks = [
    'decide' => ['Is it urgent?', 'text'],
    'choose' => [['choose' => 'Which?', 'options' => ['first', 'second']], 'text'],
    'tag' => [['tag' => 'Which?', 'labels' => ['first', 'second']], 'text'],
    'score' => [['score' => 'Which?', 'levels' => ['low', 'high']], 'text'],
    'filter' => ['Is it urgent?', ['filter-yes', 'no']],
    'rank' => ['Is it urgent?', ['rank-yes', 'no']],
    'find' => ['Which line?', ['find-first', 'find-second']],
    'annotate' => [['version' => 1, 'questions' => ['ok' => ['decide' => 'Is it urgent?']]], ['text']],
    'recognize' => [['version' => 1, 'recognize' => ['kinds' => ['person' => null]]], 'Maria Chen'],
    'relate' => [['version' => 1, 'relate' => ['relations' => [['name' => 'knows', 'source' => 'person', 'target' => 'person', 'either' => false]]]],
        [['name' => 'Ana', 'kind' => 'person'], ['name' => 'Bob', 'kind' => 'person']]],
];
$mode = $argv[1];
if ($mode === 'usage-written' || $mode === 'usage-failed') {
    $call = $client->decide('Is it?', 'surface-attribution');
    $earlier = json_encode(fixtureValue($call->facts()));
    check($call->results[0]->value === true && $call->facts()->requests_sent === 1, 'retained answer and sends');
    $observed = $client->usagePersistence();
    if ($mode === 'usage-failed') check($observed->state === ThinkThen\UsagePersistenceState::Pending && $observed->advice === null, 'held writer pending');
    $finished = $client->finishUsageStatus();
    $expected = $mode === 'usage-failed' ? ThinkThen\UsagePersistenceState::Failed : ThinkThen\UsagePersistenceState::Written;
    check($finished->state === $expected, 'finalized state');
    check($finished->advice === ($mode === 'usage-failed' ? 'check the usage folder permissions and free space' : null), 'native safe advice');
    check($client->usagePersistence() == $finished && $client->finishUsageStatus() == $finished, 'latched observation');
    check(json_encode(fixtureValue($call->facts())) === $earlier, 'retained facts changed');
    $client->close();
    foreach (['usagePersistence', 'finishUsageStatus'] as $method) {
        try { $client->$method(); throw new RuntimeException('closed method admitted'); }
        catch (ThinkThen\UsageFailure) {}
    }
    try { $finished->advice = 'changed'; throw new RuntimeException('mutable status'); } catch (Error) {}
    check($finished->state === $expected, 'owned status lost after close');
} elseif ($mode === 'surface') {
    $call = $client->decide('Is it?', 'surface-attribution');
    $client->close();
    check($call->results[0]->value === true, 'attributed call answer');
    check($call->facts() instanceof ThinkThen\Results\Facts && $call->facts()->requests_sent === 1, 'retained terminal send facts');
    check(is_string($call->facts()->call_id) && $call->facts()->call_id !== '', 'retained native call identity');
} elseif ($mode === 'named') {
    $retained = [];
    foreach ($asks as $verb => [$question, $input]) {
        $retained[$verb] = $client->$verb($question, $input);
        check($retained[$verb]->facts() instanceof ThinkThen\Results\Facts, 'typed native facts');
        check(count($retained[$verb]->results) > 0, 'native rows missing '.$verb);
        check($retained[$verb]->facts()->requests_sent > 0, 'native sends missing '.$verb);
    }
    check($retained['decide']->decideAnswer() === true, 'one safe decision');
    check(is_string($retained['choose']->chooseAnswer()) && is_array($retained['tag']->tagAnswer()) && is_numeric($retained['score']->scoreAnswer()), 'one safe scalar answer');
    check($retained['filter']->filterAnswer() === ['filter-yes'], 'filter returns only retained originals');
    check($retained['rank']->rankAnswer() === [0, 1], 'rank returns original zero-based indexes');
    check($retained['find']->findAnswer() instanceof ThinkThen\Results\Find, 'typed find answer');
    check($retained['annotate']->annotateAnswer()[0] instanceof ThinkThen\Results\Annotation, 'typed annotation answer');
    check($retained['recognize']->recognizeAnswer()[0] instanceof ThinkThen\Results\Recognition, 'typed recognition answer');
    $relation = $retained['relate']->relateAnswer();
    check($relation instanceof ThinkThen\Results\Relation && $relation->value[0]->sourceName() === 'Ana' && $relation->value[0]->targetName() === 'Bob', 'typed relation endpoints');
    try { $retained['decide']->rankAnswer(); throw new RuntimeException('wrong answer reader accepted'); }
    catch (ThinkThen\UsageFailure) {}
    check($client->filter('Is it?', [null, false])->filterAnswer() === [null, false], 'filter lost original null or false');
    $client->close();
    foreach ($retained as $call) foreach ($call->results as $row) {
        check($row instanceof ThinkThen\Results\Node, 'owned typed row');
        check(is_string($row->answer_id), 'owned answer ID');
        check(fixtureValue($row)->answer_id === $row->answer_id, 'lossless owned JSON');
    }
    ob_start();
    require dirname((new ReflectionClass(Client::class))->getFileName(), 3).'/examples/direct.php';
    $example = ob_get_clean();
    check(array_slice(array_map(fn($line) => explode(': ', $line, 2)[0], explode("\n", $example)), 0, count($asks)) === array_keys($asks), 'ten examples did not execute');
    check($retained['decide']->results[0]->value === true, 'true value');
    check($retained['decide']->results[0]->meta->question_sources[0] instanceof ThinkThen\Results\QuestionSource, 'typed nested optional source array');
} elseif ($mode === 'poll') {
    $produced = 0;
    $input = (function() use (&$produced) {
        foreach (['poll-first', 'no', 'poll-third'] as $item) { ++$produced; yield $item; }
    })();
    $operation = $client->start('decide', 'Is it?', $input);
    check($produced === 0, 'start consumed producer');
    $rows = []; $terminal = null; $deadline = microtime(true) + 3;
    while ($terminal === null) {
        $before = $produced; $packet = $operation->poll();
        check($produced <= $before + 1, 'poll consumed more than one producer item');
        if ($packet?->kind === 'row') $rows[] = $packet->value;
        if ($packet?->kind === 'terminal') $terminal = $packet;
        if (microtime(true) > $deadline) throw new RuntimeException('poll did not finish generator');
        $operation->wait();
    }
    $client->close();
    check($produced === 3 && array_map(fn($r) => $r->value, $rows) === [true, false, true], 'poll generator answers/order');
    check($terminal->facts instanceof ThinkThen\Results\Facts && $terminal->facts->requests_sent === 3 && !$terminal->has('failure'), 'poll typed terminal/count');
    foreach ($rows as $row) check($row instanceof ThinkThen\Results\DecisionResult && is_string($row->answer_id), 'poll retained typed answer');
} elseif ($mode === 'presence') {
    $serialized = new class implements JsonSerializable {
        public int $calls = 0;
        public function jsonSerialize(): mixed { return ++$this->calls === 1 ? 'serializer-first' : 'serializer-second'; }
    };
    $once = $client->decide('Is it?', $serialized);
    check($serialized->calls === 1 && $once->results[0]->input === 'serializer-first', 'serializer must run once and retain its first value');
    $annotated = $client->annotate(['version' => 1, 'questions' => [
        'ok' => ['decide' => 'Is it urgent?'], 'tags' => ['tag' => 'Which?', 'labels' => ['first', 'second']],
    ]], ['map-record']);
    $row = $annotated->results[0];
    check($row->answers()->value instanceof stdClass && $row->value()->value instanceof stdClass, 'annotation maps must remain objects');
    check($row->answers->ok instanceof ThinkThen\Results\AnnotationMemberAnswerId, 'typed map answer member');
    check($row->value->ok === true, 'map decision value');
    check(is_array($row->value->tags) && in_array('first', $row->value->tags, true), 'nested tags must remain an array');
    $false = $client->decide(['decide' => 'Is it?', 'threshold' => 0.95], 'text');
    $null = $client->decide(['decide' => 'Is it?', 'true' => null], 'text');
    check($false->results[0]->value()->present && $false->results[0]->value === false, 'false confused with absence');
    check(!$false->results[0]->question->field('true')->present, 'absent author field');
    check($null->results[0]->question->field('true')->present && $null->results[0]->question->true === null, 'authored null');
    $path = getenv('HOME').'/input.txt';
    file_put_contents($path, "located-first\n\nlocated-third\n");
    $located = $client->decide('Is it?', Client::files([$path], ['unit' => 'line']));
    check(array_map(fn($r) => $r->source->first_line, $located->results) === [1, 3], 'physical file provenance');
    $feed = $client->decide('Is it?', (function() { yield 'feed-first'; yield 'no'; })());
    check(array_map(fn($r) => $r->value, $feed->results) === [true, false], 'bounded producer order');
 } elseif ($mode === 'native-values') {
    $ffi = FFI::cdef(file_get_contents(dirname((new ReflectionClass(Client::class))->getFileName()).'/ffi_generated.h'));
    $view = $ffi->new('thinkthen_complete_immediate_facts_v1');
    $view->records = 1; $view->requests_sent = -1;
    $owned = ThinkThen\Internal\Decoder::failureFacts(FFI::addr($view));
    $number = $ffi->new('thinkthen_complete_json_v1');
    $decimal = '0.12345678901234567890123456789';
    $bytes = $ffi->new('char['.(strlen($decimal) + 1).']');
    FFI::memcpy($bytes, $decimal, strlen($decimal));
    $number->kind = 3; $number->data->number->data = FFI::addr($bytes[0]); $number->data->number->len = strlen($decimal);
    $read = new ReflectionMethod(ThinkThen\Internal\Decoder::class, 'jsonValue');
    check($read->invoke(null, $number) === $decimal, 'native authored decimal rounded');
    unset($view, $ffi);
    check($owned instanceof ThinkThen\Results\FailureFacts && !$owned->has('call_id'), 'count-only native facts gained an identity');
    check($owned->requests_sent === '18446744073709551615' && $owned->records === 1, 'native wide integer rounded or lifetime lost');
} elseif ($mode === 'failure') {
    try { $client->decide('Is it?', 'failure-one'); throw new RuntimeException('backend failure answered'); }
    catch (ThinkThen\BackendFailure $error) {
        check($error->failure instanceof ThinkThen\Results\Node, 'retained typed failure');
        check($error->facts()->requests_sent === 1, 'failure send facts');
        check($error->terminal->has('failure'), 'terminal failure missing');
        check($error->kind === ThinkThen\ErrorKind::Backend && !$error->retryable, 'typed error kind and retryability');
        $prefix = new ThinkThen\Results\Completed($error->results, $error->terminal, $error->packets);
        try { $prefix->decideAnswer(); throw new RuntimeException('failed prefix answered'); }
        catch (ThinkThen\BackendFailure $again) { check($again->facts()->requests_sent === 1, 'failed prefix lost facts'); }
    }
} elseif ($mode === 'zero') {
    $preview = $client->plan('decide', 'Is it?', 'secret-preview');
    check($preview instanceof ThinkThen\Results\Plan && $preview->records === 1 && $preview->requests === 1 && !$preview->upper_bound, 'typed native plan counts');
    $emptyPreview = $client->plan('decide', 'Is it?', []);
    check($emptyPreview->first_body_utf8()->present && $emptyPreview->first_body_utf8 === null, 'required null plan body became absent');
    check(!method_exists(ThinkThen\Results\Node::class, 'toObject'), 'public packet transport dump remains');
    check($preview->estimated_bytes > 0 && $preview->first_body_utf8()->present && str_contains($preview->first_body_utf8, 'secret-preview') && $preview->estimated_input_tokens instanceof ThinkThen\Results\TokenBand, 'complete typed plan fields');
    $produced = false;
    try { $client->plan('decide', 'Is it?', (function() use (&$produced) { $produced = true; yield 'unreachable'; })()); throw new RuntimeException('feed plan admitted'); }
    catch (ThinkThen\UsageFailure) {}
    check(!$produced, 'plan consumed producer');
    try { new Client([], getenv('TT_ERROR_LIBRARY')); throw new RuntimeException('synthetic constructor answered'); }
    catch (ThinkThen\BackendFailure $error) {
        check($error->retryable && $error->kind === ThinkThen\ErrorKind::Backend && $error->facts() instanceof ThinkThen\Results\FailureFacts, 'constructor error view not copied');
        check($error->facts()->records === 7 && $error->facts()->requests_sent === 3 && !$error->facts()->has('call_id'), 'constructor facts changed');
    }
    try { new Client(new ThinkThen\Inputs\Settings(max_retries: -1)); throw new RuntimeException('invalid settings admitted'); }
    catch (ThinkThen\UsageFailure $error) { check($error->kind === ThinkThen\ErrorKind::Usage && !$error->retryable && $error->facts() === null, 'constructor error kind/retryability/facts'); }
    $marker = new RuntimeException('private producer marker');
    $producer = (function() use ($marker) { throw $marker; yield 'unreachable'; })();
    try { $client->decide('Is it?', $producer); throw new RuntimeException('producer answered'); }
    catch (ThinkThen\LocalFailure $error) {
        check($error->getPrevious() === $marker, 'producer cause lost');
        check($error->getMessage() === 'session producer raised an exception' && $error->facts()->requests_sent === 0, 'producer native diagnostic changed');
    }
    try { $client->decide(['decide' => 'Is it?', 'item_schema' => ['type' => 'array']], 'unreachable'); throw new RuntimeException('unsupported declaration admitted'); }
    catch (ThinkThen\UsageFailure $error) { check($error->getMessage() === 'the question declaration uses an unsupported feature', 'native admission changed'); }

    try { $client->start('decide', 'Is it?', ['unreachable-first', Client::item('unreachable-tail', ['context' => false])]); throw new RuntimeException('invalid tail admitted eagerly'); }
    catch (ThinkThen\UsageFailure $error) {
        check($error->getMessage() === 'the per-item context does not match context_schema', 'eager tail admission changed');
        check(!$error->retryable && $error->facts() === null, 'immediate admission facts lost');
    }

    try { new Client([], getenv('HOME').'/missing-library.so'); throw new RuntimeException('missing library loaded'); }
    catch (ThinkThen\LocalFailure) {}
    try { $client->decide('', 'invalid'); throw new RuntimeException('empty admitted'); }
    catch (ThinkThen\UsageFailure) {}
    foreach (['{"decide":"secret-wording","wording_version":2e0}', '{"decide":"secret-wording","name":"a","name":"b"}', '{"decide":"secret-wording"}},"input":{"kind":"text","text":"injected"}'] as $raw) {
        file_put_contents(getenv('HOME').'/raw-question.json', $raw);
        try { $client->decide(Client::questionFile(getenv('HOME').'/raw-question.json'), 'secret-evidence'); throw new RuntimeException('raw definition admitted'); }
        catch (ThinkThen\LocalFailure $error) {
            check(!str_contains($error->getMessage(), 'secret-') && !str_contains($error->getMessage(), 'injected'), 'raw refusal exposed input');
        }
    }
    foreach (['DECIDE', 'Decide', 'missing'] as $function) {
        try { $client->start($function, 'Is it?', 'invalid-function'); throw new RuntimeException('unknown function admitted'); }
        catch (ThinkThen\UsageFailure $error) { check($error->getMessage() === 'unknown function', 'unknown function diagnostic'); }
    }
    $cycle = new stdClass(); $cycle->self = $cycle;
    $arrayCycle = []; $arrayCycle['self'] = &$arrayCycle;
    $serializedCycle = new class implements JsonSerializable {
        public int $calls = 0;
        public function jsonSerialize(): mixed {
            ++$this->calls;
            $cycle = new stdClass(); $cycle->self = $cycle;
            return $cycle;
        }
    };
    foreach ([$cycle, $arrayCycle, $serializedCycle, INF, NAN] as $invalid) {
        try { $client->decide('Is it?', $invalid); throw new RuntimeException('non-JSON input admitted'); }
        catch (ThinkThen\UsageFailure) {}
    }
    check($serializedCycle->calls === 1, 'cyclic serializer must run once');
    $token = new ThinkThen\Cancellation(); $token->cancel();
    try { $client->decide('Is it?', 'precancel', cancel: $token); throw new RuntimeException('cancel admitted'); }
    catch (ThinkThen\CancelledFailure) {}
} elseif ($mode === 'destroy') {
    $operation = $client->start('decide', 'Is it?', (function() { yield 'hold-php'; })());
    $deadline = microtime(true) + 3;
    while (!file_exists(getenv('TT_BARRIER').'/arrived-hold-php')) {
        $operation->poll();
        if (microtime(true) > $deadline) throw new RuntimeException('provider did not arrive');
        $operation->wait();
    }
    $weak = WeakReference::create($client);
    unset($client);
    check($weak->get() !== null, 'open operation lost client');
    $operation->poll();
    $operation->close();
    check($weak->get() === null, 'closed operation retained client');
    $temporary = (new Client(['cache' => false]))->start('decide', 'Is it?', 'temporary-client');
    check($temporary->result()->results[0]->value === true, 'temporary client closed operation');
    check(!file_exists(getenv('TT_BARRIER').'/release-hold-php'), 'cleanup needed provider release');
    $other = new Client(['cache' => false]);
    check($other->decide('Is it?', 'independent')->results[0]->value === true, 'cleanup stalled unrelated engine');
} else { throw new RuntimeException('unknown fixture mode'); }
echo "PHP_SESSION_PASS ".$mode."\n";

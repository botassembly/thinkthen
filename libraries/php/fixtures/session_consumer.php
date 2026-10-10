<?php
declare(strict_types=1);
require getenv('TT_AUTOLOAD');
use ThinkThen\Client;

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
if ($mode === 'named') {
    $retained = [];
    foreach ($asks as $verb => [$question, $input]) {
        $retained[$verb] = $client->$verb($question, $input);
        check($retained[$verb]->facts() instanceof ThinkThen\Results\NativeFacts, 'typed native facts');
        check(count($retained[$verb]->results) > 0, 'native rows missing '.$verb);
        check($retained[$verb]->facts()->requests_sent > 0, 'native sends missing '.$verb);
    }
    $client->close();
    foreach ($retained as $call) foreach ($call->results as $row) {
        check($row instanceof ThinkThen\Results\Node, 'owned typed row');
        check(is_string($row->answer_id), 'owned answer ID');
        check($row->toObject()->answer_id === $row->answer_id, 'lossless owned JSON');
    }
    check($retained['decide']->results[0]->value === true, 'true value');
    check($retained['decide']->results[0]->meta->question_sources[0] instanceof ThinkThen\Results\NativeQuestionSource, 'typed nested optional source array');
} elseif ($mode === 'presence') {
    $false = $client->decide(['decide' => 'Is it?', 'threshold' => 0.95], 'text');
    $null = $client->decide(['decide' => 'Is it?', 'true' => null], 'text');
    check($false->results[0]->value()->present && $false->results[0]->value === false, 'false confused with absence');
    check(!$false->results[0]->question->field('true')->present, 'absent author field');
    check($null->results[0]->question->field('true')->present && $null->results[0]->question->true === null, 'authored null');
    $original = $false->terminal->toObject();
    $original->unknown_extension = (object)['value' => false];
    $copy = ThinkThen\Results\Decoder::packet(json_encode($original, JSON_THROW_ON_ERROR));
    check($copy->has('unknown_extension') && $copy->unknown_extension->value === false && $copy->toObject()->unknown_extension->value === false, 'unknown output lost');
    $wideJson = str_replace('"requests_sent":1', '"requests_sent":18446744073709551615', $false->terminal->nativeJson());
    $wide = ThinkThen\Results\Decoder::packet($wideJson);
    check($wide->facts->requests_sent === '18446744073709551615' && $wide->nativeJson() === $wideJson, 'wide integer rounded');
    $path = getenv('HOME').'/input.txt';
    file_put_contents($path, "located-first\n\nlocated-third\n");
    $located = $client->decide('Is it?', Client::files([$path], ['unit' => 'line']));
    check(array_map(fn($r) => $r->source->first_line, $located->results) === [1, 3], 'physical file provenance');
    $feed = $client->decide('Is it?', (function() { yield 'feed-first'; yield 'no'; })());
    check(array_map(fn($r) => $r->value, $feed->results) === [true, false], 'bounded producer order');
} elseif ($mode === 'failure') {
    try { $client->decide('Is it?', 'failure-one'); throw new RuntimeException('backend failure answered'); }
    catch (ThinkThen\BackendFailure $error) {
        check($error->failure instanceof ThinkThen\Results\Node, 'retained typed failure');
        check($error->facts()->requests_sent === 1, 'failure send facts');
        check($error->terminal->has('failure'), 'terminal failure missing');
    }
} elseif ($mode === 'zero') {
    try { $client->decide('', 'invalid'); throw new RuntimeException('empty admitted'); }
    catch (ThinkThen\UsageFailure) {}
    $token = new ThinkThen\Cancellation(); $token->cancel();
    try { $client->decide('Is it?', 'precancel', cancel: $token); throw new RuntimeException('cancel admitted'); }
    catch (ThinkThen\CancelledFailure) {}
} elseif ($mode === 'destroy') {
    $operation = $client->start('decide', 'Is it?', 'hold-php');
    $deadline = microtime(true) + 3;
    while (!file_exists(getenv('TT_BARRIER').'/arrived-hold-php')) {
        $operation->poll();
        if (microtime(true) > $deadline) throw new RuntimeException('provider did not arrive');
        usleep(1000);
    }
    $weak = WeakReference::create($client);
    unset($client);
    check($weak->get() === null, 'open operation retained engine');
    try { $operation->poll(); throw new RuntimeException('destructor left native owner open'); }
    catch (ThinkThen\UsageFailure) {}
    check(!file_exists(getenv('TT_BARRIER').'/release-hold-php'), 'cleanup needed provider release');
    $other = new Client(['cache' => false]);
    check($other->decide('Is it?', 'independent')->results[0]->value === true, 'cleanup stalled unrelated engine');
} else { throw new RuntimeException('unknown fixture mode'); }
echo "PHP_SESSION_PASS ".$mode."\n";

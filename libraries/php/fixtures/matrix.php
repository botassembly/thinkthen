<?php
declare(strict_types=1);
require getenv('TT_AUTOLOAD');
function check(bool $ok, string $what): void { if (!$ok) throw new RuntimeException($what); }
function failed(callable $f, int $code): ThinkThenFailure {
    try { $f(); } catch (ThinkThenFailure $e) {
        check($e->nativeCode === $code, "expected $code, got " . $e->nativeCode . ' ' . $e->getMessage());
        return $e;
    }
    throw new RuntimeException("expected failure $code");
}
function refused(callable $f): void {
    try { $f(); } catch (InvalidArgumentException $e) { return; }
    throw new RuntimeException('NUL input was accepted');
}
function assertCallEnvelope(string $json, mixed $expected, string $label, int $expectedRecords=1, int $expectedRequests=1, bool $verifyValue=true): void {
    $row = json_decode($json, true, 512, JSON_THROW_ON_ERROR);
    if (!is_array($row) || array_keys($row) !== ['value', 'facts'] || ($verifyValue && $row['value'] !== $expected))
        throw new RuntimeException($label . ' envelope/value: ' . $json);
    $facts = $row['facts'];
    if (!is_array($facts) || array_keys($facts) !== ['cache_answers','input_tokens','model','output_tokens','records','requests_sent','seconds']
        || $facts['records'] !== $expectedRecords || $facts['requests_sent'] !== $expectedRequests
        || $facts['cache_answers'] !== 0 || $facts['input_tokens'] !== $expectedRequests
        || $facts['output_tokens'] !== $expectedRequests || $facts['model'] !== 'jev-1.13.0'
        || !is_float($facts['seconds']) || $facts['seconds'] < 0)
        throw new RuntimeException($label . ' facts: ' . $json);
}
function assertTypedFacts(array $result, int $records): void {
    check(array_keys($result) === ['value', 'facts'], 'typed envelope keys');
    $facts = $result['facts'];
    check(is_array($facts) && $facts['records'] === $records && is_int($facts['requests_sent'])
        && is_int($facts['cache_answers']) && is_numeric($facts['seconds']) && $facts['seconds'] >= 0,
        'typed facts: ' . json_encode($result));
    if ($records === 0) check($facts['requests_sent'] === 0, 'empty bulk sent work');
}
$door = new ThinkThen(getenv('TT_LIBRARY'));
$retained = null;
$typedRetained = null;
if (getenv('TT_FACTS_PROOF') === '1') {
    try {
        $one = $door->decide('Is it?', 'no-usage');
        assertTypedFacts($one, 1);
        check(!array_key_exists('input_tokens', $one['facts']) && !array_key_exists('output_tokens', $one['facts']), 'scalar omitted usage');
        // Another question, so the cache the scalar call wrote cannot answer 'no-usage' (ADR 0111).
        $many = $door->decideMany('Is it now?', ['no-usage', 'yes']);
        assertTypedFacts($many, 2);
        check(!array_key_exists('input_tokens', $many['facts']) && !array_key_exists('output_tokens', $many['facts']), 'bulk omitted usage');
        $saved = $one['facts'];
    } finally { $door->close(); }
    check($one['facts'] === $saved, 'typed facts changed after later call and close');
    echo "PHP_OMITTED_FACTS_PASS\n";
    return;
}
try {
    foreach (['café' => 1, 'yes' => 1, 'no' => 0, 'unsure' => 2] as $state => $outcome) {
        $q = $state === 'unsure' ? '{"decide":"Is it?","threshold":"0.4:0.8"}' : 'Is it?';
        $scalar = $door->decide($q, $state);
        assertTypedFacts($scalar, 1);
        check($scalar['value']['outcome'] === $outcome, 'scalar ' . $state);
    }
    check($door->decide('Is it?', "a\0b")['value']['outcome'] === 1, 'counted text NUL');
    failed(fn() => $door->decideMany('Is it?', ['bulk-before-bad','bulk-middle-bad','bulk-after-bad']), 2);
    $typedRetained = failed(fn() => $door->decide('Is it?', 'malformed-backend', 200), 2);
    check($typedRetained->factsJson !== null, 'typed started failure has no facts');
    $typedFactsJson = $typedRetained->factsJson;
    $typedFacts = json_decode($typedRetained->factsJson, true, 512, JSON_THROW_ON_ERROR);
    check($typedFacts['records'] === 0 && $typedFacts['requests_sent'] === 1
        && $typedFacts['cache_answers'] === 0, 'typed started failure facts');
    $typedMessage = $typedRetained->getMessage();
    foreach (['transport-close','retry-status'] as $state) {
        try { $door->decide('Is it?', $state, 200); throw new RuntimeException('backend failure passed'); }
        catch (ThinkThenFailure $e) { check(in_array($e->nativeCode, [2,3], true), 'backend failure code'); }
    }
    check($door->decide('Is it?', 'post-failure-recovery')['value']['outcome'] === 1, 'failure recovery');
    check($typedRetained->getMessage() === $typedMessage && $typedRetained->factsJson === $typedFactsJson
        && json_decode($typedRetained->factsJson, true, 512, JSON_THROW_ON_ERROR) === $typedFacts,
        'typed failure facts changed after later same-engine call');
    check($door->decide('Is it?', 'maximum-deadline', 4294967295000)['value']['outcome'] === 1, 'max deadline');
    $empty = $door->decideMany('Is it?', []);
    assertTypedFacts($empty, 0);
    check($empty['value'] === [] && !array_key_exists('input_tokens', $empty['facts']), 'empty bulk');
    $rows = $door->decideMany('Is it?', ['first','second','third']);
    assertTypedFacts($rows, 3);
    check(array_column($rows['value'], 'probability') === [0.9,0.1,0.6], 'reverse completion reorders: '.json_encode($rows));
    $repeated = $door->decideMany('Is it?', ['first','second','first']);
    check(array_column($repeated['value'], 'probability') === [0.9,0.1,0.9], 'repeated bulk');
    $requests = [
      '{"decide":"Is it?","evidence":"json-decide","details":true}',
      '{"choose":"Which team?","options":["first","second"],"evidence":"choose"}',
      '{"tag":"Which labels?","labels":["first","second"],"evidence":"tag"}',
      '{"score":"What level?","levels":["Low.","High."],"evidence":"score"}',
      '{"filter":"Is it?","records":["filter-one","filter-two"]}',
      '{"rank":"Is it?","records":["rank-one","rank-two"]}',
      '{"find":"Which line?","units":["find-one","find-two"]}',
      '{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?"}}},"records":["annotate-one"]}',
      '{"recognize":{"kinds":{"person":"A person\'s name."}},"version":1,"evidence":"Maria Chen"}',
      '{"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]},"version":1,"records":[{"name":"First","kind":"alert"},{"name":"Second","kind":"alert"}]}',
      '{"find":"Which line?","none":true,"units":["find-none","find-another"]}',
      '{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?","on":"/body"}}},"records":["{\\"body\\":\\"annotate-on\\",\\"hidden\\":\\"not-sent\\"}"]}'
    ];
    foreach ($requests as $i => $request) {
        $result = $door->call($request);
        $envelope = json_decode($result, true, 512, JSON_THROW_ON_ERROR);
        file_put_contents(getenv('TT_BARRIER_DIR').'/call-results.jsonl', json_encode(['index'=>$i,'result'=>$envelope], JSON_UNESCAPED_UNICODE)."\n", FILE_APPEND);
        check(is_array($envelope) && array_keys($envelope) === ['value','facts'], 'call success envelope keys');
        assertCallEnvelope($result, null, 'call index '.$i, in_array($i,[4,5],true)?2:1, $i===8?2:1, false);
        $result = json_encode($envelope['value'], JSON_UNESCAPED_UNICODE|JSON_UNESCAPED_SLASHES|JSON_THROW_ON_ERROR);
        switch ($i) {
            case 0:
                $v=$envelope['value'];
                check(is_array($v) && array_keys($v) === ['schema','value','question','answer','threshold','meta']
                    && $v['schema']==='thinkthen.result/1' && $v['value']===true
                    && $v['question']===['verb'=>'decide','text'=>'Is it?']
                    && $v['answer']===['kind'=>'yes_no','probability'=>0.9]
                    && $v['threshold']===0.5 && is_array($v['meta']), 'details'); break;
            case 1: check($result === '"first"', 'choose'); break;
            case 2: check($result === '["first","second"]', 'tag'); break;
            case 3: check((float)$result === 0.1, 'score'); break;
            case 4: check(str_contains($result, 'filter-one'), 'record order'); break;
            case 5: check($result === '[{"index":0,"record":"rank-one","probability":0.9},{"index":1,"record":"rank-two","probability":0.9}]', 'rank'); break;
            case 6: check($result === '{"index":0,"unit":"find-one","probability":0.9}', 'find'); break;
            case 7: case 11: check(str_contains($result, '"check":true'), 'annotate'); break;
            case 8: check(str_contains($result, '"length"') && str_contains($result,'"text"'), 'recognize shape'); break;
            case 9: check(str_contains($result, '"edges"'), 'relate'); break;
            case 10: check($result === 'null', 'none'); break;
        }
    }
    $spec = '{"version":1,"recognize":{"kinds":{"person":"A person\'s name."}}}';
    $named = $door->recognize($spec, 'John Smith');
    assertTypedFacts($named, 1);
    check(str_contains($named['value'], '"length"'), 'typed recognize');
    check(str_contains($door->recognize('{"version":1,"recognize":{}}', 'Ada Lovelace')['value'], '"kind":"ENTITY"'), 'default kind');
    $relation = '{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}';
    $edges = $door->relate($relation, ['{"name":"Third","kind":"alert"}','{"name":"Fourth","kind":"alert"}']);
    assertTypedFacts($edges, 1);
    check(str_contains($edges['value'], '"edges"'), 'typed relate');
    check(str_contains($door->call('{"usage":true}'), 'requests_sent'), 'usage');
    refused(fn() => $door->decide("Is it?\0bad",'x'));
    refused(fn() => $door->call("{}\0bad"));
    refused(fn() => $door->recognize("{}\0bad",'x'));
    refused(fn() => $door->relate("{}\0bad",[]));
    failed(fn() => $door->decide('Is it?', "x\xff"), 1);
    $prestart = failed(fn() => $door->decide('Is it?', ''), 1);
    check($prestart->factsJson === null, 'pre-start failure had facts');
    failed(fn() => $door->decide('Is it?', 'x', 0), 3);
    failed(fn() => $door->decide('Is it?', 'x', -2), 1);
    failed(fn() => $door->decide('Is it?', 'x', 4294967295001), 1);
    failed(fn() => $door->call('bad json'), 1);
    $retained = failed(fn() => $door->call('{"decide":"Is it?","evidence":"failure-one"}'), 2);
    check($retained->kind === 'backend' && $retained->factsJson !== null, 'named failure/facts');
    $failureFacts = json_decode($retained->factsJson, true, 512, JSON_THROW_ON_ERROR);
    check($failureFacts['requests_sent'] === 1 && $failureFacts['cache_answers'] === 0,
          'started failure facts: ' . $retained->factsJson);
    $message = $retained->getMessage();
    failed(fn() => $door->call('{"decide":"Is it?","evidence":"failure-two"}'), 2);
    check($retained->getMessage() === $message, 'error message copied before next failure');
    check(json_decode($retained->factsJson, true, 512, JSON_THROW_ON_ERROR) === $failureFacts,
          'error facts copied before next failure');
    $other = new ThinkThen(getenv('TT_LIBRARY'));
    try { check($other->decide('Is it?', 'success')['value']['outcome'] === 1, 'other engine'); }
    finally { $other->close(); }
    $token = $door->token();
    try {
        $door->fire($token); $door->fire($token);
        failed(fn() => $door->decide('Is it?', 'never-sent', -1, $token), 5);
    } finally { $door->freeToken($token); }
    failed(fn() => $door->decide('Is it?', 'hold-deadline', 1000), 3);
    check($door->decide('Is it?', 'recovery-scalar')['value']['outcome'] === 1, 'recovery after deadline');
} finally { $door->close(); }
check(isset($scalar) && isset($scalar['facts']) && $scalar['facts']['records'] === 1, 'typed facts changed after close');
check($typedRetained !== null && $typedRetained->getMessage() === $typedMessage
    && $typedRetained->factsJson === $typedFactsJson
    && json_decode($typedRetained->factsJson, true, 512, JSON_THROW_ON_ERROR) === $typedFacts,
    'typed failure facts changed after close');
check($retained !== null && $retained->nativeCode === 2 && $retained->getMessage() !== '', 'copied error survives engine free');
echo "PHP_MATRIX_PASS\n";

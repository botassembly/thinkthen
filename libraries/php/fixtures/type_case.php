<?php
declare(strict_types=1);
require dirname(__DIR__) . '/autoload.php';

/** A failure as the corpus runner compares it: the named kind and its code. */
function failure(ThinkThenFailure $failure): array
{
    return ['failed' => ['kind' => $failure->kind, 'code' => $failure->nativeCode]];
}
function check(bool $ok, string $what): void { if (!$ok) throw new RuntimeException($what); }

$mode = $argv[1] ?? 'call';
$settings = $mode === 'limits' ? '{"max_requests_total":0,"cache":false}' : null;
$door = new ThinkThen(getenv('TT_LIBRARY'), $settings);
try {
    try {
        switch ($mode) {
            case 'plan': // plan VERB QUESTION SETTINGS TEXT...
                $result = $door->plan($argv[2], $argv[3], array_slice($argv, 5), $argv[4]);
                break;
            case 'helper': // The null-versus-failure edge table.
                $failure = ['kind' => 'backend', 'cause' => 'missing_answer'];
                check(ThinkThen::failed(['failed' => $failure]) === $failure, 'failure');
                check(ThinkThen::failed(['failed' => $failure + ['surprise' => 1]])['cause'] === 'missing_answer', 'extra member');
                foreach ([null, true, false, 'billing', 1.2, ['billing', 'urgent'], [], ['failed' => 'text'],
                          ['failed' => $failure, 'other' => 1]] as $value)
                    check(ThinkThen::failed($value) === null, 'value read as failure: ' . json_encode($value));
                check([ThinkThen::NO, ThinkThen::YES, ThinkThen::UNSURE] === [0, 1, 2], 'outcome codes');
                $result = ['helper' => 'pass'];
                break;
            case 'limits': // A zero cap and zero budgets refuse before any send.
                try { $door->decide('Is it?', 'capped'); throw new RuntimeException('zero cap sent'); }
                catch (ThinkThenFailure $e) { check($e->kind === 'usage' && str_contains($e->getMessage(), 'process send budget'), 'cap: ' . $e->getMessage()); }
                $pairs = ['{"name":"First","kind":"alert"}', '{"name":"Second","kind":"alert"}'];
                $spent = [
                    fn() => $door->call('{"decide":"Is it?","evidence":"spent"}', 0),
                    fn() => $door->recognize('{"version":1,"recognize":{}}', 'Ada Lovelace', 0),
                    fn() => $door->relate('{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}', $pairs, 0),
                ];
                foreach ($spent as $call) {
                    try { $call(); throw new RuntimeException('zero budget sent'); }
                    catch (ThinkThenFailure $e) { check($e->kind === 'deadline' && $e->nativeCode === 3, 'budget: ' . $e->getMessage()); }
                }
                $result = ['limits' => 'pass'];
                break;
            case 'fields': // Each answer of each annotate row, read through ThinkThen::failed.
                $rows = json_decode($door->call(stream_get_contents(STDIN)), true, 512, JSON_THROW_ON_ERROR)['value'];
                $result = array_map(fn(array $row) => array_map(function (mixed $member): string {
                    $failed = ThinkThen::failed($member);
                    return $failed !== null ? "failed {$failed['kind']} {$failed['cause']}" : ($member === null ? 'unresolved' : 'answered');
                }, $row), $rows);
                break;
            default:
                echo $door->call(stream_get_contents(STDIN)), "\n";
                return;
        }
    } catch (ThinkThenFailure $failure) {
        $result = failure($failure);
    }
    echo json_encode($result, JSON_THROW_ON_ERROR | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE), "\n";
} finally {
    $door->close();
}

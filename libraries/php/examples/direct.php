<?php
declare(strict_types=1);
require dirname(__DIR__) . '/autoload.php';
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
$door = new ThinkThen(getenv('TT_LIBRARY'));
try {
    $answer = $door->decide('Is it?', 'direct-php');
    if ($answer['outcome'] !== 1 || $answer['probability'] !== 0.9) throw new RuntimeException('direct scalar: ' . json_encode($answer));
    $result = $door->call('{"decide":"Is it?","evidence":"direct-json"}');
    assertCallEnvelope($result, true, 'direct JSON');
    echo "DIRECT_PHP_PASS\n";
} finally { $door->close(); }

<?php
declare(strict_types=1);
require '/work/install with spaces/package/autoload.php';

$library = '/work/install with spaces/native/lib/libthinkthen.so';
$settings = getenv('TT_USE_SETTINGS') === '1'
    ? json_encode(['base_url' => getenv('TT_SETTINGS_BASE_URL')], JSON_THROW_ON_ERROR) : null;
$door = new ThinkThen($library, $settings);
try {
    $answer = $door->decide('Is it?', 'consumer-php');
    if ($answer['value'] !== ['outcome' => 1, 'probability' => 0.9] || $answer['facts']['records'] !== 1 || $answer['facts']['requests_sent'] !== 1) throw new RuntimeException('scalar mismatch');
    $envelope = json_decode($door->call('{"decide":"Is it?","evidence":"consumer-json"}'), true, 512, JSON_THROW_ON_ERROR);
    $expected = getenv('TT_PLANT_WRONG_VALUE') === '1' ? false : true;
    if (array_keys($envelope) !== ['value', 'facts'] || $envelope['value'] !== $expected
        || $envelope['facts']['requests_sent'] !== 1 || $envelope['facts']['records'] !== 1)
        throw new RuntimeException('value/facts mismatch');
    $token = $door->token();
    try {
        $door->fire($token);
        try { $door->decide('Is it?', 'never-sent', -1, $token); throw new RuntimeException('spent token accepted'); }
        catch (ThinkThenFailure $failure) {
            if ($failure->kind !== 'cancelled' || $failure->factsJson !== null) throw $failure;
        }
    } finally { $door->freeToken($token); }
    echo "INSTALLED_PHP_CONSUMER_PASS\n";
} finally { $door->close(); }

try { new ThinkThen($library, '{"unknown":true}'); throw new RuntimeException('invalid settings accepted'); }
catch (ThinkThenFailure $failure) {
    if ($failure->kind !== 'usage' || $failure->factsJson !== null) throw $failure;
}

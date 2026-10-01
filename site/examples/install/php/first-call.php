<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$question = 'Does the customer ask for a refund?';
$isRefund = $tt->decide(
    $question,
    'Please refund my order. It arrived broken.',
);
$outcome = $isRefund['value']['outcome'];
assert($outcome === ThinkThen::YES);

$refund = '{"decide": "' . $question . '", '
    . '"threshold": "0.2:0.8"}';
$backIsRefund = $tt->decide(
    $refund,
    'I want to send this back.',
);
$backOutcome = $backIsRefund['value']['outcome'];
assert($backOutcome === ThinkThen::UNSURE);
$tt->close();

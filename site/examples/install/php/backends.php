<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$question = 'Does the customer ask for a refund?';
$brokenIsRefund = $tt->decide(
    $question,
    'Please refund my order. It arrived broken.',
);
$thanksIsRefund = $tt->decide(
    $question,
    'Thanks for the quick help yesterday!',
);
$broken = $brokenIsRefund['value']['outcome'];
$thanks = $thanksIsRefund['value']['outcome'];
assert($broken === ThinkThen::YES);
assert($thanks === ThinkThen::NO);
$tt->close();

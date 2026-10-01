<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$question = 'Does the customer ask for a refund?';
$isRefund = $tt->decide(
    $question,
    'Please refund my order. It arrived broken.',
)['value']['outcome'];
assert($isRefund === ThinkThen::YES);
$tt->close();

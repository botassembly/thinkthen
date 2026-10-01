<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$urgency = $tt->call(json_encode([
    'score' => 'How urgent is this?',
    'levels' => ['Routine.', 'Soon.', 'Immediate.'],
    'evidence' => "Our checkout page is down "
        . "and customers cannot pay.\n",
]));
$level = json_decode($urgency, true)['value'];
assert($level === 2.0);
$tt->close();

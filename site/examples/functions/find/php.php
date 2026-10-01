<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$units = [
    'Returns need the original receipt.',
    'Refunds are issued within 30 days of purchase.',
    'Shipping is free on orders over $50.',
    'Gift cards cannot be exchanged for cash.',
];
$deadline = $tt->call(json_encode([
    'find' => 'Which line gives the refund deadline?',
    'units' => $units,
]));
$line = json_decode($deadline, true)['value']['unit'];
assert($line === $units[1]);
$tt->close();

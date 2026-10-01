<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$mostUrgent = $tt->call(json_encode([
    'rank' => 'Is this urgent?',
    'records' => [
        'Newsletter: our autumn catalog is here. '
            . 'No reply needed.',
        'Our checkout page is down and customers '
            . 'cannot pay',
        'Reminder: your invoice is due in 30 days',
        'Please send the signed quote by 5 pm today',
    ],
]));
$order = json_decode($mostUrgent, true)['value'];
assert(array_column($order, 'index') === [1, 3, 2, 0]);
$tt->close();

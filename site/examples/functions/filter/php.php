<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$complaints = $tt->call(json_encode([
    'filter' => 'Is this a complaint?',
    'records' => [
        'Arrived a day early. Thank you!',
        'The zipper broke the first time I used it.',
        'Does this come in blue?',
        'The strap snapped on day two.',
    ],
]));
$reviews = json_decode($complaints, true)['value'];
assert($reviews === [
    'The zipper broke the first time I used it.',
    'The strap snapped on day two.',
]);
$tt->close();

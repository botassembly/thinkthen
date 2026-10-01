<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$spec = json_encode(['version' => 1, 'recognize' => [
    'kinds' => [
        'person' => null,
        'organization' => null,
        'place' => null,
    ],
]]);
$text = 'Maria Chen joined Northwind Freight, '
    . 'a company in Chicago.';
$facts = $tt->recognize($spec, $text);
$entities = json_decode($facts['value'], true)['entities'];
$names = array_map(
    fn ($one) => [$one['text'], $one['kind']],
    $entities,
);
assert($names === [
    ['Maria Chen', 'person'],
    ['Northwind Freight', 'organization'],
    ['Chicago', 'place'],
]);
$tt->close();

<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$spec = json_encode(['version' => 1, 'recognize' => [
    'kinds' => [
        'PER' => "Part of a person's name.",
        'ORG' => 'Part of the name of an organization: '
            . 'a company, band, team, agency, government '
            . 'body, or media outlet.',
        'LOC' => 'Part of the name of a place: a country, '
            . 'region, city, or geographic feature.',
        'MISC' => 'Part of another named entity: a '
            . 'nationality, an event, a product, or the '
            . 'name of a creative work.',
    ],
]]);
$names = $tt->recognize(
    $spec,
    'Maria Chen joined Northwind Freight in Chicago '
        . 'last spring.',
);
$entities = json_decode($names['value'], true)['entities'];
$spans = array_map(
    fn ($one) => [$one['text'], $one['kind']],
    $entities,
);
assert($spans === [
    ['Maria Chen', 'PER'],
    ['Northwind Freight', 'ORG'],
    ['Chicago', 'LOC'],
]);
$tt->close();

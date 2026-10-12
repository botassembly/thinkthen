<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$kinds = ['person', 'organization', 'place'];
$text = 'Maria Chen joined Northwind Freight, '
    . 'a company in Chicago.';
$ask = [
    'version' => 1,
    'recognize' => [
        'kinds' => array_fill_keys($kinds, null),
    ],
];
$facts = $client->recognize($ask, $text)->results[0]->value;
$names = array_map(
    fn ($one) => [$one->text, $one->kind],
    $facts->entities,
);
assert($names === [
    ['Maria Chen', 'person'],
    ['Northwind Freight', 'organization'],
    ['Chicago', 'place'],
]);
$client->close();

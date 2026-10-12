<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$kinds = ['person', 'product', 'place'];
$text = 'Sam Patel says his Kindle arrived broken '
    . 'in Denver.';
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
    ['Sam Patel', 'person'],
    ['Kindle', 'product'],
    ['Denver', 'place'],
]);
$client->close();

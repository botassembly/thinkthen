<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$names = [
    ['name' => 'Apple', 'kind' => 'company'],
    ['name' => 'Amazon', 'kind' => 'company'],
    ['name' => 'iPhone', 'kind' => 'product'],
    ['name' => 'Kindle', 'kind' => 'product'],
];
$makes = [
    'name' => 'makes',
    'source' => 'company',
    'target' => 'product',
];
$ask = [
    'version' => 1,
    'relate' => ['relations' => [$makes]],
];
$whoMakes = $client->relate($ask, $names)
    ->relateAnswer()->value;
$pairs = array_map(
    fn ($edge) => [
        $edge->source->name,
        $edge->target->name,
    ],
    $whoMakes,
);
assert($pairs === [
    ['Apple', 'iPhone'],
    ['Amazon', 'Kindle'],
]);
$client->close();

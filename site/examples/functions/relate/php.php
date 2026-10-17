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
$makes = new ThinkThen\Inputs\RelationRule(
    name: 'makes', source: 'company', target: 'product',
);
$ask = new ThinkThen\Inputs\RelateQuestion(
    new ThinkThen\Inputs\RelationSpec([$makes]),
);
$whoMakes = $client->relate($ask, $names)
    ->value;
$pairs = array_map(
    fn ($edge) => [
        $edge->sourceName(),
        $edge->targetName(),
    ],
    $whoMakes,
);
assert($pairs === [
    ['Apple', 'iPhone'],
    ['Amazon', 'Kindle'],
]);
$client->close();

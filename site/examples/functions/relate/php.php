<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$names = [
    ['name' => 'Paul McCartney', 'kind' => 'singer'],
    ['name' => 'Ringo Starr', 'kind' => 'singer'],
    ['name' => 'Yesterday', 'kind' => 'song'],
    ['name' => "Octopus's Garden", 'kind' => 'song'],
];
$sings = [
    'name' => 'sings',
    'source' => 'singer',
    'target' => 'song',
];
$ask = [
    'version' => 1,
    'relate' => ['relations' => [$sings]],
];
$whoSings = $client->relate($ask, $names)
    ->results[0]->value;
$pairs = array_map(
    fn ($edge) => [
        $edge->source->name,
        $edge->target->name,
    ],
    $whoSings,
);
assert($pairs === [
    ['Paul McCartney', 'Yesterday'],
    ['Ringo Starr', "Octopus's Garden"],
]);
$client->close();

<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$spec = json_encode(['version' => 1, 'relate' => [
    'relations' => [
        ['name' => 'sings', 'source' => 'singer',
            'target' => 'song'],
    ],
]]);
$sings = $tt->relate($spec, [
    '{"name": "Paul McCartney", "kind": "singer"}',
    '{"name": "Ringo Starr", "kind": "singer"}',
    '{"name": "Yesterday", "kind": "song"}',
    '{"name": "Octopus\'s Garden", "kind": "song"}',
]);
$edges = json_decode($sings['value'], true)['edges'];
$pairs = array_map(fn ($edge) => [
    $edge['source']['name'],
    $edge['target']['name'],
    $edge['probability'],
], $edges);
assert($pairs === [
    ['Paul McCartney', 'Yesterday', 0.81],
    ['Ringo Starr', "Octopus's Garden", 0.88],
]);
$tt->close();

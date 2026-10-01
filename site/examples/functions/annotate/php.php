<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$form = json_decode(file_get_contents('form.json'));
$triage = $tt->call(json_encode([
    'annotate' => $form,
    'records' => [
        'Steps: click Export. It is very slow.',
        'Steps: click Log in. Nobody gets in.',
        'The Pay button on billing is too blue.',
    ],
]));
$rows = json_decode($triage, true)['value'];
$triaged = array_map(fn ($row) => [
    $row['steps'],
    $row['area'],
    $row['impact'],
], $rows);
assert($triaged === [
    [true, 'export', 1.04],
    [true, 'login', 1.98],
    [false, 'billing', 0.09],
]);
$tt->close();

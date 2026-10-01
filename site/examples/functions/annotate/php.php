<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$form = json_decode(file_get_contents('form.json'));
$report = 'Steps: click Log in. Nobody gets in.';
$rows = $tt->call(json_encode([
    'annotate' => $form,
    'records' => [$report],
]));
$triage = json_decode($rows, true)['value'];
assert($triage === [
    ['steps' => true, 'area' => 'login', 'impact' => 1.98],
]);
$tt->close();

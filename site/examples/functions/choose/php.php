<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$question = 'Which team owns this?';
$teams = [
    'billing' => 'Invoices, fees, and refunds.',
    'shipping' => 'Parcels and delivery.',
    'account' => 'Logins and passwords.',
];
$parcel = 'My parcel went to the wrong address.';
$owner = $tt->call(json_encode([
    'choose' => $question,
    'options' => $teams,
    'evidence' => $parcel,
]));
$team = json_decode($owner, true)['value'];
assert($team === 'shipping');
$tt->close();

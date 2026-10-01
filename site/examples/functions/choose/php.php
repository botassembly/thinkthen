<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$owner = $tt->call(json_encode([
    'choose' => 'Which team owns this?',
    'options' => [
        'billing' => 'Invoices, fees, and refunds.',
        'shipping' => 'Parcels and delivery.',
        'account' => 'Logins and passwords.',
    ],
    'records' => [
        'Please refund the extra fee on my invoice.',
        'My parcel went to the wrong address.',
        'I cannot reset my password.',
    ],
]));
$teams = json_decode($owner, true)['value'];
assert($teams === ['billing', 'shipping', 'account']);
$tt->close();

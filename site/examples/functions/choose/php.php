<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'Which team owns this?';
$teams = [
    'billing' => 'Invoices, fees, and refunds.',
    'shipping' => 'Parcels and delivery.',
    'account' => 'Logins and passwords.',
];
$parcel = 'My parcel went to the wrong address.';
$team = $client->choose(
    ['choose' => $question, 'options' => $teams],
    $parcel,
)->chooseAnswer();
assert($team === 'shipping');
$client->close();

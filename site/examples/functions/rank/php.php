<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'Is this urgent?';
$inbox = [
    'Newsletter: our autumn catalog is here. '
        . 'No reply needed.',
    'Our checkout page is down and customers cannot pay',
    'Reminder: your invoice is due in 30 days',
    'Please send the signed quote by 5 pm today',
];
$order = $client->rank($question, $inbox)->rankAnswer();
assert($order === [1, 3, 2, 0]);
$client->close();

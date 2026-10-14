<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'Which line gives the refund deadline?';
$policy = [
    'Returns need the original receipt.',
    'Refunds are issued within 30 days of purchase.',
    'Shipping is free on orders over $50.',
    'Gift cards cannot be exchanged for cash.',
];
$refundDeadline = $client->find($question, $policy)
    ->findAnswer()->value;
assert($refundDeadline === $policy[1]);
$client->close();

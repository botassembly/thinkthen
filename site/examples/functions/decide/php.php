<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'Does the customer ask for a refund?';
$broken = 'Please refund my order. It arrived broken.';
$isRefund = $client->decide($question, $broken)
    ->results[0]->value;
assert($isRefund === true);
$client->close();

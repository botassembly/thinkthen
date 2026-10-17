<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'How urgent is this?';
$levels = ['Routine.', 'Soon.', 'Immediate.'];
$outage = "Our checkout page is down "
    . "and customers cannot pay.\n";
$urgency = $client->score(
    ['score' => $question, 'levels' => $levels],
    $outage,
)->value;
assert($urgency === 2.0);
$client->close();

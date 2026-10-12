<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'Which labels fit this message?';
$labels = ['praise', 'bug', 'billing'];
$message = "Love the new dashboard, "
    . "but export crashes the app,\n"
    . "and I was charged twice.\n";
$fittingLabels = $client->tag(
    ['tag' => $question, 'labels' => $labels],
    $message,
)->results[0]->value;
assert($fittingLabels === ['praise', 'bug', 'billing']);
$client->close();

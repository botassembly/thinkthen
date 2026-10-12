<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$report = 'Steps: click Log in. Nobody gets in.';
$triage = $client->annotate(
    Client::questionFile('form.json'),
    [$report],
)->results[0]->value;
assert((array) $triage === [
    'steps' => true,
    'area' => 'login',
    'impact' => 1.98,
]);
$client->close();

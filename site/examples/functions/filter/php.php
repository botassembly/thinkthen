<?php
require 'thinkthen-php/autoload.php';

use ThinkThen\Client;

$client = new Client();
$question = 'Is this a complaint?';
$reviews = [
    'Arrived a day early. Thank you!',
    'The zipper broke the first time I used it.',
    'Does this come in blue?',
    'The strap snapped on day two.',
];
$complaints = array_map(
    fn ($row) => $row->input,
    $client->filter($question, $reviews)->results,
);
assert($complaints === [$reviews[1], $reviews[3]]);
$client->close();

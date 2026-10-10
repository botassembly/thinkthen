<?php
declare(strict_types=1);
require dirname(__DIR__).'/autoload.php';
$client = new ThinkThen\Client();
try {
    $call = $client->decide('Does this ask for a refund?', 'Please refund my order.');
    echo json_encode($call->results[0]->value()->value, JSON_THROW_ON_ERROR), "\n";
} finally { $client->close(); }

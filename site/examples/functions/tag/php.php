<?php
declare(strict_types=1);
require __DIR__ . '/thinkthen-php/autoload.php';

$library = __DIR__ . '/thinkthen-c/lib/libthinkthen.so';
$tt = new ThinkThen($library);

$fitting = $tt->call(json_encode([
    'tag' => 'Which labels fit this message?',
    'labels' => ['praise', 'bug', 'billing'],
    'evidence' => "Love the new dashboard, but export "
        . "crashes the app,\nand I was charged twice.\n",
]));
$labels = json_decode($fitting, true)['value'];
assert($labels === ['praise', 'bug', 'billing']);
$tt->close();

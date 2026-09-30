<?php
// The replay smoke (ticket 0335): one decide through the environment-reading
// engine, with the question and text sdlc/scripts/smoke names.
declare(strict_types=1);
require dirname(__DIR__) . '/autoload.php';
$door = new ThinkThen(getenv('TT_LIBRARY'));
try {
    $outcome = $door->decide(getenv('THINKTHEN_TEST_SMOKE_QUESTION'), getenv('THINKTHEN_TEST_SMOKE_TEXT'))['value']['outcome'];
    echo 'smoke: ' . [0 => 'false', 1 => 'true', 2 => 'null'][$outcome] . "\n";
} finally { $door->close(); }

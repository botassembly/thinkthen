<?php
declare(strict_types=1);
require dirname(__DIR__) . '/autoload.php';

$door = new ThinkThen(getenv('TT_LIBRARY'));
try {
    try {
        echo $door->call(stream_get_contents(STDIN)), "\n";
    } catch (ThinkThenFailure $failure) {
        echo json_encode(['failed' => ['kind' => $failure->kind,
            'code' => $failure->nativeCode]], JSON_THROW_ON_ERROR), "\n";
    }
} finally {
    $door->close();
}

<?php
declare(strict_types=1);

require getenv('TT_AUTOLOAD');
$corpus = json_decode(file_get_contents(getenv('TT_PORTABLE_CORPUS')), true, 512, JSON_THROW_ON_ERROR);
if ($corpus['schema'] !== 'thinkthen.portable-batch-records/1' || count($corpus['texts']) !== 5)
    throw new RuntimeException('shared five-text corpus changed');
$door = new ThinkThen(getenv('TT_LIBRARY'), getenv('TT_PORTABLE_SETTINGS'));
try {
    $rows = $door->decideMany($corpus['question'], $corpus['texts']);
    if (count($rows['value']) !== count($corpus['texts'])) throw new RuntimeException('bulk row count changed');
    if ($rows['facts']['records'] !== 5 || $rows['facts']['requests_sent'] !== 3)
        throw new RuntimeException('portable bulk facts changed');
    foreach ($rows['value'] as $at => $answer) {
        if ($answer !== ['outcome' => 1, 'probability' => 0.9])
            throw new RuntimeException('bulk answer changed at ' . $at);
    }
    echo "PHP_PORTABLE_BATCH_PASS five ordered rows\n";
} finally {
    $door->close();
}

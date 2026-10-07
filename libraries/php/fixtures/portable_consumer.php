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
    if ($rows['facts']['records'] !== 5 || $rows['facts']['requests_sent'] !== 1)
        throw new RuntimeException('portable bulk facts changed');
    foreach ($rows['value'] as $at => $answer) {
        if ($answer !== ['outcome' => 1, 'probability' => 0.9])
            throw new RuntimeException('bulk answer changed at ' . $at);
    }
    echo "PHP_PORTABLE_BATCH_PASS five ordered rows\n";
} finally {
    $door->close();
}

$native = new ThinkThen\Native\Engine(getenv('TT_LIBRARY'), getenv('TT_PORTABLE_SETTINGS') ?: null);
try {
    $question = ThinkThen\Native\Question::spec(new ThinkThen\Native\QuestionSpec(
        ThinkThen\Native\FunctionKind::DECIDE, ThinkThen\Native\Content::text($corpus['question'])));
    $source = new ThinkThen\Native\Records(array_map(fn($text) => new ThinkThen\Native\Record(
        ThinkThen\Native\Content::text($text)), $corpus['texts']));
    $result = $native->decide($question, $source);
    if (count($result->rows) !== 5 || $result->summary->facts->value->requests_sent !== '1')
        throw new RuntimeException('installed complete rows/facts changed');
    foreach ($result->rows as $row) if ($row->value->data->boolean !== 1 || $row->common->answer->value->data->probability !== .9)
        throw new RuntimeException('installed complete answer changed');
    $token = $native->cancellation();
    try {
        $token->fire();
        try { $native->decide($question, $source, new ThinkThen\Native\Controls(cancel: $token)); throw new RuntimeException('spent token accepted'); }
        catch (ThinkThen\Native\CompleteFailure $failure) {
            if ($failure->kind() !== ThinkThen\Complete\FailureKind::CANCELLED || $failure->summary->facts->present !== 0) throw $failure;
        }
    } finally { $token->close(); }
} finally { $native->close(); }
if (strlen($result->rows[0]->common->answer_id->data) !== 64) throw new RuntimeException('installed complete copy expired');
try { new ThinkThen\Native\Engine(getenv('TT_LIBRARY'), '{"unknown":true}'); throw new RuntimeException('invalid settings accepted'); }
catch (ThinkThen\Native\CompleteFailure $failure) {
    if ($failure->kind() !== ThinkThen\Complete\FailureKind::USAGE || $failure->summary->facts->present !== 0) throw $failure;
}
echo "PHP_INSTALLED_COMPLETE_PASS five typed rows, cancellation and usage refuse before sending\n";

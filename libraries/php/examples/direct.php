<?php
declare(strict_types=1);
require dirname(__DIR__).'/autoload.php';
use ThinkThen\Client;
use ThinkThen\Inputs\{ChooseQuestion, RecognizeQuestion, RecognitionSpec, RelateQuestion, RelationSpec, RelationRule};

$client = new Client();
try {
    $answers = [
        'decide' => $client->decide('Does this ask for a refund?', 'Please refund my order.')->value,
        'choose' => $client->choose(new ChooseQuestion(choose: 'Which team?', options: ['billing', 'support']), 'Refund please.')->value,
        'tag' => $client->tag(['tag' => 'Which topics?', 'labels' => ['billing', 'support']], 'Refund please.')->value,
        'score' => $client->score(['score' => 'How urgent?', 'levels' => ['low', 'high']], 'Refund today.')->value,
        'filter' => $client->filter('Does this ask for a refund?', ['Refund please.', 'Thank you.'])->value,
        'rank' => $client->rank('Does this ask for a refund?', ['Refund now.', 'Thank you.'])->value,
        'find' => $client->find('Which line asks for a refund?', ['Refund please.', 'Thank you.'])->value,
        'annotate' => $client->annotate(['version' => 1, 'questions' => ['refund' => ['decide' => 'Does this ask for a refund?']]], ['Refund please.'])->value,
        'recognize' => $client->recognize(new RecognizeQuestion(new RecognitionSpec(kinds: ['person' => null])), 'Maria Chen')->value,
        'relate' => $client->relate(new RelateQuestion(new RelationSpec([new RelationRule(name: 'knows', source: 'person', target: 'person')])), [['name' => 'Ana', 'kind' => 'person'], ['name' => 'Bob', 'kind' => 'person']])->value,
    ];
    foreach ($answers as $function => $answer) echo $function, ': ', is_array($answer) ? count($answer).' results' : ($answer instanceof ThinkThen\Results\Node ? 'answered' : json_encode($answer, JSON_THROW_ON_ERROR)), "\n";
    foreach ($answers['relate'] as $edge) echo $edge->sourceName(), ' -> ', $edge->targetName(), "\n";
} finally { $client->close(); }

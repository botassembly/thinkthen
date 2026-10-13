<?php
declare(strict_types=1);
require dirname(__DIR__).'/autoload.php';
use ThinkThen\Client;
use ThinkThen\Inputs\{RecognizeQuestion, RecognitionSpec, RelateQuestion, RelationSpec, RelationRule};

$client = new Client();
try {
    $answers = [
        'decide' => $client->decide('Does this ask for a refund?', 'Please refund my order.')->decideAnswer(),
        'choose' => $client->choose(['choose' => 'Which team?', 'options' => ['billing', 'support']], 'Refund please.')->chooseAnswer(),
        'tag' => $client->tag(['tag' => 'Which topics?', 'labels' => ['billing', 'support']], 'Refund please.')->tagAnswer(),
        'score' => $client->score(['score' => 'How urgent?', 'levels' => ['low', 'high']], 'Refund today.')->scoreAnswer(),
        'filter' => $client->filter('Does this ask for a refund?', ['Refund please.', 'Thank you.'])->filterAnswer(),
        'rank' => $client->rank('Does this ask for a refund?', ['Refund now.', 'Thank you.'])->rankAnswer(),
        'find' => $client->find('Which line asks for a refund?', ['Refund please.', 'Thank you.'])->findAnswer(),
        'annotate' => $client->annotate(['version' => 1, 'questions' => ['refund' => ['decide' => 'Does this ask for a refund?']]], ['Refund please.'])->annotateAnswer(),
        'recognize' => $client->recognize(new RecognizeQuestion(new RecognitionSpec(kinds: ['person' => null])), 'Maria Chen')->recognizeAnswer(),
        'relate' => $client->relate(new RelateQuestion(new RelationSpec([new RelationRule(name: 'knows', source: 'person', target: 'person')])), [['name' => 'Ana', 'kind' => 'person'], ['name' => 'Bob', 'kind' => 'person']])->relateAnswer(),
    ];
    foreach ($answers as $function => $answer) echo $function, ': ', json_encode($answer instanceof ThinkThen\Results\Node ? $answer->toObject() : $answer, JSON_THROW_ON_ERROR), "\n";
    foreach ($answers['relate']->value as $edge) echo $edge->sourceName(), ' -> ', $edge->targetName(), "\n";
} finally { $client->close(); }

<?php
declare(strict_types=1);
require (getenv('THINKTHEN_PHP_PACKAGE') ?: dirname(__DIR__)).'/autoload.php';
use ThinkThen\{Client, CallFailure, Cancellation};

function consume(stdClass $fixture, Client $client): array
{
    $verb = $fixture->verb;
    $root = getenv('TT_REPO');
    $question = $fixture->question;
    if (isset($fixture->loader)) $question = match ($fixture->loader) {
        'file', 'load' => Client::questionFile($fixture->reference),
        'named', 'load_named' => Client::questionNamed($fixture->reference),
        'reference', 'load_reference' => Client::questionReference($fixture->reference),
    };
    elseif (($fixture->question_form ?? null) === 'file') $question = Client::questionFile('fixture-question.json');
    elseif (isset($fixture->raw)) {
        $question = Client::questionJson($fixture->raw);
    }
    $none = $verb === 'find' && ($question->none ?? false);
    if ($verb === 'find' && $question instanceof stdClass) unset($question->none);
    $injection = $fixture->operation->injection ?? null;
    if (isset($fixture->paths) || $injection === 'recording_read_failure') {
        $unit = match ($fixture->source_unit ?? 3) { 1, 5 => 'line', 2 => 'window', default => 'file' };
        $reading = ['unit' => $unit];
        if (!empty($fixture->window)) $reading['window'] = $fixture->window;
        $input = Client::files(array_map(fn($p) => ($fixture->owned_jsonl ?? false) ? $p : $root.'/'.$p,
            $fixture->paths ?? ['target/missing-input']), $reading,
            ($fixture->owned_jsonl ?? false) || ($fixture->source_unit ?? 0) === 5 ? 'jsonl' : null,
            ($fixture->image_reader ?? false) || ($fixture->source_unit ?? 0) === 4 ? 'image' : null);
    } else {
        $images = array_map(fn($p) => ['kind' => 'file', 'path' => $root.'/'.$p, 'media' => $fixture->media ?? 'image/png'], $fixture->image_paths ?? []);
        $input = [];
        foreach ($fixture->items as $i => $value) {
            $fields = [];
            if ($images) $fields['images'] = $images;
            if (isset($fixture->contexts)) $fields['context'] = $fixture->contexts[$i];
            elseif ($fixture->context_present ?? false) $fields['context'] = $fixture->context;
            if (isset($fixture->candidate_orders)) $fields['options'] = array_map(fn($name) => ['name' => $name], $fixture->candidate_orders[$i]);
            if ($fixture->caption_files ?? false) $value = file_get_contents('caption-'.$i.'.txt');
            $input[] = ($fixture->image_only ?? false) ? Client::images($images) : Client::item($value, $fields);
        }
    }
    if (($fixture->incremental ?? false) && !isset($fixture->paths)) {
        $records = $input;
        $input = (function () use ($records) { yield from $records; })();
    }
    $options = ['attempts' => true];
    if ($none) $options['none'] = true;
    if ($injection === 'expired_deadline') $options['deadline_ms'] = 0;
    if (isset($fixture->shared_context)) $options['context'] = $fixture->shared_context;
    $cancel = new Cancellation();
    if ($injection === 'cancel_token') $cancel->cancel();
    try {
        if ($fixture->held_cancel ?? false) {
            $operation = $client->start($verb, $question, $input, $options);
            $packets = []; $signal = getenv('HOME').'/cancel';
            try {
                do {
                    if (file_exists($signal)) { $operation->cancel(); touch($signal.'.fired'); }
                    $packet = $operation->poll();
                    if ($packet !== null) $packets[] = $packet;
                    usleep(1000);
                } while ($packet?->kind !== 'terminal');
            } finally { $operation->close(); }
        } else $packets = $client->$verb($question, $input, $options, $cancel)->packets;
    } catch (CallFailure $error) {
        if (!$error->packets) throw $error;
        $packets = $error->packets;
    }
    return ['packets' => array_map(fn($p) => $p->toObject(), $packets)];
}
$client = null;
try {
    $fixture = json_decode(file_get_contents($argv[1]), flags: JSON_THROW_ON_ERROR);
    $client = new Client(json_decode($argv[3], true, flags: JSON_THROW_ON_ERROR), $argv[2]);
    $payload = consume($fixture, $client);
} catch (CallFailure $error) {
    $kind = match (get_class($error)) {
        ThinkThen\UsageFailure::class => ThinkThen\Session\THINKTHEN_EUSAGE, ThinkThen\BackendFailure::class => ThinkThen\Session\THINKTHEN_EBACKEND, ThinkThen\DeadlineFailure::class => ThinkThen\Session\THINKTHEN_EDEADLINE,
        ThinkThen\LocalFailure::class => ThinkThen\Session\THINKTHEN_ELOCAL, ThinkThen\CancelledFailure::class => ThinkThen\Session\THINKTHEN_ECANCELLED, default => ThinkThen\Session\THINKTHEN_EDEFECT,
    };
    $payload = ['admission' => ['code' => $kind, 'message' => $error->getMessage()]];
} finally { $client?->close(); }
echo json_encode($payload, JSON_THROW_ON_ERROR | JSON_UNESCAPED_UNICODE), "\n";

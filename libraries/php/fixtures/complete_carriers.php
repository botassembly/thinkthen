<?php
declare(strict_types=1);
require dirname(__DIR__) . '/src/complete/bootstrap.php';
use ThinkThen\Complete as T;
function check(bool $ok, string $behavior): void { if (!$ok) throw new RuntimeException($behavior); }
function rejects(callable $read, string $behavior): void {
    try { $read(); } catch (UnexpectedValueException|InvalidArgumentException $e) {
        check(!str_contains($e->getMessage(),'secret-sentinel'), 'failure diagnostic is redacted'); return;
    }
    throw new RuntimeException($behavior);
}
$f=json_decode(file_get_contents(__DIR__.'/complete.json'),false,512,JSON_THROW_ON_ERROR|JSON_BIGINT_AS_STRING);
foreach ($f->results as $verb=>$json) {
    $r=T\readModel(ucfirst($verb).'Result',$json);
    check(T\Requests::project($r)==$json,'all known result fields round trip: '.$verb);
}
$decide=T\DecideResult::fromJson($f->results->decide);
check($decide->value instanceof T\BooleanDecision && $decide->value->value===false && $decide->input->present && $decide->input->value===null,'false and present null');
$authored=T\DecideResult::fromJson($f->authored_decision);
check($authored->value instanceof T\AuthoredDecision && $authored->value->meaning->value->accepted===false,'structured authored decision remains authored');
$authoredNull=clone $f->authored_decision; $authoredNull->value=null;
rejects(fn()=>T\DecideResult::fromJson($authoredNull),'authored null requires actual C discriminator');
check((new T\AuthoredDecision(T\JsonContent::fromJson(null)))->meaning->value===null,'native adapter can carry authored null without guessing');
check($decide->threshold->low===0.1 && $decide->position->value->last->value===4,'band and physical line window');
$choose=T\ChooseResult::fromJson($f->results->choose);
check($choose->value===null && $choose->answer->probabilities->entries[0]->name==='0','numeric option names remain strings');
check($choose->answer->probabilities->get('0')===0.9 && $choose->answer->confidence->value===0.8,'full ordered probabilities and confidence');
check(!T\FindResult::fromJson($f->results->find)->answer->confidence->present,'omitted confidence differs from zero');
$annotation=T\AnnotateResult::fromJson($f->results->annotate);
check($annotation->value->get('0') instanceof T\BooleanValue && $annotation->value->get('unresolved') instanceof T\Unresolved,'typed annotation values');
check($annotation->answers->get('0') instanceof T\AnnotationSuccess && $annotation->answers->get('fail') instanceof T\AnnotationFailure,'annotation state discriminants');
check($annotation->answers->get('fail')->failure->cause===T\CauseKind::MISSING_PROBABILITY,'typed member cause');
$recognition=T\RecognizeResult::fromJson($f->results->recognize);
check($recognition->value->entities[0]->start===1 && $recognition->value->entities[0]->length===2,'Unicode scalar span remains native');
check($recognition->answer->names[0]->kinds===null && $recognition->answer->names[0]->edges->entries===[],'null map and empty map');
$relations=T\RelateResult::fromJson($f->results->relate);
check($relations->value[0]->source->record->value->ok===false && $relations->value[0]->target->first_line->value===2,'located endpoint original payload');
check($relations->answer->questions[0]->target===null && $relations->answer->questions[0]->accepted===false,'relation target absence and false acceptance');
$facts=T\Facts::fromJson($f->facts);
check($facts->estimated_cost_usd->value==='12345678901234567890.000001' && $facts->command_ms->value===0,'exact cost and zero timing');
check(!$facts->input_tokens->present,'unreported usage remains absent');
$attempt=T\Attempt::fromJson($f->attempt);
check($attempt->sdk_request_id instanceof T\SdkRequestId && $attempt->server_ms->value===0,'typed attempt ID and reported zero server time');
foreach ($f->errors as $e) check(T\CallError::fromJson($e)->facts->value->call_id instanceof T\CallId,'six started failures retain typed final facts');
foreach ($f->negative as $bad) rejects(fn()=>T\Read::value($bad->type,$bad->value),'reject invalid '.$bad->type);
foreach ($f->requests as $case) {
    $q=T\readModel($case->question_type,$case->question);$input=T\readModel($case->input_type,$case->input);
    $request=T\Requests::{$case->verb}($q,$input,T\Controls::fromJson($case->controls));
    check(T\Requests::project($request)==$case->expected,'independent named builder expectation: '.$case->verb);
}
$defaults=T\RecognitionSpec::fromJson(json_decode('{"version":1,"recognize":{"relations":[{"name":"knows"}]}}'));
check(!T\Requests::recognize($defaults,new T\TextInput(T\Authored::fromJson('A knows B')))->question->recognize->relations->value[0]->source->present,'native any-kind relation sides stay absent');
check(T\Controls::fromJson(json_decode('{"deadline_ms":-1}'))->deadline_ms->value===-1,'explicit unlimited deadline is retained');
$view=new T\ImageView(T\MediaType::PNG,"\x00\xff",1,1);
check($view->bytes==="\x00\xff" && !$view->filename->present,'copied image view retains binary bytes and absent name');
$question=new T\QuestionFile('/explicit/questions.json');
$files=new T\Files(['same.txt','same.txt'],T\UnitKind::WINDOW,new T\Optional(true,2));
$request=T\Requests::decide($question,$files);
check($request->input->paths===['same.txt','same.txt'],'duplicate file occurrences survive');
try { $request->questionJson(); throw new RuntimeException('host loaded a question file'); }
catch (LogicException $e) { check($e->getMessage()==='question files require the native loader','question file remains explicit'); }
$image=new T\ImageInput([new T\ImageBytes("\xff\x00", T\MediaType::PNG),new T\ImageBytes("\x01", T\MediaType::PNG),new T\ImageBytes("\xff\x00", T\MediaType::PNG)]);
foreach (['decide','choose','score'] as $verb) {
    $q=match($verb) {'decide'=>new T\DecideSpec(T\Authored::fromJson('Is it?')),'choose'=>new T\ChooseSpec(T\Authored::fromJson('Which?'),T\Labels::fromJson(['a','b'])),'score'=>new T\ScoreSpec(T\Authored::fromJson('How?'),T\Labels::fromJson(['low','high']))};
    check(T\Requests::{$verb}($q,$image)->input->images[2]->data==="\xff\x00",'binary image order and duplicates');
}
foreach (['tag','filter','rank','find','annotate','recognize','relate'] as $verb) {
    $case=current(array_filter($f->requests,fn($c)=>$c->verb===$verb));$q=T\readModel($case->question_type,$case->question);
    rejects(fn()=>T\Requests::{$verb}($q,$image),'seven explicit image builder refusals');
    rejects(fn()=>T\Requests::{$verb}($q,new T\Files(['a.png'],T\UnitKind::FILE,media:new T\Optional(true,T\MediaKind::IMAGE))),'seven image-source builder refusals');
}
rejects(fn()=>T\Requests::decide(new T\DecideSpec(T\Authored::fromJson('Is it?')),new T\Files(['x'],T\UnitKind::LINE,new T\Optional(true,2))),'line units refuse a window');
$max=json_decode('{"call_id":"'.str_repeat('a',64).'","records":9223372036854775807,"requests_sent":0,"cache_answers":0,"seconds":0}');
check(T\Facts::fromJson($max)->records===PHP_INT_MAX,'signed 64-bit boundary');
$overflow=json_decode(str_replace('9223372036854775807','9223372036854775808',json_encode($max)),false,512,JSON_BIGINT_AS_STRING);
rejects(fn()=>T\Facts::fromJson($overflow),'no count truncation above signed 64-bit');
$call=T\CompleteCall::fromJson((object)['value'=>$f->results->decide,'facts'=>$f->facts,'attempts'=>[]],T\DecideResult::fromJson(...));
check($call->value->value instanceof T\BooleanDecision && $call->value->value->value===false && $call->attempts->present && $call->attempts->value===[],'typed call envelope and requested zero attempts');
ob_start();var_dump($request,$decide,$facts);$debug=ob_get_clean();
check(!str_contains($debug,'explicit/questions') && !str_contains($debug,'12345678901234567890'),'carrier debug withholds content');
echo "PHP private carrier and builder consumers PASS; native complete parity pending\n";

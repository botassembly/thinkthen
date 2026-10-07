<?php
declare(strict_types=1);
require dirname(__DIR__).'/autoload.php';
use ThinkThen\Native\{Engine,Question,QuestionSpec,FunctionKind,Content,Choice,Member,Relation,Records,Record,Author,Declaration,Property,PropertyKind};
$e=new Engine($argv[1],$argv[2]);
try {
    $decide=Question::spec(new QuestionSpec(FunctionKind::DECIDE,Content::text('Does this ask for a refund?'),
        author:new Author('refund','2147483647',Declaration::string())));
    $specs=[
        'decide'=>$decide,
        'choose'=>Question::spec(new QuestionSpec(FunctionKind::CHOOSE,Content::text('Which?'),choices:[new Choice('a',Content::json((object)['what'=>'First'])),new Choice('b')])),
        'tag'=>Question::spec(new QuestionSpec(FunctionKind::TAG,Content::text('Labels?'),choices:[new Choice('a'),new Choice('b')])),
        'score'=>Question::spec(new QuestionSpec(FunctionKind::SCORE,Content::text('Grade?'),choices:[new Choice('low'),new Choice('high')])),
        'filter'=>$decide,'rank'=>$decide,
        'find'=>Question::spec(new QuestionSpec(FunctionKind::FIND,Content::text('Which text?'),none:true)),
        'annotate'=>Question::spec(new QuestionSpec(FunctionKind::ANNOTATE,members:[new Member('refund',$decide)])),
        'recognize'=>Question::spec(new QuestionSpec(FunctionKind::RECOGNIZE,kinds:[new Choice('person'),new Choice('place')])),
        'relate'=>Question::spec(new QuestionSpec(FunctionKind::RELATE,relations:[new Relation('visits','person','place')])),
    ];
    $results=[];
    foreach($specs as $verb=>$q){
        $source=$verb==='relate'?new Records([new Record(Content::json((object)['name'=>'Alice','kind'=>'person'])),new Record(Content::json((object)['name'=>'Paris','kind'=>'place']))]):new Records($verb==='find'?[new Record(Content::text('Alice visited Paris.')),new Record(Content::text('Refund me.'))]:[new Record(Content::text('Refund me please.'))]);
        $r=$e->$verb($q,$source);
        if($r->summary->state!==1 || !$r->summary->facts->present || !$r->rows || strlen($r->rows[0]->common->answer_id->data)!==64)throw new RuntimeException('typed constructor did not execute');
        if($verb==='decide' && $r->authors[0]->wording_version->value!=='2147483647')throw new RuntimeException('unsigned wording version changed');
        $results[$verb]=$r;
    }
    $e->close();
    foreach($results as $r)if(strlen($r->rows[0]->common->meta->requests->data[0]->data)!==64)throw new RuntimeException('copied result expired');
    echo "ten typed constructors executed; copied values survive close; wording version exact\n";
} catch(Throwable $x){fwrite(STDERR,get_class($x).': '.$x->getMessage()."\n");exit(1);} finally {$e->close();}

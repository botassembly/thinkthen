<?php
declare(strict_types=1);
require_once (getenv('THINKTHEN_PHP_PACKAGE') ?: dirname(__DIR__)).'/autoload.php';
use ThinkThen\Native\{Engine,Question,LoaderRole,Records,Record,Content,Files,FileUnit,Image,Choice,Controls,CompleteFailure};

/** Exercise actual named public typed methods. Only arbitrary input content uses JSON. */
function consume(array $v,string $library,string $settings): array
{
    $e=null;$token=null;$batch=null;
    try {
        $e=new Engine($library,$settings);
        $verb=$v['verb'];$role=match($verb){'rank'=>isset($v['question']['questions'])?LoaderRole::RANK_SET:LoaderRole::RANK,'annotate'=>LoaderRole::SET,'find'=>LoaderRole::FIND,'recognize'=>LoaderRole::RECOGNIZE,'relate'=>LoaderRole::RELATE,default=>LoaderRole::ATOMIC};
        $q=isset($v['loader'])?match($v['loader']){'load','file'=>Question::file($v['reference']),'load_named','named'=>Question::named($role,$v['reference']),'load_reference','reference'=>Question::reference($role,$v['reference'])}:
            (($v['question_form']??null)==='file'?Question::file('fixture-question.json'):Question::saved($role,$v['raw']??json_encode($v['question'],JSON_THROW_ON_ERROR|JSON_UNESCAPED_SLASHES|JSON_UNESCAPED_UNICODE)));
        if($verb==='find' && ($v['question']['none']??false)){$text=$v['question']['find'];$q=Question::find(is_string($text)?Content::text($text):Content::json($text),true,$v['metadata']['name']??null,$v['metadata']['wording_version']??null);}
        if(($v['paths']??null)!==null || ($v['operation']['injection']??null)==='recording_read_failure') {
            $paths=array_map(fn($p)=>($v['owned_jsonl']??false)?$p:dirname(__DIR__,3).'/'.$p,$v['paths']??['target/0429-missing-input']);
            $s=new Files($paths,FileUnit::from($v['source_unit']??3),$v['window']??0,$v['image_reader']??false);
        } else {
            $images=[];foreach($v['image_paths']??[] as $p)$images[]=new Image(file_get_contents(dirname(__DIR__,3).'/'.$p),($v['media']??null)==='image/jpeg'?1:2);
            $items=[];foreach($v['items'] as $i=>$item) {
                $original=($v['image_only']??false)?null:(($v['text']??false)&&is_string($item)?Content::text($item):Content::json($item));
                if($v['caption_files']??false)$original=Content::text(file_get_contents('caption-'.$i.'.txt'));
                $context=($v['context_present']??false)?(is_string($v['context'])?Content::text($v['context']):Content::json($v['context'])):null;
                $options=array_map(fn($name)=>new Choice($name),$v['candidate_orders'][$i]??[]);
                $items[]=new Record($original,$context,$options,$images);
            }
            $s=new Records($items);
        }
        $injection=$v['operation']['injection']??null;
        if($injection==='cancel_token' || ($v['held_cancel']??false)){$token=$e->cancellation();if($injection==='cancel_token')$token->fire();}
        if($v['held_cancel']??false) {
            // Test-only native thread fires the public token on the fixture's release signal.
            $helper=FFI::cdef('void *cancel_reader(void *, const char *, const char *); void cancel_join(void *);',getenv('TT_CANCEL_HELPER'));
            $thread=$helper->cancel_reader($token->pointer,$library,getenv("HOME")."/cancel");
        }
        $ctx=isset($v['shared_context'])?(is_string($v['shared_context'])?Content::text($v['shared_context']):Content::json($v['shared_context'])):null;
        $c=new Controls(deadlineMs:$injection==='expired_deadline'?0:-1,cancel:$token,context:$ctx,attempts:true);
        if($v['incremental']??false) {
            $method=$verb.'Batch';$batch=$e->$method($q,$s,$c);$prefix=[];
            try { while(($r=$batch->next())!==null)$prefix[]=$r; $r=$batch->facts(); }
            catch(CompleteFailure $x) { return ['failure'=>$x->summary,'completed'=>$prefix]; }
            return ['result'=>$r,'completed'=>$prefix];
        }
        return ['result'=>$e->$verb($q,$s,$c)];
    } catch(CompleteFailure $x) { return ['failure'=>$x->summary]; }
    finally { if(isset($thread))$helper->cancel_join($thread);$batch?->close();$token?->close();$e?->close(); }
}
function encodeNative(mixed $v): mixed
{
    if($v instanceof ThinkThen\Native\ImageView){$out=get_object_vars($v);$out['bytes']=bin2hex($v->bytes);return array_map(encodeNative(...),$out);}
    if(is_object($v))return array_map(encodeNative(...),get_object_vars($v));
    if(is_array($v))return array_map(encodeNative(...),$v);
    return $v;
}
$document=json_decode(file_get_contents($argv[1]),true,512,JSON_THROW_ON_ERROR);
try { echo json_encode(encodeNative(consume($document,$argv[2],$argv[3])),JSON_THROW_ON_ERROR|JSON_UNESCAPED_UNICODE|JSON_INVALID_UTF8_SUBSTITUTE),"\n"; } catch(Throwable $error) { fwrite(STDERR,get_class($error).": ".$error->getMessage()." at ".basename($error->getFile()).":".$error->getLine()."\n");exit(1); }

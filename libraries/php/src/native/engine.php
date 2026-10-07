<?php
declare(strict_types=1);
namespace ThinkThen\Native;

/** One engine, endpoint, native reader/cache/scheduler. Released ThinkThen methods remain available. */
final class Engine
{
    use QuestionAdapter;
    private \FFI $ffi;
    private \FFI\CData $engine;
    private bool $closed=false;
    private int $batches=0;
    public function __construct(string $absoluteLibrary,?string $settingsJson=null,private readonly string $surface='php')
    {
        if(!str_starts_with($absoluteLibrary,'/')) throw new \InvalidArgumentException('absolute library path required');
        if($surface!=='php') throw new \InvalidArgumentException('PHP surface required');
        $this->ffi=\FFI::cdef(file_get_contents(__DIR__.'/abi.h'),$absoluteLibrary);
        if($settingsJson!==null && str_contains($settingsJson,"\0")) throw new \InvalidArgumentException('interior NUL');
        $e=$settingsJson===null?$this->ffi->thinkthen_engine_new():$this->ffi->thinkthen_engine_new_with($settingsJson);
        if($e===null || \FFI::isNull($e)) { $this->engine=$this->ffi->cast('thinkthen_engine *',0); $this->fail(); }
        $this->engine=$e;
    }
    private function live(): void { if($this->closed) throw new \LogicException('engine closed'); }
    public function close(): void
    {
        if($this->batches) throw new \LogicException('close batches before engine');
        if(!$this->closed) { $this->ffi->thinkthen_engine_free($this->engine); $this->closed=true; }
    }
    public function __destruct() { if(!$this->closed && !$this->batches) $this->close(); }
    public function cancellation(): Cancellation { $this->live(); return new Cancellation($this->ffi,$this->ffi->thinkthen_cancel_token_new()); }
    private function check(int $code): void { if($code) $this->fail(); }
    private function fail(): never
    {
        $r=$this->ffi->new('thinkthen_result *');
        if($this->ffi->thinkthen_error_complete($this->engine,\FFI::addr($r)) || \FFI::isNull($r)) throw new \LogicException('native error snapshot unavailable');
        try { $s=$this->ffi->new('thinkthen_summary_v1'); if($this->ffi->thinkthen_result_summary($r,\FFI::addr($s))) throw new \LogicException('native failure summary'); throw new CompleteFailure(SummaryView::copy($s)); }
        finally { $this->ffi->thinkthen_result_free($r); }
    }
    private function string(string $value,array &$keep): \FFI\CData
    {
        $s=$this->ffi->new('thinkthen_string_v1'); $b=$this->ffi->new('char['.max(1,strlen($value)).']');
        if($value!=='') \FFI::memcpy($b,$value,strlen($value)); $keep[]=$b;
        $s->data=\FFI::addr($b[0]); $s->len=strlen($value); return $s;
    }
    private function content(Content $v,array &$keep): \FFI\CData
    {
        $c=$this->ffi->new('thinkthen_content_v1'); $c->kind=$v->kind; $c->data=$this->string($v->bytes,$keep); return $c;
    }
    private function optionalContent(?Content $v,array &$keep): \FFI\CData
    {
        $c=$this->ffi->new('thinkthen_optional_content_v1'); if($v!==null) { $c->present=1;$c->value=$this->content($v,$keep); } return $c;
    }
    private function question(Question $v,array &$keep): \FFI\CData
    {
        if($v->spec!==null)return $this->constructed($v->spec,$keep);
        $out=$this->ffi->new('thinkthen_question *'); $value=$this->string($v->value,$keep);
        if($v->method==='new') {
            $spec=$this->ffi->new('thinkthen_question_spec_v1');$spec->kind=7;$spec->text=$this->content($v->text,$keep);$spec->none=(int)$v->none;
            $author=$this->ffi->new('thinkthen_question_author_v1');
            if($v->name!==null){$author->name->present=1;$author->name->value=$this->string($v->name,$keep);}
            if($v->wordingVersion!==null){$author->wording_version->present=1;$author->wording_version->value=$v->wordingVersion;}
            $this->check($this->ffi->thinkthen_question_new_authored($this->engine,\FFI::addr($spec),\FFI::addr($author),\FFI::addr($out)));return $out;
        }
        $method='thinkthen_question_'.$v->method;
        $this->check($v->method==='load'?$this->ffi->$method($this->engine,$value,\FFI::addr($out)):
            $this->ffi->$method($this->engine,$v->role->value,$value,\FFI::addr($out)));
        return $out;
    }
    private function source(Source $v,array &$keep,array &$images): \FFI\CData
    {
        $out=$this->ffi->new('thinkthen_source *');
        if($v instanceof Files) {
            $paths=$this->ffi->new('thinkthen_string_v1['.max(1,count($v->paths)).']'); $keep[]=$paths;
            foreach($v->paths as $i=>$p) $paths[$i]=$this->string($p,$keep);
            $s=$this->ffi->new('thinkthen_source_spec_v1');$s->paths->data=\FFI::addr($paths[0]);$s->paths->len=count($v->paths);$s->unit=$v->unit->value;$s->window=$v->window;
            $method=$v->imageReader?'thinkthen_source_image_files':'thinkthen_source_files';$this->check($this->ffi->$method($this->engine,\FFI::addr($s),\FFI::addr($out)));
        } elseif($v instanceof Records) {
            $records=$this->ffi->new('thinkthen_record_v1['.max(1,count($v->records)).']');$keep[]=$records;
            foreach($v->records as $i=>$r) {
                if(!$r instanceof Record) throw new \InvalidArgumentException('typed record required');
                $records[$i]->original=$this->optionalContent($r->original,$keep);$records[$i]->context=$this->optionalContent($r->context,$keep);
                $opts=$this->ffi->new('thinkthen_choice_v1['.max(1,count($r->options)).']');$keep[]=$opts;
                foreach($r->options as $j=>$c) { if(!$c instanceof Choice) throw new \InvalidArgumentException('typed choice required');$opts[$j]->name=$this->string($c->name,$keep);$opts[$j]->description=$this->optionalContent($c->description,$keep);if($c->weight!==null) {$opts[$j]->weight->present=1;$opts[$j]->weight->value=$c->weight;} }
                $records[$i]->options->data=\FFI::addr($opts[0]);$records[$i]->options->len=count($r->options);
                $ptrs=$this->ffi->new('thinkthen_image *['.max(1,count($r->images)).']');$keep[]=$ptrs;
                foreach($r->images as $j=>$im) {
                    if(!$im instanceof Image) throw new \InvalidArgumentException('typed image required');
                    $bytes=$this->string($im->bytes,$keep);$name=$this->ffi->new('thinkthen_optional_string_v1');
                    if($im->filename!==null){$name->present=1;$name->value=$this->string($im->filename,$keep);}
                    $handle=$this->ffi->new('thinkthen_image *');$this->check($this->ffi->thinkthen_image_clone($this->engine,$this->ffi->cast('uint8_t *',$bytes->data),$bytes->len,$im->media,$name,\FFI::addr($handle)));$images[]=$handle;$ptrs[$j]=$handle;
                }
                $records[$i]->images->data=\FFI::addr($ptrs[0]);$records[$i]->images->len=count($r->images);
            }
            $this->check($this->ffi->thinkthen_source_records($this->engine,$records,count($v->records),\FFI::addr($out)));
        } else throw new \InvalidArgumentException('typed source required');
        return $out;
    }
    private function controls(Controls $v,array &$keep): \FFI\CData
    {
        $c=$this->ffi->new('thinkthen_controls_v1');$c->deadline_ms=$v->deadlineMs;$c->cancel=$v->cancel?->live();$c->context=$this->optionalContent($v->context,$keep);
        if($v->batch!==null){$c->batch->present=1;$c->batch->value=$v->batch;}$c->batch_max=(int)$v->batchMax;$c->attempts=(int)$v->attempts;$c->surface=$this->string($this->surface,$keep);return $c;
    }
    private function view(\FFI\CData $r,string $type,string $method,?int $at=null): mixed
    {
        $v=$this->ffi->new('thinkthen_'.$type.'_v1');$code=$at===null?$this->ffi->$method($r,\FFI::addr($v)):$this->ffi->$method($r,$at,\FFI::addr($v));
        if($code) throw new \LogicException('native result accessor refused');
        $class=__NAMESPACE__.'\\'.str_replace(' ','',ucwords(str_replace('_',' ',$type))).(str_ends_with($type,'_view')?'':'View');return $class::copy($v);
    }
    public function copyResult(\FFI\CData $r,string $verb): CompleteResult
    {
        $summary=$this->view($r,'summary','thinkthen_result_summary');$rows=[];$details=[];$authors=[];$memberAuthors=[];$rankMembers=[];$rankMemberDetails=[];$recognitions=[];$relations=[];
        for($i=0;$i<$summary->count;++$i) {
            $rows[]=$this->view($r,$verb.'_view','thinkthen_result_'.$verb,$i);
            $details[]=$this->view($r,'details','thinkthen_result_details',$i);$authors[]=$this->view($r,'question_author','thinkthen_result_question_author',$i);
            $ma=[];if($verb==='annotate')foreach($rows[$i]->answers->data as $j=>$_){$v=$this->ffi->new('thinkthen_question_author_v1');if($this->ffi->thinkthen_result_member_author($r,$i,$j,\FFI::addr($v)))throw new \LogicException('native member author');$ma[]=QuestionAuthorView::copy($v);}
            $rm=[];$rd=[];if($verb==='rank'){$n=$this->ffi->new('size_t');if($this->ffi->thinkthen_result_rank_member_count($r,$i,\FFI::addr($n)))throw new \LogicException('native rank member count');for($j=0;$j<$n->cdata;++$j){$v=$this->ffi->new('thinkthen_rank_view_v1');if($this->ffi->thinkthen_result_rank_member($r,$i,$j,\FFI::addr($v)))throw new \LogicException('native rank member');$rm[]=RankView::copy($v);$a=$this->ffi->new('thinkthen_question_author_v1');if($this->ffi->thinkthen_result_member_author($r,$i,$j,\FFI::addr($a)))throw new \LogicException('native rank member author');$ma[]=QuestionAuthorView::copy($a);$d=$this->ffi->new('thinkthen_details_v1');if($this->ffi->thinkthen_result_rank_member_details($r,$i,$j,\FFI::addr($d)))throw new \LogicException('native rank member details');$rd[]=DetailsView::copy($d);}}$memberAuthors[]=$ma;$rankMembers[]=$rm;$rankMemberDetails[]=$rd;
            $recognitions[]=$verb==='recognize'?$this->view($r,'source_recognition','thinkthen_result_source_recognition',$i):null;
            $relations[]=$verb==='relate'?$this->view($r,'source_relations','thinkthen_result_source_relations',$i):null;
        }
        $obs=[];$od=[];$oa=[];for($i=0;$i<$summary->observation_count;++$i){$obs[]=$this->view($r,'observation','thinkthen_result_observation',$i);$od[]=$this->view($r,'details','thinkthen_result_observation_details',$i);$oa[]=$this->view($r,'question_author','thinkthen_result_observation_author',$i);}
        return new CompleteResult($summary,$rows,$obs,$details,$authors,$memberAuthors,$rankMembers,$rankMemberDetails,$od,$oa,$recognitions,$relations);
    }
    private function execute(string $verb,Question $question,Source $input,?Controls $controls,bool $lazy=false): CompleteResult|Batch
    {
        $this->live();$keep=[];$images=[];$q=null;$s=null;$r=$this->ffi->new('thinkthen_result *');
        try {
            $q=$this->question($question,$keep);$s=$this->source($input,$keep,$images);$c=$this->controls($controls??new Controls(),$keep);
            if($lazy){$b=$this->ffi->new('thinkthen_batch *');$method='thinkthen_'.$verb.'_batch_start';$this->check($this->ffi->$method($this->engine,$q,$s,\FFI::addr($c),\FFI::addr($b)));++$this->batches;return new Batch($this,$this->ffi,$b,$verb);}
            $method='thinkthen_'.$verb.'_complete';$this->check($this->ffi->$method($this->engine,$q,$s,\FFI::addr($c),\FFI::addr($r)));return $this->copyResult($r,$verb);
        } finally { $this->ffi->thinkthen_result_free($r);if($s!==null)$this->ffi->thinkthen_source_free($s);if($q!==null)$this->ffi->thinkthen_question_free($q);foreach($images as $im)$this->ffi->thinkthen_image_free($im); }
    }
    public function batchClosed(): void { --$this->batches; }
    public function batchFailure(): never { $this->fail(); }
    /** @return CompleteResult<DecideView> */
    public function decide(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("decide",$question,$input,$controls); }
    /** @return CompleteResult<ChooseView> */
    public function choose(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("choose",$question,$input,$controls); }
    /** @return CompleteResult<TagView> */
    public function tag(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("tag",$question,$input,$controls); }
    /** @return CompleteResult<ScoreView> */
    public function score(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("score",$question,$input,$controls); }
    /** @return CompleteResult<FilterView> */
    public function filter(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("filter",$question,$input,$controls); }
    /** @return CompleteResult<RankView> */
    public function rank(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("rank",$question,$input,$controls); }
    /** @return CompleteResult<FindView> */
    public function find(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("find",$question,$input,$controls); }
    /** @return CompleteResult<AnnotateView> */
    public function annotate(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("annotate",$question,$input,$controls); }
    /** @return CompleteResult<RecognizeView> */
    public function recognize(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("recognize",$question,$input,$controls); }
    /** @return CompleteResult<RelateView> */
    public function relate(Question $question,Source $input,?Controls $controls=null): CompleteResult { return $this->execute("relate",$question,$input,$controls); }
    public function decideBatch(Question $question,Source $input,?Controls $controls=null): Batch { return $this->execute("decide",$question,$input,$controls,true); }
    public function chooseBatch(Question $question,Source $input,?Controls $controls=null): Batch { return $this->execute("choose",$question,$input,$controls,true); }
    public function tagBatch(Question $question,Source $input,?Controls $controls=null): Batch { return $this->execute("tag",$question,$input,$controls,true); }
    public function scoreBatch(Question $question,Source $input,?Controls $controls=null): Batch { return $this->execute("score",$question,$input,$controls,true); }
    public function filterBatch(Question $question,Source $input,?Controls $controls=null): Batch { return $this->execute("filter",$question,$input,$controls,true); }
    public function annotateBatch(Question $question,Source $input,?Controls $controls=null): Batch { return $this->execute("annotate",$question,$input,$controls,true); }
}

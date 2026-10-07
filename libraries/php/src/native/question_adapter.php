<?php
declare(strict_types=1);
namespace ThinkThen\Native;

/** Internal counted-input copying shared by typed constructors. */
trait QuestionAdapter
{
    private function choices(array $values,array &$keep): \FFI\CData
    {
        $a=$this->ffi->new('thinkthen_choice_v1['.max(1,count($values)).']');$keep[]=$a;
        foreach($values as $i=>$v){if(!$v instanceof Choice)throw new \InvalidArgumentException('typed choice required');$a[$i]->name=$this->string($v->name,$keep);$a[$i]->description=$this->optionalContent($v->description,$keep);if($v->weight!==null){$a[$i]->weight->present=1;$a[$i]->weight->value=$v->weight;}}
        $out=$this->ffi->new('thinkthen_choices_v1');$out->data=\FFI::addr($a[0]);$out->len=count($values);return $out;
    }
    private function stringList(array $values,array &$keep): \FFI\CData
    {
        $a=$this->ffi->new('thinkthen_string_v1['.max(1,count($values)).']');$keep[]=$a;
        foreach($values as $i=>$v)$a[$i]=$this->string($v,$keep);
        $out=$this->ffi->new('thinkthen_strings_v1');$out->data=\FFI::addr($a[0]);$out->len=count($values);return $out;
    }
    private function optionalString(?string $v,array &$keep): \FFI\CData
    {
        $out=$this->ffi->new('thinkthen_optional_string_v1');if($v!==null){$out->present=1;$out->value=$this->string($v,$keep);}return $out;
    }
    private function rule(?Rule $v): \FFI\CData
    {
        $out=$this->ffi->new('thinkthen_rule_v1');if($v!==null){$out->kind=$v->kind;$out->low=$v->low;$out->high=$v->high;}return $out;
    }
    private function declaration(?Declaration $v,array &$keep): \FFI\CData
    {
        $out=$this->ffi->new('thinkthen_input_declaration_v1');if($v===null)return $out;
        $out->kind=$v->kind;$out->required=$this->stringList($v->required,$keep);
        $a=$this->ffi->new('thinkthen_input_property_v1['.max(1,count($v->properties)).']');$keep[]=$a;
        foreach($v->properties as $i=>$p){if(!$p instanceof Property)throw new \InvalidArgumentException('typed property required');$a[$i]->name=$this->string($p->name,$keep);$a[$i]->kind=$p->kind->value;}
        $out->properties->data=\FFI::addr($a[0]);$out->properties->len=count($v->properties);return $out;
    }
    private function author(?Author $v,array &$keep): \FFI\CData
    {
        $out=$this->ffi->new('thinkthen_question_author_v1');if($v===null)return $out;
        $out->name=$this->optionalString($v->name,$keep);
        if($v->wordingVersion!==null){
            if(!preg_match('/^(0|[1-9][0-9]*)$/D',$v->wordingVersion) || strlen($v->wordingVersion)>20 || (strlen($v->wordingVersion)===20 && strcmp($v->wordingVersion,'18446744073709551615')>0))throw new \InvalidArgumentException('unsigned wording version');
            // PHP integer is signed. Populate the exact u64 from decimal without floating point.
            $out->wording_version->present=1;
            $decimal=$v->wordingVersion;$bytes='';
            for($i=0;$i<8;++$i){$quotient='';$carry=0;foreach(str_split($decimal) as $digit){$carry=$carry*10+(int)$digit;$q=intdiv($carry,256);if($quotient!=='' || $q)$quotient.=(string)$q;$carry%=256;}$bytes.=chr($carry);$decimal=$quotient===''?'0':$quotient;}
            if(pack('S',1)!=="\x01\x00")$bytes=strrev($bytes);
            $n=$this->ffi->new('uint64_t');\FFI::memcpy(\FFI::addr($n),$bytes,8);$out->wording_version->value=$n->cdata;
        }
        $out->item_schema=$this->declaration($v->itemSchema,$keep);$out->context_schema=$this->declaration($v->contextSchema,$keep);return $out;
    }
    private function constructed(QuestionSpec $v,array &$keep): \FFI\CData
    {
        $s=$this->ffi->new('thinkthen_question_spec_v1');$s->kind=$v->kind->value;if($v->text!==null)$s->text=$this->content($v->text,$keep);
        $s->yes=$this->optionalContent($v->yes,$keep);$s->no=$this->optionalContent($v->no,$keep);$s->choices=$this->choices($v->choices,$keep);
        $s->threshold=$this->rule($v->threshold);$s->relation_threshold=$this->rule($v->relationThreshold);$s->model=$this->optionalString($v->model,$keep);$s->profile=$this->optionalString($v->profile,$keep);
        if($v->batch!==null){$s->batch->present=1;$s->batch->value=$v->batch;}$s->batch_max=(int)$v->batchMax;$s->none=(int)$v->none;$s->on=$this->stringList($v->on,$keep);
        $members=$this->ffi->new('thinkthen_member_spec_v1['.max(1,count($v->members)).']');$keep[]=$members;$children=[];
        try {
            foreach($v->members as $i=>$m){if(!$m instanceof Member)throw new \InvalidArgumentException('typed member required');$members[$i]->name=$this->string($m->name,$keep);$child=$this->question($m->question,$keep);$children[]=$child;$members[$i]->question=$child;}
            $s->members->data=\FFI::addr($members[0]);$s->members->len=count($v->members);$s->kinds=$this->choices($v->kinds,$keep);
            $rels=$this->ffi->new('thinkthen_relation_v1['.max(1,count($v->relations)).']');$keep[]=$rels;
            foreach($v->relations as $i=>$r){if(!$r instanceof Relation)throw new \InvalidArgumentException('typed relation required');foreach(['name','source','target'] as $key)$rels[$i]->$key=$this->string($r->$key,$keep);$rels[$i]->reads=$this->optionalString($r->reads,$keep);$rels[$i]->either=(int)$r->either;$rels[$i]->single=(int)$r->single;}
            $s->relations->data=\FFI::addr($rels[0]);$s->relations->len=count($v->relations);$s->name_pointer=$this->optionalString($v->namePointer,$keep);$s->kind_pointer=$this->optionalString($v->kindPointer,$keep);
            $a=$this->author($v->author,$keep);$out=$this->ffi->new('thinkthen_question *');$this->check($this->ffi->thinkthen_question_new_authored($this->engine,\FFI::addr($s),\FFI::addr($a),\FFI::addr($out)));return $out;
        } finally { foreach($children as $child)$this->ffi->thinkthen_question_free($child); }
    }
}

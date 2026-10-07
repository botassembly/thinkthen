<?php
declare(strict_types=1);
namespace ThinkThen\Native;

/** Native owns pulling, packing and scheduling. Each returned result is copied. */
final class Batch
{
    private bool $closed=false;
    public function __construct(private readonly Engine $owner,private readonly \FFI $ffi,
        private readonly \FFI\CData $pointer,private readonly string $verb) {}
    private function live(): void { if($this->closed)throw new \LogicException('batch closed'); }
    public function next(): ?CompleteResult
    {
        $this->live();$r=$this->ffi->new('thinkthen_result *');
        try { if($this->ffi->thinkthen_batch_next($this->pointer,\FFI::addr($r)))$this->owner->batchFailure();return \FFI::isNull($r)?null:$this->owner->copyResult($r,$this->verb); }
        finally { $this->ffi->thinkthen_result_free($r); }
    }
    public function facts(): CompleteResult
    {
        $this->live();$r=$this->ffi->new('thinkthen_result *');
        try { if($this->ffi->thinkthen_batch_facts($this->pointer,\FFI::addr($r)))throw new \LogicException('batch has not terminated');return $this->owner->copyResult($r,$this->verb); }
        finally { $this->ffi->thinkthen_result_free($r); }
    }
    public function close(): void { if(!$this->closed){$this->ffi->thinkthen_batch_free($this->pointer);$this->closed=true;$this->owner->batchClosed();} }
    public function __destruct() { $this->close(); }
}

<?php
declare(strict_types=1);
namespace ThinkThen\Native;

/** @template T
 * All nested values are copied from native views; no borrowed pointer escapes. */
final class CompleteResult
{
    /** @param list<T> $rows
     * @param list<ObservationView> $observations @param list<DetailsView> $details
     * @param list<QuestionAuthorView> $authors @param list<list<QuestionAuthorView>> $memberAuthors
     * @param list<list<RankView>> $rankMembers */
    public function __construct(public readonly SummaryView $summary,public readonly array $rows,
        public readonly array $observations,public readonly array $details,public readonly array $authors,
        public readonly array $memberAuthors,public readonly array $rankMembers,
        public readonly array $observationDetails,public readonly array $observationAuthors,
        public readonly array $sourceRecognitions,public readonly array $sourceRelations) {}
}
final class CompleteFailure extends \RuntimeException
{
    public function __construct(public readonly SummaryView $summary)
    {
        parent::__construct($summary->error->value->message->data,$summary->error->value->code);
    }
    public function kind(): \ThinkThen\Complete\FailureKind
    {
        return match($this->getCode()) {
            1=>\ThinkThen\Complete\FailureKind::USAGE,2=>\ThinkThen\Complete\FailureKind::BACKEND,
            3=>\ThinkThen\Complete\FailureKind::DEADLINE,4=>\ThinkThen\Complete\FailureKind::LOCAL,
            5=>\ThinkThen\Complete\FailureKind::CANCELLED,6=>\ThinkThen\Complete\FailureKind::DEFECT,
            default=>throw new \LogicException('native error kind')
        };
    }
}
final class Cancellation
{
    private bool $closed=false;
    public function __construct(private readonly \FFI $ffi,public readonly \FFI\CData $pointer) {}
    public function fire(): void { if($this->closed) throw new \LogicException('token closed'); $this->ffi->thinkthen_cancel($this->pointer); }
    public function close(): void { if(!$this->closed) { $this->ffi->thinkthen_cancel_token_free($this->pointer); $this->closed=true; } }
    public function live(): \FFI\CData { if($this->closed) throw new \LogicException('token closed'); return $this->pointer; }
    public function __destruct() { $this->close(); }
}

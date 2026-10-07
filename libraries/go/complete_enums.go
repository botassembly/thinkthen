package thinkthen

type Function string

const (
	FunctionDecide    Function = "decide"
	FunctionChoose    Function = "choose"
	FunctionTag       Function = "tag"
	FunctionScore     Function = "score"
	FunctionFilter    Function = "filter"
	FunctionRank      Function = "rank"
	FunctionFind      Function = "find"
	FunctionAnnotate  Function = "annotate"
	FunctionRecognize Function = "recognize"
	FunctionRelate    Function = "relate"
)

type ContentKind string

const (
	ContentKindText ContentKind = "text"
	ContentKindJson ContentKind = "json"
)

type RuleKind string

const (
	RuleKindDefault RuleKind = "default"
	RuleKindNull    RuleKind = "null"
	RuleKindCut     RuleKind = "cut"
	RuleKindBand    RuleKind = "band"
)

type Media string

const (
	MediaJpeg Media = "jpeg"
	MediaPng  Media = "png"
)

type SourceUnit string

const (
	SourceUnitLine      SourceUnit = "line"
	SourceUnitWindow    SourceUnit = "window"
	SourceUnitFile      SourceUnit = "file"
	SourceUnitImageFile SourceUnit = "image_file"
)

type ValueKind string

const (
	ValueKindNull     ValueKind = "null"
	ValueKindBoolean  ValueKind = "boolean"
	ValueKindAuthored ValueKind = "authored"
)

type AtomicKind string

const (
	AtomicKindYesNo  AtomicKind = "yes_no"
	AtomicKindChoice AtomicKind = "choice"
	AtomicKindTag    AtomicKind = "tag"
	AtomicKindScore  AtomicKind = "score"
	AtomicKindFind   AtomicKind = "find"
)

type MemberState string

const (
	MemberStateSuccess MemberState = "success"
	MemberStateFailure MemberState = "failure"
)

type MemberCause string

const (
	MemberCauseMissingAnswer         MemberCause = "missing_answer"
	MemberCauseWrongKind             MemberCause = "wrong_kind"
	MemberCauseMissingProbability    MemberCause = "missing_probability"
	MemberCauseInvalidProbability    MemberCause = "invalid_probability"
	MemberCauseInvalidDistribution   MemberCause = "invalid_distribution"
	MemberCauseUnexpectedProbability MemberCause = "unexpected_probability"
)

type Origin string

const (
	OriginLive   Origin = "live"
	OriginCache  Origin = "cache"
	OriginReplay Origin = "replay"
	OriginProxy  Origin = "proxy"
	OriginMemory Origin = "memory"
)

type AttemptOutcome string

const (
	AttemptOutcomeOk        AttemptOutcome = "ok"
	AttemptOutcomeStatus    AttemptOutcome = "status"
	AttemptOutcomeTransport AttemptOutcome = "transport"
)

type RelationMethod string

const (
	RelationMethodYesNo  RelationMethod = "yes_no"
	RelationMethodChoice RelationMethod = "choice"
)

type Direction string

const (
	DirectionSourceToTarget Direction = "source_to_target"
	DirectionEither         Direction = "either"
)

type Stage string

const (
	StageBoundary Stage = "boundary"
	StageKind     Stage = "kind"
	StageEdge     Stage = "edge"
	StageRelation Stage = "relation"
)

type IdentityKind string

const (
	IdentityKindObservation IdentityKind = "observation"
	IdentityKindFailure     IdentityKind = "failure"
)

type StopCause string

const (
	StopCauseUsage     StopCause = "usage"
	StopCauseLocal     StopCause = "local"
	StopCauseNoKey     StopCause = "no_key"
	StopCauseTransport StopCause = "transport"
	StopCauseStatus    StopCause = "status"
	StopCauseTooLarge  StopCause = "too_large"
	StopCauseReply     StopCause = "reply"
	StopCauseBackend   StopCause = "backend"
	StopCauseCancelled StopCause = "cancelled"
	StopCauseDefect    StopCause = "defect"
	StopCauseDeadline  StopCause = "deadline"
)

type BatchKind string

const (
	BatchKindRecords BatchKind = "records"
	BatchKindMax     BatchKind = "max"
)

type EventKind string

const (
	EventKindQuestion EventKind = "question"
	EventKindRow      EventKind = "row"
)

// QuestionRole is the explicit native saved grammar; it never guesses by shape.
type QuestionRole uint32

const (
	LoadAtomic QuestionRole = 1 + iota
	LoadSet
	LoadDynamicChoose
	LoadRecognize
	LoadRelate
	LoadRank
	LoadRankSet
	LoadFind
)
const SourceUnitJSONL SourceUnit = "jsonl"

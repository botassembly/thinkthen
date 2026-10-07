package thinkthen

type TokenUsage struct {
	InputTokens  uint64
	OutputTokens uint64
}

type QuestionSource struct {
	Origin     Origin
	AnsweredBy string
}

type ObservationIdentity struct {
	Kind          IdentityKind
	ObservationId Optional[ObservationId]
	FailureId     Optional[FailureId]
}

type ProfileWarning struct {
	TunedFor string
	Running  string
}

type BatchSetting struct {
	Kind    BatchKind
	Records uint64
}

type BatchWarning struct {
	TunedFor BatchSetting
	Running  BatchSetting
}

type Attempt struct {
	Ordinal       uint64
	RequestSha256 Digest
	WallMs        uint64
	Outcome       AttemptOutcome
	SdkRequestId  SdkRequestId
	Status        Optional[uint64]
	ServerMs      Optional[uint64]
	RequestId     Optional[string]
}

type Meta struct {
	Tool            string
	QuestionSha256  Optional[Digest]
	QuestionsSha256 Optional[Digest]
	Url             string
	Model           string
	Usage           Optional[TokenUsage]
	RequestsSent    uint64
	Cached          bool
	Requests        []Digest
	FailedQuestions uint64
	ProfileWarning  Optional[ProfileWarning]
	BatchSetting    Optional[BatchSetting]
	BatchWarning    Optional[BatchWarning]
	ContextSha256   Optional[Digest]
	Attempts        Optional[[]Attempt]
	Origin          Optional[Origin]
	QuestionSources []QuestionSource
	Observations    []ObservationIdentity
	AnsweredBy      Optional[string]
}

type CallFacts struct {
	CallId           CallId
	CacheAnswers     uint64
	EstimatedCostUsd Optional[string]
	InputTokens      Optional[uint64]
	Model            Optional[string]
	OutputTokens     Optional[uint64]
	Records          uint64
	RequestsSent     uint64
	Seconds          float64
	CommandMs        Optional[uint64]
}

type Stopped struct {
	At        Optional[uint64]
	Cause     StopCause
	Status    Optional[uint64]
	Retryable bool
}

type CompleteError struct {
	Code      uint64
	Message   string
	Retryable bool
	Stopped   Optional[Stopped]
	Facts     Optional[CallFacts]
	Attempts  Optional[[]Attempt]
}

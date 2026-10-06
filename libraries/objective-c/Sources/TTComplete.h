#ifndef TT_COMPLETE_H
#define TT_COMPLETE_H
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "TTJSON.h"
/* Host descriptors, not C ABI layouts. Every pointer borrows caller storage.
 * Keep it alive while using a descriptor. No native handles are stored here. */
typedef struct { const char *data; size_t length; } TTCText;
typedef enum { TTCFunctionDecide, TTCFunctionChoose, TTCFunctionTag, TTCFunctionScore, TTCFunctionFilter, TTCFunctionRank, TTCFunctionFind, TTCFunctionAnnotate, TTCFunctionRecognize, TTCFunctionRelate } TTCFunction;
typedef enum { TTCFailureUsage=1,TTCFailureBackend,TTCFailureDeadline,TTCFailureLocal,TTCFailureCancelled,TTCFailureDefect } TTCFailureKind;
typedef struct { bool present; uint16_t value; } TTCOptionalStatus;
typedef enum { TTCContentKindText, TTCContentKindJson } TTCContentKind;
typedef enum { TTCRuleKindDefault, TTCRuleKindNull, TTCRuleKindCut, TTCRuleKindBand } TTCRuleKind;
typedef enum { TTCMediaJpeg, TTCMediaPng } TTCMedia;
typedef enum { TTCSourceUnitLine, TTCSourceUnitWindow, TTCSourceUnitFile, TTCSourceUnitImageFile } TTCSourceUnit;
typedef enum { TTCValueKindNull, TTCValueKindBoolean, TTCValueKindAuthored } TTCValueKind;
typedef enum { TTCAtomicKindYesNo, TTCAtomicKindChoice, TTCAtomicKindTag, TTCAtomicKindScore, TTCAtomicKindFind } TTCAtomicKind;
typedef enum { TTCMemberStateSuccess, TTCMemberStateFailure } TTCMemberState;
typedef enum { TTCMemberCauseMissingAnswer, TTCMemberCauseWrongKind, TTCMemberCauseMissingProbability, TTCMemberCauseInvalidProbability, TTCMemberCauseInvalidDistribution, TTCMemberCauseUnexpectedProbability } TTCMemberCause;
typedef enum { TTCOriginLive, TTCOriginCache, TTCOriginReplay, TTCOriginProxy, TTCOriginMemory } TTCOrigin;
typedef enum { TTCAttemptOutcomeOk, TTCAttemptOutcomeStatus, TTCAttemptOutcomeTransport } TTCAttemptOutcome;
typedef enum { TTCRelationMethodYesNo, TTCRelationMethodChoice } TTCRelationMethod;
typedef enum { TTCDirectionSourceToTarget, TTCDirectionEither } TTCDirection;
typedef enum { TTCStageBoundary, TTCStageKind, TTCStageEdge, TTCStageRelation } TTCStage;
typedef enum { TTCIdentityKindObservation, TTCIdentityKindFailure } TTCIdentityKind;
typedef enum { TTCStopCauseUsage, TTCStopCauseLocal, TTCStopCauseNoKey, TTCStopCauseTransport, TTCStopCauseStatus, TTCStopCauseTooLarge, TTCStopCauseReply, TTCStopCauseBackend, TTCStopCauseCancelled, TTCStopCauseDefect, TTCStopCauseDeadline } TTCStopCause;
typedef enum { TTCBatchKindRecords, TTCBatchKindMax } TTCBatchKind;
typedef enum { TTCEventKindQuestion, TTCEventKindRow } TTCEventKind;
typedef struct { char value[64]; } TTCCallId;
typedef struct { char value[64]; } TTCSdkRequestId;
typedef struct { char value[64]; } TTCObservationId;
typedef struct { char value[64]; } TTCFailureId;
typedef struct { char value[64]; } TTCAnswerId;
typedef struct { char value[64]; } TTCDigest;
typedef struct TTCContent TTCContent;
typedef struct TTCRule TTCRule;
typedef struct TTCChoice TTCChoice;
typedef struct TTCRelation TTCRelation;
typedef struct TTCQuestionMember TTCQuestionMember;
typedef struct TTCQuestion TTCQuestion;
typedef struct TTCImageInput TTCImageInput;
typedef struct TTCImageView TTCImageView;
typedef struct TTCRecordInput TTCRecordInput;
typedef struct TTCFileSource TTCFileSource;
typedef struct TTCCallControls TTCCallControls;
typedef struct TTCProbability TTCProbability;
typedef struct TTCDecideValue TTCDecideValue;
typedef struct TTCAtomicAnswer TTCAtomicAnswer;
typedef struct TTCLocation TTCLocation;
typedef struct TTCMemberValue TTCMemberValue;
typedef struct TTCMemberFailure TTCMemberFailure;
typedef struct TTCMemberSuccess TTCMemberSuccess;
typedef struct TTCAnnotationMember TTCAnnotationMember;
typedef struct TTCEntity TTCEntity;
typedef struct TTCEntityEdge TTCEntityEdge;
typedef struct TTCPlace TTCPlace;
typedef struct TTCPiece TTCPiece;
typedef struct TTCNameSpan TTCNameSpan;
typedef struct TTCPairSpan TTCPairSpan;
typedef struct TTCRecognizeValue TTCRecognizeValue;
typedef struct TTCRecognizeAnswer TTCRecognizeAnswer;
typedef struct TTCEndpoint TTCEndpoint;
typedef struct TTCEdge TTCEdge;
typedef struct TTCRelationSuccess TTCRelationSuccess;
typedef struct TTCRelationAnswer TTCRelationAnswer;
typedef struct TTCTokenUsage TTCTokenUsage;
typedef struct TTCQuestionSource TTCQuestionSource;
typedef struct TTCObservationIdentity TTCObservationIdentity;
typedef struct TTCProfileWarning TTCProfileWarning;
typedef struct TTCBatchSetting TTCBatchSetting;
typedef struct TTCBatchWarning TTCBatchWarning;
typedef struct TTCAttempt TTCAttempt;
typedef struct TTCMeta TTCMeta;
typedef struct TTCCallFacts TTCCallFacts;
typedef struct TTCStopped TTCStopped;
typedef struct TTCCompleteError TTCCompleteError;
typedef struct TTCCommonRow TTCCommonRow;
typedef struct TTCDecideRow TTCDecideRow;
typedef struct TTCChooseRow TTCChooseRow;
typedef struct TTCTagRow TTCTagRow;
typedef struct TTCScoreRow TTCScoreRow;
typedef struct TTCFilterRow TTCFilterRow;
typedef struct TTCRankRow TTCRankRow;
typedef struct TTCFindRow TTCFindRow;
typedef struct TTCAnnotateRow TTCAnnotateRow;
typedef struct TTCRecognizeRow TTCRecognizeRow;
typedef struct TTCRelateRow TTCRelateRow;
typedef struct TTCRowValue TTCRowValue;
typedef struct TTCObservedProbabilities TTCObservedProbabilities;
typedef struct TTCObservationSuccess TTCObservationSuccess;
typedef struct TTCQuestionObservation TTCQuestionObservation;
typedef struct TTCRowObservation TTCRowObservation;
typedef struct TTCObservationEvent TTCObservationEvent;
typedef struct TTCQuestionInput TTCQuestionInput;
typedef struct TTCInputSource TTCInputSource;
typedef struct TTCCompleteRequest TTCCompleteRequest;
typedef struct TTCCompleteSummary TTCCompleteSummary;
typedef struct { const uint8_t *data; size_t count; } TTCBytes;
typedef struct { const TTCAnnotationMember *data; size_t count; } TTCListAnnotationMember;
typedef struct { const TTCAttempt *data; size_t count; } TTCListAttempt;
typedef struct { const TTCChoice *data; size_t count; } TTCListChoice;
typedef struct { const TTCDigest *data; size_t count; } TTCListDigest;
typedef struct { const TTCEdge *data; size_t count; } TTCListEdge;
typedef struct { const TTCEntity *data; size_t count; } TTCListEntity;
typedef struct { const TTCEntityEdge *data; size_t count; } TTCListEntityEdge;
typedef struct { const TTCImageInput *data; size_t count; } TTCListImageInput;
typedef struct { const TTCImageView *data; size_t count; } TTCListImageView;
typedef struct { const TTCNameSpan *data; size_t count; } TTCListNameSpan;
typedef struct { const TTCObservationIdentity *data; size_t count; } TTCListObservationIdentity;
typedef struct { const TTCPairSpan *data; size_t count; } TTCListPairSpan;
typedef struct { const TTCPiece *data; size_t count; } TTCListPiece;
typedef struct { const TTCProbability *data; size_t count; } TTCListProbability;
typedef struct { const TTCQuestionMember *data; size_t count; } TTCListQuestionMember;
typedef struct { const TTCQuestionSource *data; size_t count; } TTCListQuestionSource;
typedef struct { const TTCRecordInput *data; size_t count; } TTCListRecordInput;
typedef struct { const TTCRelation *data; size_t count; } TTCListRelation;
typedef struct { const TTCRelationAnswer *data; size_t count; } TTCListRelationAnswer;
typedef struct { const TTCText *data; size_t count; } TTCListstring;
struct TTCContent { TTCContentKind kind; TTCText data; };
struct TTCRule { TTCRuleKind kind; double low; double high; };
typedef struct { bool present; TTCContent value; } TTCOptionalContent;
typedef struct { bool present; double value; } TTCOptionaldouble;
struct TTCChoice { TTCText name; TTCOptionalContent description; TTCOptionaldouble weight; };
typedef struct { bool present; TTCText value; } TTCOptionalstring;
struct TTCRelation { TTCText name; TTCText source; TTCText target; TTCOptionalstring reads; bool either; bool single; };
struct TTCQuestionMember { TTCText name; const TTCQuestion * question; };
typedef struct { bool present; uint64_t value; } TTCOptionalulong;
struct TTCQuestion { TTCFunction kind; TTCContent text; TTCOptionalContent yes; TTCOptionalContent no; TTCListChoice choices; TTCRule threshold; TTCRule relationThreshold; TTCOptionalstring model; TTCOptionalstring profile; TTCOptionalulong batch; bool batchMax; bool none; TTCListstring on; TTCListQuestionMember members; TTCListChoice kinds; TTCListRelation relations; TTCOptionalstring namePointer; TTCOptionalstring kindPointer; };
struct TTCImageInput { TTCMedia media; TTCBytes bytes; TTCOptionalstring filename; };
struct TTCImageView { TTCMedia media; TTCBytes bytes; uint32_t width; uint32_t height; TTCOptionalstring filename; };
struct TTCRecordInput { TTCOptionalContent original; TTCOptionalContent context; TTCListChoice options; TTCListImageInput images; };
struct TTCFileSource { TTCListstring paths; TTCSourceUnit unit; uint64_t window; };
struct TTCCallControls { TTCOptionalContent context; TTCOptionalulong batch; bool batchMax; bool attempts; };
struct TTCProbability { TTCText name; double value; };
struct TTCDecideValue { TTCValueKind kind; bool boolean; TTCOptionalContent authored; };
struct TTCAtomicAnswer { TTCAtomicKind kind; TTCOptionaldouble probability; TTCOptionalstring pick; TTCOptionalstring level; TTCListProbability probabilities; TTCOptionaldouble confidence; };
struct TTCLocation { TTCOptionalstring file; TTCOptionalulong firstLine; TTCOptionalulong lastLine; };
typedef struct { bool present; TTCDecideValue value; } TTCOptionalDecideValue;
typedef struct { bool present; TTCListstring value; } TTCOptionalIReadOnlyListstring;
struct TTCMemberValue { TTCFunction function; TTCOptionalDecideValue decide; TTCOptionalstring choose; TTCOptionalIReadOnlyListstring tag; TTCOptionaldouble score; };
struct TTCMemberFailure { TTCFailureId failureId; TTCMemberCause cause; };
struct TTCMemberSuccess { TTCAnswerId answerId; TTCMemberValue value; TTCAtomicAnswer answer; TTCRule threshold; };
typedef struct { bool present; TTCMemberSuccess value; } TTCOptionalMemberSuccess;
typedef struct { bool present; TTCMemberFailure value; } TTCOptionalMemberFailure;
struct TTCAnnotationMember { TTCText name; TTCDigest request; const TTCQuestion * question; TTCMemberState state; TTCOptionalMemberSuccess success; TTCOptionalMemberFailure failure; };
struct TTCEntity { TTCText text; uint64_t start; uint64_t end; uint64_t length; TTCText kind; double strength; };
struct TTCEntityEdge { TTCText relation; TTCEntity source; TTCEntity target; double probability; bool either; };
struct TTCPlace { uint64_t start; uint64_t end; };
struct TTCPiece { uint64_t start; uint64_t end; TTCListProbability tags; };
typedef struct { bool present; TTCListProbability value; } TTCOptionalIReadOnlyListProbability;
struct TTCNameSpan { uint64_t start; uint64_t end; TTCOptionalIReadOnlyListProbability kinds; TTCOptionalIReadOnlyListProbability edges; };
struct TTCPairSpan { TTCText relation; TTCPlace source; TTCPlace target; double probability; };
typedef struct { bool present; TTCListEntityEdge value; } TTCOptionalIReadOnlyListEntityEdge;
struct TTCRecognizeValue { TTCListEntity entities; TTCOptionalIReadOnlyListEntityEdge relations; };
struct TTCRecognizeAnswer { TTCListPiece pieces; TTCListNameSpan names; TTCListPairSpan pairs; };
struct TTCEndpoint { TTCText name; TTCText kind; };
struct TTCEdge { TTCText relation; TTCEndpoint source; TTCEndpoint target; double probability; bool either; };
struct TTCRelationSuccess { TTCAnswerId answerId; double probability; bool accepted; };
typedef struct { bool present; TTCEndpoint value; } TTCOptionalEndpoint;
typedef struct { bool present; TTCRelationSuccess value; } TTCOptionalRelationSuccess;
struct TTCRelationAnswer { TTCText relation; TTCText reads; TTCRelationMethod method; TTCDirection direction; TTCEndpoint source; TTCOptionalEndpoint target; TTCDigest request; TTCMemberState state; TTCOptionalRelationSuccess success; TTCOptionalMemberFailure failure; };
struct TTCTokenUsage { uint64_t inputTokens; uint64_t outputTokens; };
struct TTCQuestionSource { TTCOrigin origin; TTCText answeredBy; };
typedef struct { bool present; TTCObservationId value; } TTCOptionalObservationId;
typedef struct { bool present; TTCFailureId value; } TTCOptionalFailureId;
struct TTCObservationIdentity { TTCIdentityKind kind; TTCOptionalObservationId observationId; TTCOptionalFailureId failureId; };
struct TTCProfileWarning { TTCText tunedFor; TTCText running; };
struct TTCBatchSetting { TTCBatchKind kind; uint64_t records; };
struct TTCBatchWarning { TTCBatchSetting tunedFor; TTCBatchSetting running; };
struct TTCAttempt { uint64_t ordinal; TTCDigest requestSha256; uint64_t wallMs; TTCAttemptOutcome outcome; TTCSdkRequestId sdkRequestId; TTCOptionalStatus status; TTCOptionalulong serverMs; TTCOptionalstring requestId; };
typedef struct { bool present; TTCDigest value; } TTCOptionalDigest;
typedef struct { bool present; TTCTokenUsage value; } TTCOptionalTokenUsage;
typedef struct { bool present; TTCProfileWarning value; } TTCOptionalProfileWarning;
typedef struct { bool present; TTCBatchSetting value; } TTCOptionalBatchSetting;
typedef struct { bool present; TTCBatchWarning value; } TTCOptionalBatchWarning;
typedef struct { bool present; TTCListAttempt value; } TTCOptionalIReadOnlyListAttempt;
typedef struct { bool present; TTCOrigin value; } TTCOptionalOrigin;
struct TTCMeta { TTCText tool; TTCOptionalDigest questionSha256; TTCOptionalDigest questionsSha256; TTCText url; TTCText model; TTCOptionalTokenUsage usage; uint64_t requestsSent; bool cached; TTCListDigest requests; uint64_t failedQuestions; TTCOptionalProfileWarning profileWarning; TTCOptionalBatchSetting batchSetting; TTCOptionalBatchWarning batchWarning; TTCOptionalDigest contextSha256; TTCOptionalIReadOnlyListAttempt attempts; TTCOptionalOrigin origin; TTCListQuestionSource questionSources; TTCListObservationIdentity observations; TTCOptionalstring answeredBy; };
struct TTCCallFacts { TTCCallId callId; uint64_t cacheAnswers; TTCOptionalstring estimatedCostUsd; TTCOptionalulong inputTokens; TTCOptionalstring model; TTCOptionalulong outputTokens; uint64_t records; uint64_t requestsSent; double seconds; TTCOptionalulong commandMs; };
struct TTCStopped { TTCOptionalulong at; TTCStopCause cause; TTCOptionalStatus status; bool retryable; };
typedef struct { bool present; TTCStopped value; } TTCOptionalStopped;
typedef struct { bool present; TTCCallFacts value; } TTCOptionalCallFacts;
struct TTCCompleteError { TTCFailureKind code; TTCText message; bool retryable; TTCOptionalStopped stopped; TTCOptionalCallFacts facts; TTCOptionalIReadOnlyListAttempt attempts; };
typedef struct { bool present; const TTCQuestion * value; } TTCOptionalQuestion;
typedef struct { bool present; TTCAtomicAnswer value; } TTCOptionalAtomicAnswer;
typedef struct { bool present; TTCRule value; } TTCOptionalRule;
typedef struct { bool present; TTCLocation value; } TTCOptionalLocation;
typedef struct { bool present; TTCListImageView value; } TTCOptionalIReadOnlyListImageView;
struct TTCCommonRow { TTCAnswerId answerId; TTCOptionalContent input; TTCOptionalQuestion question; TTCOptionalAtomicAnswer answer; TTCOptionalRule threshold; TTCOptionalLocation position; TTCOptionalstring inputFile; TTCMeta meta; TTCOptionalIReadOnlyListImageView images; };
struct TTCDecideRow { TTCCommonRow common; TTCDecideValue value; };
struct TTCChooseRow { TTCCommonRow common; TTCOptionalstring value; };
struct TTCTagRow { TTCCommonRow common; TTCListstring value; };
struct TTCScoreRow { TTCCommonRow common; double value; };
struct TTCFilterRow { TTCCommonRow common; bool value; };
struct TTCRankRow { TTCCommonRow common; TTCOptionalulong value; TTCOptionalstring questionName; };
struct TTCFindRow { TTCCommonRow common; TTCOptionalContent value; TTCOptionalulong index; };
struct TTCAnnotateRow { TTCCommonRow common; TTCListAnnotationMember answers; };
struct TTCRecognizeRow { TTCCommonRow common; TTCRecognizeValue value; TTCRecognizeAnswer answer; };
struct TTCRelateRow { TTCCommonRow common; TTCListEdge value; TTCListRelationAnswer questions; };
typedef struct { bool present; TTCDecideRow value; } TTCOptionalDecideRow;
typedef struct { bool present; TTCChooseRow value; } TTCOptionalChooseRow;
typedef struct { bool present; TTCTagRow value; } TTCOptionalTagRow;
typedef struct { bool present; TTCScoreRow value; } TTCOptionalScoreRow;
typedef struct { bool present; TTCFilterRow value; } TTCOptionalFilterRow;
typedef struct { bool present; TTCRankRow value; } TTCOptionalRankRow;
typedef struct { bool present; TTCFindRow value; } TTCOptionalFindRow;
typedef struct { bool present; TTCAnnotateRow value; } TTCOptionalAnnotateRow;
typedef struct { bool present; TTCRecognizeRow value; } TTCOptionalRecognizeRow;
typedef struct { bool present; TTCRelateRow value; } TTCOptionalRelateRow;
struct TTCRowValue { TTCFunction function; TTCOptionalDecideRow decide; TTCOptionalChooseRow choose; TTCOptionalTagRow tag; TTCOptionalScoreRow score; TTCOptionalFilterRow filter; TTCOptionalRankRow rank; TTCOptionalFindRow find; TTCOptionalAnnotateRow annotate; TTCOptionalRecognizeRow recognize; TTCOptionalRelateRow relate; };
struct TTCObservedProbabilities { TTCOptionaldouble yes; TTCOptionalIReadOnlyListProbability named; };
struct TTCObservationSuccess { TTCAnswerId answerId; TTCObservationId observationId; TTCMemberValue value; TTCObservedProbabilities probabilities; TTCOptionaldouble confidence; };
typedef struct { bool present; TTCStage value; } TTCOptionalStage;
typedef struct { bool present; TTCObservationSuccess value; } TTCOptionalObservationSuccess;
struct TTCQuestionObservation { uint64_t index; TTCOptionalstring member; TTCOptionalStage stage; uint64_t position; TTCDigest questionSha256; TTCText model; TTCText url; TTCListDigest requests; uint64_t requestsSent; bool cached; uint64_t failedQuestions; TTCOptionalTokenUsage usage; TTCListQuestionSource questionSources; TTCMemberState state; TTCOptionalObservationSuccess success; TTCOptionalMemberFailure failure; };
struct TTCRowObservation { uint64_t index; TTCRowValue value; };
typedef struct { bool present; TTCQuestionObservation value; } TTCOptionalQuestionObservation;
typedef struct { bool present; TTCRowObservation value; } TTCOptionalRowObservation;
struct TTCObservationEvent { TTCEventKind kind; TTCOptionalQuestionObservation question; TTCOptionalRowObservation row; };
struct TTCQuestionInput { TTCOptionalQuestion question; TTCOptionalstring file; };
typedef struct { bool present; TTCListRecordInput value; } TTCOptionalIReadOnlyListRecordInput;
typedef struct { bool present; TTCFileSource value; } TTCOptionalFileSource;
struct TTCInputSource { TTCOptionalIReadOnlyListRecordInput records; TTCOptionalFileSource files; };
struct TTCCompleteRequest { TTCFunction function; TTCQuestionInput question; TTCInputSource source; TTCCallControls controls; };
struct TTCCompleteSummary { uint64_t count; uint64_t observationCount; TTCText schema; TTCAnswerId answerId; TTCFunction function; TTCMeta meta; TTCCallFacts facts; TTCOptionalIReadOnlyListAttempt attempts; };
typedef struct { TTCCompleteSummary summary; const TTCRowValue *rows; size_t rowCount; const TTCObservationEvent *observations; size_t observationCount; } TTCCompleteResult;
/* Validate/copy opaque IDs without truncation; failure leaves output untouched. */
static inline bool ttc_identity(const char *data, size_t length, char out[64]) {
    if (!data || !out || length != 64) return false;
    for (size_t i=0; i<64; ++i) if (!((data[i]>='0' && data[i]<='9') || (data[i]>='a' && data[i]<='f'))) return false;
    for (size_t i=0; i<64; ++i) out[i]=data[i];
    return true;
}
static inline TTCCompleteRequest ttc_decide(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionDecide, question, source, controls}; }
static inline TTCCompleteRequest ttc_choose(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionChoose, question, source, controls}; }
static inline TTCCompleteRequest ttc_tag(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionTag, question, source, controls}; }
static inline TTCCompleteRequest ttc_score(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionScore, question, source, controls}; }
static inline TTCCompleteRequest ttc_filter(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionFilter, question, source, controls}; }
static inline TTCCompleteRequest ttc_rank(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionRank, question, source, controls}; }
static inline TTCCompleteRequest ttc_find(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionFind, question, source, controls}; }
static inline TTCCompleteRequest ttc_annotate(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionAnnotate, question, source, controls}; }
static inline TTCCompleteRequest ttc_recognize(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionRecognize, question, source, controls}; }
static inline TTCCompleteRequest ttc_relate(TTCQuestionInput question, TTCInputSource source, TTCCallControls controls) { return (TTCCompleteRequest){TTCFunctionRelate, question, source, controls}; }
static inline TTCQuestion ttc_question(TTCFunction kind, TTCContent text) {
    TTCQuestion result = {0}; result.kind = kind; result.text = text; return result;
}
static inline TTCQuestionInput ttc_asked(const TTCQuestion *value) { return (TTCQuestionInput){{true, value}, {0}}; }
static inline TTCQuestionInput ttc_question_file(TTCText path) { return (TTCQuestionInput){{0}, {true, path}}; }
static inline TTCInputSource ttc_records(TTCListRecordInput values) { return (TTCInputSource){{true, values}, {0}}; }
static inline TTCInputSource ttc_files(TTCFileSource value) { return (TTCInputSource){{0}, {true, value}}; }
/* Readers accept existing host JSON trees; borrowed strings live until tt_json_free.
 * Probability storage is caller-owned; capacity counts entries, not bytes.
 * Failure leaves output and probability storage untouched. Facts require real call_id. */
bool ttc_atomic_read(const TTJSON *object, TTCAtomicAnswer *out, TTCProbability *storage, size_t capacity);
bool ttc_facts_read(const TTJSON *object, TTCCallFacts *out);
#endif

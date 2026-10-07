part of 'abi.dart';

final class CPairView extends Struct {
  external CStringView relation;
  external CPlaceView source;
  external CPlaceView target;
  @Double()
  external double probability;
}

final class CPairsView extends Struct {
  external Pointer<CPairView> data;
  @Size()
  external int len;
}

final class CRecognizeValueView extends Struct {
  external CEntitiesView entities;
  external COptionalEntityEdgesView relations;
}

final class CRecognizeAnswerView extends Struct {
  external CPiecesView pieces;
  external CNamesView names;
  external CPairsView pairs;
}

final class CEndpointView extends Struct {
  external CStringView name;
  external CStringView kind;
}

final class COptionalEndpointView extends Struct {
  @Int32()
  external int present;
  external CEndpointView value;
}

final class CEdgeView extends Struct {
  external CStringView relation;
  external CEndpointView source;
  external CEndpointView target;
  @Double()
  external double probability;
  @Int32()
  external int either;
}

final class CEdgesView extends Struct {
  external Pointer<CEdgeView> data;
  @Size()
  external int len;
}

final class CRelationSuccessView extends Struct {
  external CStringView answer_id;
  @Double()
  external double probability;
  @Int32()
  external int accepted;
}

final class CRelationAnswerV1DataView extends Union {
  external CRelationSuccessView success;
  external CMemberFailureView failure;
}

final class CRelationAnswerView extends Struct {
  external CStringView relation;
  external CStringView reads;
  @Uint32()
  external int method;
  @Uint32()
  external int direction;
  external CEndpointView source;
  external COptionalEndpointView target;
  external CStringView request;
  @Uint32()
  external int state;
  external CRelationAnswerV1DataView data;
}

final class CRelationAnswersView extends Struct {
  external Pointer<CRelationAnswerView> data;
  @Size()
  external int len;
}

final class CUsageView extends Struct {
  @Uint64()
  external int input_tokens;
  @Uint64()
  external int output_tokens;
}

final class COptionalUsageView extends Struct {
  @Int32()
  external int present;
  external CUsageView value;
}

final class CQuestionSourceView extends Struct {
  @Uint32()
  external int origin;
  external CStringView answered_by;
}

final class CQuestionSourcesView extends Struct {
  external Pointer<CQuestionSourceView> data;
  @Size()
  external int len;
}

final class CObservationIdentityV1DataView extends Union {
  external CStringView observation_id;
  external CStringView failure_id;
}

final class CObservationIdentityView extends Struct {
  @Uint32()
  external int kind;
  external CObservationIdentityV1DataView data;
}

final class CObservationIdentitiesView extends Struct {
  external Pointer<CObservationIdentityView> data;
  @Size()
  external int len;
}

final class CProfileWarningView extends Struct {
  external CStringView tuned_for;
  external CStringView running;
}

final class COptionalProfileWarningView extends Struct {
  @Int32()
  external int present;
  external CProfileWarningView value;
}

final class CBatchView extends Struct {
  @Uint32()
  external int kind;
  @Size()
  external int records;
}

final class COptionalBatchView extends Struct {
  @Int32()
  external int present;
  external CBatchView value;
}

final class CBatchWarningView extends Struct {
  external CBatchView tuned_for;
  external CBatchView running;
}

final class COptionalBatchWarningView extends Struct {
  @Int32()
  external int present;
  external CBatchWarningView value;
}

final class CAttemptView extends Struct {
  @Uint64()
  external int ordinal;
  external CStringView request_sha256;
  @Uint64()
  external int wall_ms;
  @Uint32()
  external int outcome;
  external CStringView sdk_request_id;
  external COptionalU16View status;
  external COptionalU64View server_ms;
  external COptionalStringView request_id;
}

final class CAttemptsView extends Struct {
  external Pointer<CAttemptView> data;
  @Size()
  external int len;
}

final class COptionalAttemptsView extends Struct {
  @Int32()
  external int present;
  external CAttemptsView value;
}

final class CMetaView extends Struct {
  external CStringView tool;
  external COptionalStringView question_sha256;
  external COptionalStringView questions_sha256;
  external CStringView url;
  external CStringView model;
  external COptionalUsageView usage;
  @Uint64()
  external int requests_sent;
  @Int32()
  external int cached;
  external CStringsView requests;
  @Size()
  external int failed_questions;
  external COptionalProfileWarningView profile_warning;
  external COptionalBatchView batch_setting;
  external COptionalBatchWarningView batch_warning;
  external COptionalStringView context_sha256;
  external COptionalAttemptsView attempts;
  external COptionalDiscriminatorView origin;
  external CQuestionSourcesView question_sources;
  external CObservationIdentitiesView observations;
  external COptionalStringView answered_by;
}

final class COptionalMetaView extends Struct {
  @Int32()
  external int present;
  external CMetaView value;
}

final class CFactsView extends Struct {
  external CStringView call_id;
  @Uint64()
  external int cache_answers;
  external COptionalStringView estimated_cost_usd;
  external COptionalU64View input_tokens;
  external COptionalStringView model;
  external COptionalU64View output_tokens;
  @Uint64()
  external int records;
  @Uint64()
  external int requests_sent;
  @Double()
  external double seconds;
  external COptionalU64View command_ms;
}

final class COptionalFactsView extends Struct {
  @Int32()
  external int present;
  external CFactsView value;
}

final class CStoppedView extends Struct {
  external COptionalSizeView at;
  @Uint32()
  external int cause;
  external COptionalU16View status;
  @Int32()
  external int retryable;
}

final class COptionalStoppedView extends Struct {
  @Int32()
  external int present;
  external CStoppedView value;
}

final class CErrorView extends Struct {
  @Int32()
  external int code;
  external CStringView message;
  @Int32()
  external int retryable;
  external COptionalStoppedView stopped;
}

final class COptionalErrorView extends Struct {
  @Int32()
  external int present;
  external CErrorView value;
}

final class CRowView extends Struct {
  external CStringView answer_id;
  external COptionalContentView input;
  external COptionalQuestionView question;
  external COptionalAnswerView answer;
  external COptionalRuleView threshold;
  external COptionalLocationView position;
  external COptionalStringView input_file;
  external CMetaView meta;
  external COptionalImageViewsView images;
}

final class CDecideView extends Struct {
  external CRowView common;
  external CDecideValueView value;
}

final class CChooseView extends Struct {
  external CRowView common;
  external COptionalStringView value;
}

final class CTagView extends Struct {
  external CRowView common;
  external CStringsView value;
}

final class CScoreView extends Struct {
  external CRowView common;
  @Double()
  external double value;
}

final class CFilterView extends Struct {
  external CRowView common;
  @Int32()
  external int value;
}

final class CRankView extends Struct {
  external CRowView common;
  external COptionalSizeView value;
  external COptionalStringView question_name;
}

final class CFindView extends Struct {
  external CRowView common;
  external COptionalContentView value;
  external COptionalSizeView index;
}

final class CAnnotateView extends Struct {
  external CRowView common;
  external CMembersView answers;
}

final class CRecognizeView extends Struct {
  external CRowView common;
  external CRecognizeValueView value;
  external CRecognizeAnswerView answer;
}

final class CRelateView extends Struct {
  external CRowView common;
  external CEdgesView value;
  external CRelationAnswersView questions;
}

final class CObservedProbabilitiesV1DataView extends Union {
  @Double()
  external double yes;
  external CProbabilitiesView named;
}

final class CObservedProbabilitiesView extends Struct {
  @Uint32()
  external int kind;
  external CObservedProbabilitiesV1DataView data;
}

final class CObservationSuccessView extends Struct {
  external CStringView answer_id;
  external CStringView observation_id;
  external CMemberValueView value;
  external CObservedProbabilitiesView probabilities;
  external COptionalDoubleView confidence;
}

final class CQuestionObservationV1DataView extends Union {
  external CObservationSuccessView success;
  external CMemberFailureView failure;
}

final class CQuestionObservationView extends Struct {
  @Size()
  external int index;
  external COptionalStringView member;
  external COptionalDiscriminatorView stage;
  @Size()
  external int position;
  external CStringView question_sha256;
  external CStringView model;
  external CStringView url;
  external CStringsView requests;
  @Uint64()
  external int requests_sent;
  @Int32()
  external int cached;
  @Size()
  external int failed_questions;
  external COptionalUsageView usage;
  external CQuestionSourcesView question_sources;
  @Uint32()
  external int state;
  external CQuestionObservationV1DataView data;
}

final class CRowObservationV1DataView extends Union {
  external CDecideView decide;
  external CChooseView choose;
  external CTagView tag;
  external CScoreView score;
  external CFilterView filter;
  external CRankView rank;
  external CFindView find;
  external CAnnotateView annotate;
  external CRecognizeView recognize;
  external CRelateView relate;
}

final class CRowObservationView extends Struct {
  @Size()
  external int index;
  @Uint32()
  external int function;
  external CRowObservationV1DataView data;
}

final class CObservationV1DataView extends Union {
  external CQuestionObservationView question;
  external CRowObservationView row;
}

final class CObservationView extends Struct {
  @Uint32()
  external int kind;
  external CObservationV1DataView data;
}

final class CSummaryView extends Struct {
  @Uint32()
  external int state;
  external CStringView schema;
  external COptionalStringView answer_id;
  external COptionalDiscriminatorView function;
  @Size()
  external int count;
  @Size()
  external int observation_count;
  external COptionalMetaView meta;
  external COptionalFactsView facts;
  external COptionalAttemptsView attempts;
  external COptionalErrorView error;
}

final class CReportedUsageView extends Struct {
  @Int32()
  external int present;
  external COptionalU64View input_tokens;
  external COptionalU64View output_tokens;
}

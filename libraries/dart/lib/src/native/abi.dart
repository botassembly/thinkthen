import 'dart:ffi';

final class CLegacyAnswerView extends Struct {
  @Int32()
  external int outcome;
  @Double()
  external double probability;
}

final class CStringView extends Struct {
  external Pointer<Uint8> data;
  @Size()
  external int len;
}

final class CStringsView extends Struct {
  external Pointer<CStringView> data;
  @Size()
  external int len;
}

final class COptionalStringView extends Struct {
  @Int32()
  external int present;
  external CStringView value;
}

final class COptionalSizeView extends Struct {
  @Int32()
  external int present;
  @Size()
  external int value;
}

final class COptionalU64View extends Struct {
  @Int32()
  external int present;
  @Uint64()
  external int value;
}

final class COptionalU16View extends Struct {
  @Int32()
  external int present;
  @Uint16()
  external int value;
}

final class COptionalDoubleView extends Struct {
  @Int32()
  external int present;
  @Double()
  external double value;
}

final class COptionalDiscriminatorView extends Struct {
  @Int32()
  external int present;
  @Uint32()
  external int value;
}

final class CContentView extends Struct {
  @Uint32()
  external int kind;
  external CStringView data;
}

final class COptionalContentView extends Struct {
  @Int32()
  external int present;
  external CContentView value;
}

final class CRuleView extends Struct {
  @Uint32()
  external int kind;
  @Double()
  external double low;
  @Double()
  external double high;
}

final class COptionalRuleView extends Struct {
  @Int32()
  external int present;
  external CRuleView value;
}

final class CChoiceView extends Struct {
  external CStringView name;
  external COptionalContentView description;
  external COptionalDoubleView weight;
}

final class CChoicesView extends Struct {
  external Pointer<CChoiceView> data;
  @Size()
  external int len;
}

final class CRelationView extends Struct {
  external CStringView name;
  external CStringView source;
  external CStringView target;
  external COptionalStringView reads;
  @Int32()
  external int either;
  @Int32()
  external int single;
}

final class CRelationsView extends Struct {
  external Pointer<CRelationView> data;
  @Size()
  external int len;
}

final class CMemberSpecView extends Struct {
  external CStringView name;
  external Pointer<Void> question;
}

final class CMemberSpecsView extends Struct {
  external Pointer<CMemberSpecView> data;
  @Size()
  external int len;
}

final class CQuestionSpecView extends Struct {
  @Uint32()
  external int kind;
  external CContentView text;
  external COptionalContentView yes;
  external COptionalContentView no;
  external CChoicesView choices;
  external CRuleView threshold;
  external CRuleView relation_threshold;
  external COptionalStringView model;
  external COptionalStringView profile;
  external COptionalSizeView batch;
  @Int32()
  external int batch_max;
  @Int32()
  external int none;
  external CStringsView on;
  external CMemberSpecsView members;
  external CChoicesView kinds;
  external CRelationsView relations;
  external COptionalStringView name_pointer;
  external COptionalStringView kind_pointer;
}

final class CQuestionMemberView extends Struct {
  external CStringView name;
  external Pointer<CQuestionView> question;
}

final class CQuestionMembersView extends Struct {
  external Pointer<CQuestionMemberView> data;
  @Size()
  external int len;
}

final class CQuestionView extends Struct {
  @Uint32()
  external int kind;
  external CContentView text;
  external COptionalContentView yes;
  external COptionalContentView no;
  external CChoicesView choices;
  external CRuleView threshold;
  external CRuleView relation_threshold;
  external COptionalStringView model;
  external COptionalStringView profile;
  external COptionalSizeView batch;
  @Int32()
  external int batch_max;
  @Int32()
  external int none;
  external CStringsView on;
  external CQuestionMembersView members;
  external CChoicesView kinds;
  external CRelationsView relations;
  external COptionalStringView name_pointer;
  external COptionalStringView kind_pointer;
}

final class COptionalQuestionView extends Struct {
  @Int32()
  external int present;
  external CQuestionView value;
}

final class CImagesView extends Struct {
  external Pointer<Pointer<Void>> data;
  @Size()
  external int len;
}

final class CImageView extends Struct {
  @Uint32()
  external int media;
  external Pointer<Uint8> bytes;
  @Size()
  external int bytes_len;
  @Uint32()
  external int width;
  @Uint32()
  external int height;
  external COptionalStringView filename;
}

final class CImageViewsView extends Struct {
  external Pointer<CImageView> data;
  @Size()
  external int len;
}

final class COptionalImageViewsView extends Struct {
  @Int32()
  external int present;
  external CImageViewsView value;
}

final class CRecordView extends Struct {
  external COptionalContentView original;
  external COptionalContentView context;
  external CChoicesView options;
  external CImagesView images;
}

final class CSourceSpecView extends Struct {
  external CStringsView paths;
  @Uint32()
  external int unit;
  @Size()
  external int window;
}

final class CControlsView extends Struct {
  @Int64()
  external int deadline_ms;
  external Pointer<Void> cancel;
  external COptionalContentView context;
  external COptionalSizeView batch;
  @Int32()
  external int batch_max;
  @Int32()
  external int attempts;
  external CStringView surface;
}

final class CDecideValueV1DataView extends Union {
  @Int32()
  external int boolean;
  external CContentView authored;
}

final class CDecideValueView extends Struct {
  @Uint32()
  external int kind;
  external CDecideValueV1DataView data;
}

final class CProbabilityView extends Struct {
  external CStringView name;
  @Double()
  external double probability;
}

final class CProbabilitiesView extends Struct {
  external Pointer<CProbabilityView> data;
  @Size()
  external int len;
}

final class COptionalProbabilitiesView extends Struct {
  @Int32()
  external int present;
  external CProbabilitiesView value;
}

final class CNamedAnswerView extends Struct {
  external CStringView pick;
  external CProbabilitiesView probabilities;
  external COptionalDoubleView confidence;
}

final class CScoreAnswerView extends Struct {
  external CStringView level;
  external CProbabilitiesView probabilities;
  external COptionalDoubleView confidence;
}

final class CAnswerV1DataView extends Union {
  @Double()
  external double probability;
  external CNamedAnswerView choice;
  external CProbabilitiesView tag;
  external CScoreAnswerView score;
  external CNamedAnswerView find;
}

final class CAnswerView extends Struct {
  @Uint32()
  external int kind;
  external CAnswerV1DataView data;
}

final class COptionalAnswerView extends Struct {
  @Int32()
  external int present;
  external CAnswerView value;
}

final class CLocationView extends Struct {
  external COptionalStringView file;
  external COptionalSizeView first_line;
  external COptionalSizeView last_line;
}

final class COptionalLocationView extends Struct {
  @Int32()
  external int present;
  external CLocationView value;
}

final class CMemberValueV1DataView extends Union {
  external CDecideValueView decide;
  external COptionalStringView choose;
  external CStringsView tag;
  @Double()
  external double score;
}

final class CMemberValueView extends Struct {
  @Uint32()
  external int kind;
  external CMemberValueV1DataView data;
}

final class CMemberFailureView extends Struct {
  external CStringView failure_id;
  @Uint32()
  external int cause;
}

final class CMemberSuccessView extends Struct {
  external CStringView answer_id;
  external CMemberValueView value;
  external CAnswerView answer;
  external CRuleView threshold;
}

final class CMemberV1DataView extends Union {
  external CMemberSuccessView success;
  external CMemberFailureView failure;
}

final class CMemberView extends Struct {
  external CStringView name;
  external CStringView request;
  external CQuestionView question;
  @Uint32()
  external int state;
  external CMemberV1DataView data;
}

final class CMembersView extends Struct {
  external Pointer<CMemberView> data;
  @Size()
  external int len;
}

final class CEntityView extends Struct {
  external CStringView text;
  @Size()
  external int start;
  @Size()
  external int end;
  @Size()
  external int length;
  external CStringView kind;
  @Double()
  external double strength;
}

final class CEntitiesView extends Struct {
  external Pointer<CEntityView> data;
  @Size()
  external int len;
}

final class CEntityEdgeView extends Struct {
  external CStringView relation;
  external CEntityView source;
  external CEntityView target;
  @Double()
  external double probability;
  @Int32()
  external int either;
}

final class CEntityEdgesView extends Struct {
  external Pointer<CEntityEdgeView> data;
  @Size()
  external int len;
}

final class COptionalEntityEdgesView extends Struct {
  @Int32()
  external int present;
  external CEntityEdgesView value;
}

final class CPlaceView extends Struct {
  @Size()
  external int start;
  @Size()
  external int end;
}

final class CPieceView extends Struct {
  @Size()
  external int start;
  @Size()
  external int end;
  external CProbabilitiesView tags;
}

final class CPiecesView extends Struct {
  external Pointer<CPieceView> data;
  @Size()
  external int len;
}

final class CNameView extends Struct {
  @Size()
  external int start;
  @Size()
  external int end;
  external COptionalProbabilitiesView kinds;
  external COptionalProbabilitiesView edges;
}

final class CNamesView extends Struct {
  external Pointer<CNameView> data;
  @Size()
  external int len;
}

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

final class CSourceDetailView extends Struct {
  @Uint32()
  external int origin;
  external CStringView answered_by;
  external COptionalSizeView batch_size;
}

final class CSourceDetailsView extends Struct {
  external Pointer<CSourceDetailView> data;
  @Size()
  external int len;
}

final class CInputView extends Struct {
  external COptionalContentView original;
  external COptionalLocationView position;
  external COptionalImageViewsView images;
}

final class CInputViewsView extends Struct {
  external Pointer<CInputView> data;
  @Size()
  external int len;
}

final class CDetailsView extends Struct {
  external COptionalQuestionView question;
  external COptionalRuleView threshold;
  external COptionalStringView raw_pick;
  external CReportedUsageView usage;
  external CSourceDetailsView question_sources;
  external CObservationIdentitiesView observations;
  external CInputViewsView inputs;
}

final class CSourceEntityView extends Struct {
  external CEntityView entity;
  external COptionalLocationView position;
}

final class CSourceEntitiesView extends Struct {
  external Pointer<CSourceEntityView> data;
  @Size()
  external int len;
}

final class CSourceEntityEdgeView extends Struct {
  external CStringView relation;
  external CSourceEntityView source;
  external CSourceEntityView target;
  @Double()
  external double probability;
  @Int32()
  external int either;
}

final class CSourceEntityEdgesView extends Struct {
  external Pointer<CSourceEntityEdgeView> data;
  @Size()
  external int len;
}

final class COptionalSourceEntityEdgesView extends Struct {
  @Int32()
  external int present;
  external CSourceEntityEdgesView value;
}

final class CSourceRecognitionView extends Struct {
  @Int32()
  external int present;
  external CSourceEntitiesView entities;
  external COptionalSourceEntityEdgesView relations;
}

final class CSourceEndpointView extends Struct {
  @Size()
  external int ordinal;
  external CEndpointView endpoint;
  external CContentView record;
  external COptionalLocationView position;
}

final class CSourceEdgeView extends Struct {
  external CStringView relation;
  external CSourceEndpointView source;
  external CSourceEndpointView target;
  @Double()
  external double probability;
  @Int32()
  external int either;
}

final class CSourceEdgesView extends Struct {
  external Pointer<CSourceEdgeView> data;
  @Size()
  external int len;
}

final class CSourceRelationsView extends Struct {
  @Int32()
  external int present;
  external CSourceEdgesView edges;
}

final class CInputPropertyView extends Struct {
  external CStringView name;
  @Uint32()
  external int kind;
}

final class CInputPropertiesView extends Struct {
  external Pointer<CInputPropertyView> data;
  @Size()
  external int len;
}

final class CInputDeclarationView extends Struct {
  @Uint32()
  external int kind;
  external CInputPropertiesView properties;
  external CStringsView required;
}

final class CQuestionAuthorView extends Struct {
  external COptionalStringView name;
  external COptionalU64View wording_version;
  external CInputDeclarationView item_schema;
  external CInputDeclarationView context_schema;
}

final class CRecognitionTaskView extends Struct {
  external COptionalStringView instructions;
  external COptionalStringView entity_definition;
}

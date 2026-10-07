part of 'abi.dart';

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
  external Pointer<CQuestionViewView> question;
}

final class CQuestionMembersView extends Struct {
  external Pointer<CQuestionMemberView> data;
  @Size()
  external int len;
}

final class CQuestionViewView extends Struct {
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
  external CQuestionViewView value;
}

final class CImagesView extends Struct {
  external Pointer<Pointer<Void>> data;
  @Size()
  external int len;
}

final class CImageViewView extends Struct {
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
  external Pointer<CImageViewView> data;
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
  external CQuestionViewView question;
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

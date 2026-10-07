part of 'abi.dart';

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

final class CInputViewView extends Struct {
  external COptionalContentView original;
  external COptionalLocationView position;
  external COptionalImageViewsView images;
}

final class CInputViewsView extends Struct {
  external Pointer<CInputViewView> data;
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

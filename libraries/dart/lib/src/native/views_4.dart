part of 'views.dart';

final class InputDeclarationView {
  final int kind;
  final InputPropertiesView properties;
  final StringsView required;
  const InputDeclarationView(this.kind, this.properties, this.required);
  factory InputDeclarationView.copy(CInputDeclarationView v) =>
      InputDeclarationView(v.kind, InputPropertiesView.copy(v.properties),
          StringsView.copy(v.required));
}

final class QuestionAuthorView {
  final OptionalStringView name;
  final OptionalU64View wording_version;
  final InputDeclarationView item_schema;
  final InputDeclarationView context_schema;
  const QuestionAuthorView(
      this.name, this.wording_version, this.item_schema, this.context_schema);
  factory QuestionAuthorView.copy(CQuestionAuthorView v) => QuestionAuthorView(
      OptionalStringView.copy(v.name),
      OptionalU64View.copy(v.wording_version),
      InputDeclarationView.copy(v.item_schema),
      InputDeclarationView.copy(v.context_schema));
}

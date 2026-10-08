import 'input.dart';

enum FunctionKind {
  decide,
  choose,
  tag,
  score,
  filter,
  rank,
  find,
  annotate,
  recognize,
  relate
}

final class Rule {
  final int kind;
  final double low, high;
  const Rule.missing()
      : kind = 0,
        low = 0,
        high = 0;
  const Rule.none()
      : kind = 1,
        low = 0,
        high = 0;
  const Rule.cut(this.low)
      : kind = 2,
        high = 0;
  const Rule.band(this.low, this.high) : kind = 3;
}

final class Member {
  final String name;
  final Question question;
  const Member(this.name, this.question);
}

final class Relation {
  final String name, source, target;
  final String? reads;
  final bool either, single;
  const Relation(this.name, this.source, this.target,
      {this.reads, this.either = false, this.single = false});
}

enum PropertyKind { string, number, boolean, stringList }

final class Property {
  final String name;
  final PropertyKind kind;
  const Property(this.name, this.kind);
}

final class Declaration {
  final int kind;
  final List<Property> properties;
  final List<String> required;
  const Declaration.absent()
      : kind = 0,
        properties = const [],
        required = const [];
  const Declaration.string()
      : kind = 1,
        properties = const [],
        required = const [];
  Declaration.object(List<Property> properties,
      {List<String> required = const []})
      : kind = 2,
        properties = List.unmodifiable(properties),
        required = List.unmodifiable(required);
}

final class Author {
  final String? name;
  final BigInt? wordingVersion;
  final Declaration? itemSchema, contextSchema;
  const Author(
      {this.name, this.wordingVersion, this.itemSchema, this.contextSchema});
}

/// Counted typed question descriptor; native owns semantic admission.
final class QuestionSpec {
  final FunctionKind kind;
  final Content? text, yes, no;
  final List<Choice> choices, kinds;
  final Rule? threshold, relationThreshold;
  final String? model,
      profile,
      namePointer,
      kindPointer,
      instructions,
      entityDefinition;
  final int? batch;
  final bool batchMax, none;
  final List<String> on;
  final List<Member> members;
  final List<Relation> relations;
  final Author? author;
  QuestionSpec(this.kind,
      {this.instructions,
      this.entityDefinition,
      this.text,
      this.yes,
      this.no,
      List<Choice> choices = const [],
      this.threshold,
      this.relationThreshold,
      this.model,
      this.profile,
      this.batch,
      this.batchMax = false,
      this.none = false,
      List<String> on = const [],
      List<Member> members = const [],
      List<Choice> kinds = const [],
      List<Relation> relations = const [],
      this.namePointer,
      this.kindPointer,
      this.author})
      : choices = List.unmodifiable(choices),
        on = List.unmodifiable(on),
        members = List.unmodifiable(members),
        kinds = List.unmodifiable(kinds),
        relations = List.unmodifiable(relations);
}

// Private result/2 and input carriers; execution waits for the actual complete API.
import 'values.dart';
import 'read.dart';
import 'decision.dart';

part 'facts.dart';
part 'answers.dart';
part 'questions.dart';
part 'entities.dart';
part 'members.dart';
part 'atomic_results.dart';
part 'set_results.dart';
part 'question_specs.dart';
part 'plans.dart';
part 'sources.dart';
part 'annotation_specs.dart';
part 'errors.dart';

abstract interface class Observation {}

abstract interface class AtomicAnswer {}

abstract interface class AtomicQuestion {}

abstract interface class AnnotationEntry {}

abstract interface class RelationEntry {}

abstract interface class Result {}

abstract interface class QuestionSpec {}

abstract interface class RankSpec {}

abstract interface class Selection {}

abstract interface class AnnotationSpec {}

enum OriginKind {
  live,
  cache,
  replay,
  proxy,
  memory;
}

enum OutcomeKind {
  ok,
  status,
  transport;
}

enum CauseKind {
  missing_answer,
  wrong_kind,
  missing_probability,
  invalid_probability,
  invalid_distribution,
  unexpected_probability;
}

enum FailureKind {
  usage,
  backend,
  local,
  cancelled,
  deadline,
  defect;
}

enum UnitKind {
  line,
  window,
  file;
}

enum MediaKind {
  text,
  image;
}

enum MethodKind {
  yes_no,
  choice;
}

enum DirectionKind {
  source_to_target,
  either;
}

enum StopCauseKind {
  usage,
  local,
  no_key,
  transport,
  status,
  too_large,
  reply,
  backend,
  cancelled,
  defect,
  deadline;
}

enum MediaType { jpeg, png }

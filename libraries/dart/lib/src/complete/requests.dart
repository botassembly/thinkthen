// Private: no old C JSON execution path can satisfy complete parity.
import 'models.dart';
import 'values.dart';
import 'read.dart';

final class Request extends Carrier {
  final String function;
  final Carrier question;
  final Selection input;
  final Controls controls;
  const Request(this.function, this.question, this.input, this.controls);
  Object? questionJson() {
    if (question is QuestionFile)
      throw StateError('question files require the native loader');
    return project(question);
  }

  @override
  Map<String, Object?> toJson() => {
        'function': function,
        'question': project(question),
        'input': project(input),
        'controls': project(controls)
      };
}

abstract final class Requests {
  static Request _build(
      String verb, Carrier q, Selection input, Controls? controls) {
    // Validate and snapshot through the typed readers. Native owns semantic admission.
    q = _readQuestion(q);
    input = _readSelection(input);
    controls = controls == null
        ? const Controls()
        : Controls.fromJson(project(controls));
    final images = input is ImageInput ||
        (input is Files &&
            input.media.present &&
            input.media.value == MediaKind.image);
    if (images && !['decide', 'choose', 'score'].contains(verb))
      throw ArgumentError('this function is text-only');
    if (input is CandidateInput && verb != 'find')
      throw ArgumentError('candidates require find');
    if (input is RecordInput && verb == 'find')
      throw ArgumentError('find requires candidates');
    if (input is TextInput &&
        ['filter', 'rank', 'find', 'annotate', 'relate'].contains(verb))
      throw ArgumentError('this function requires a complete record set');
    if (input is Files) {
      if ((input.unit == UnitKind.window) != input.window.present)
        throw ArgumentError('window requires its size');
      if (images && input.unit != UnitKind.file)
        throw ArgumentError('image sources require file units');
    }
    if (input is ImageInput && input.images.isEmpty)
      throw ArgumentError('images require attachments');
    if (verb != 'rank' && controls.top.present)
      throw ArgumentError('top requires rank');
    if (verb != 'find' && controls.none.present)
      throw ArgumentError('none requires find');
    return Request(verb, q, input, controls);
  }

  static Carrier _readQuestion(Carrier q) {
    final v = project(q);
    switch (q) {
      case DecideSpec():
        return DecideSpec.fromJson(v);
      case ChooseSpec():
        return ChooseSpec.fromJson(v);
      case TagSpec():
        return TagSpec.fromJson(v);
      case ScoreSpec():
        return ScoreSpec.fromJson(v);
      case FindSpec():
        return FindSpec.fromJson(v);
      case QuestionFile():
        return QuestionFile.fromJson(v);
      case QuestionSet():
        return QuestionSet.fromJson(v);
      case RecognitionSpec():
        return RecognitionSpec.fromJson(v);
      case RelationSpec():
        return RelationSpec.fromJson(v);
      default:
        return invalid();
    }
  }

  static Selection _readSelection(Selection input) {
    final v = project(input);
    switch (input) {
      case TextInput():
        return TextInput.fromJson(v);
      case RecordInput():
        return RecordInput.fromJson(v);
      case CandidateInput():
        return CandidateInput.fromJson(v);
      case ImageInput():
        return ImageInput.fromJson(v);
      case Files():
        return Files.fromJson(v);
      default:
        return invalid();
    }
  }

  static Request decide(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is DecideSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('decide', question, input, controls);
  }

  static Request choose(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is ChooseSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('choose', question, input, controls);
  }

  static Request tag(Carrier question, Selection input, [Controls? controls]) {
    if (!(question is TagSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('tag', question, input, controls);
  }

  static Request score(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is ScoreSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('score', question, input, controls);
  }

  static Request filter(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is DecideSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('filter', question, input, controls);
  }

  static Request rank(Carrier question, Selection input, [Controls? controls]) {
    if (!(question is DecideSpec ||
        question is ScoreSpec ||
        question is QuestionSet ||
        question is QuestionFile)) throw ArgumentError('wrong question kind');
    return _build('rank', question, input, controls);
  }

  static Request find(Carrier question, Selection input, [Controls? controls]) {
    if (!(question is FindSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('find', question, input, controls);
  }

  static Request annotate(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is QuestionSet || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('annotate', question, input, controls);
  }

  static Request recognize(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is RecognitionSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('recognize', question, input, controls);
  }

  static Request relate(Carrier question, Selection input,
      [Controls? controls]) {
    if (!(question is RelationSpec || question is QuestionFile))
      throw ArgumentError('wrong question kind');
    return _build('relate', question, input, controls);
  }
}

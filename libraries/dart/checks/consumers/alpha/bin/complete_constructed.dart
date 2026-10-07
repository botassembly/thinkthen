import 'package:thinkthen_dart/thinkthen_complete.dart';

void main(List<String> args) {
  final engine = Engine(args[0], settingsJson: args[1]);
  try {
    final results = constructed(engine);
    engine.close();
    for (final r in results) {
      if (r.summary.schema.data != 'thinkthen.result/2')
        throw StateError('copied result expired');
    }
    print(
        'ten typed constructors executed; copied values survive close; wording version exact');
  } finally {
    engine.close();
  }
}

List<CompleteResult<Object>> constructed(CompleteApi engine) {
  final decide = Question.spec(QuestionSpec(FunctionKind.decide,
      text: const Content.text('Does this ask for a refund?'),
      author: Author(
          name: 'refund',
          wordingVersion: BigInt.from(2147483647),
          itemSchema: const Declaration.string())));
  final questions = <String, Question>{
    'decide': decide,
    'choose': Question.spec(QuestionSpec(FunctionKind.choose,
        text: const Content.text('Which?'),
        choices: [
          Choice('a', description: Content.json({'what': 'First'})),
          const Choice('b')
        ])),
    'tag': Question.spec(QuestionSpec(FunctionKind.tag,
        text: const Content.text('Labels?'),
        choices: [const Choice('a'), const Choice('b')])),
    'score': Question.spec(QuestionSpec(FunctionKind.score,
        text: const Content.text('Grade?'),
        choices: [const Choice('low'), const Choice('high')])),
    'filter': decide,
    'rank': decide,
    'find': Question.spec(QuestionSpec(FunctionKind.find,
        text: const Content.text('Which text?'), none: true)),
    'annotate': Question.spec(QuestionSpec(FunctionKind.annotate,
        members: [Member('refund', decide)])),
    'recognize': Question.spec(QuestionSpec(FunctionKind.recognize,
        kinds: [const Choice('person'), const Choice('place')])),
    'relate': Question.spec(QuestionSpec(FunctionKind.relate,
        relations: [const Relation('visits', 'person', 'place')]))
  };
  final results = <CompleteResult<Object>>[];
  for (final entry in questions.entries) {
    final verb = entry.key;
    final q = entry.value;
    final source = verb == 'relate'
        ? Records([
            Record(Content.json({'name': 'Alice', 'kind': 'person'})),
            Record(Content.json({'name': 'Paris', 'kind': 'place'}))
          ])
        : verb == 'find'
            ? Records([
                Record(const Content.text('Alice visited Paris.')),
                Record(const Content.text('Refund me.'))
              ])
            : Records([Record(const Content.text('Refund me please.'))]);
    final result = switch (verb) {
      'decide' => engine.decide(q, source),
      'choose' => engine.choose(q, source),
      'tag' => engine.tag(q, source),
      'score' => engine.score(q, source),
      'filter' => engine.filter(q, source),
      'rank' => engine.rank(q, source),
      'find' => engine.find(q, source),
      'annotate' => engine.annotate(q, source),
      'recognize' => engine.recognize(q, source),
      'relate' => engine.relate(q, source),
      _ => throw StateError('function')
    };
    if (result.summary.state != 1 ||
        result.summary.facts.present != 1 ||
        result.rows.isEmpty)
      throw StateError('typed constructor did not execute');
    if (verb == 'decide' &&
        result.authors.first.wording_version.value != BigInt.from(2147483647))
      throw StateError('wording version changed');
    if (verb == 'find') {
      final row = result.rows.first as FindView;
      if (result.summary.function.value != 7 ||
          row.common.question.value!.kind != 7 ||
          row.common.answer.value!.kind != 5)
        throw StateError('find ABI tags changed');
    }
    results.add(result);
  }
  final r = results[5];
  final legacy = CompleteResult(
      r.summary,
      r.rows,
      r.observations,
      r.details,
      r.authors,
      r.memberAuthors,
      r.rankMembers,
      r.observationDetails,
      r.observationAuthors,
      r.sourceRecognitions,
      r.sourceRelations);
  final supplied = <List<DetailsView>>[
    [r.details.first]
  ];
  final extended = CompleteResult(
      r.summary,
      r.rows,
      r.observations,
      r.details,
      r.authors,
      r.memberAuthors,
      r.rankMembers,
      r.observationDetails,
      r.observationAuthors,
      r.sourceRecognitions,
      r.sourceRelations,
      supplied);
  supplied.first.clear();
  supplied.clear();
  if (legacy.rankMemberDetails.isNotEmpty ||
      !identical(legacy.rows.first, r.rows.first) ||
      !identical(extended.rankMemberDetails.first.first, r.details.first))
    throw StateError('public result constructor compatibility');
  return results;
}

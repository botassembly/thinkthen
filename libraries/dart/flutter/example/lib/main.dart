import 'package:thinkthen_flutter/thinkthen_flutter.dart';

Future<void> showAnswers(Engine engine) async {
  final question = QuestionText(text: 'Does this ask for a refund?');
  final text = InputText(text: 'Please refund my order.');
  final records = InputRecords(
    items: [
      Item(original: Presence.present(OriginalText(text: 'Refund please.'))),
      Item(original: Presence.present(OriginalText(text: 'Thank you.'))),
    ],
  );
  print('decide: ${(await engine.decide(question, text)).value}');
  print(
    'choose: ${(await engine.choose(DefinedQuestion(
      value: ChooseQuestion(choose: const Wording.text('Which team?'), options: const Presence.present(ChoiceOptions.list(['billing', 'support']))),
    ), text)).value}',
  );
  print(
    'tag: ${(await engine.tag(DefinedQuestion(
      value: TagQuestion(tag: const Wording.text('Which topics?'), labels: const Presence.present(Labels.list(['billing', 'support']))),
    ), text)).value}',
  );
  print(
    'score: ${(await engine.score(DefinedQuestion(
      value: ScoreQuestion(score: const Wording.text('How urgent?'), levels: const Presence.present(Levels.list(['low', 'high']))),
    ), text)).value}',
  );
  print('filter: ${(await engine.filter(question, records)).value}');
  print('rank: ${(await engine.rank(question, records)).value}');
  print(
    'find: ${(await engine.find(QuestionText(text: 'Which line asks for a refund?'), InputUnits(items: records.items))).value}',
  );
  final annotations = (await engine.annotate(
    DefinedQuestion(
      value: AnnotateQuestion(
        questions: {
          'refund': AnnotateQuestionDecide(
            decide: const Wording.text('Does this ask for a refund?'),
          ),
        },
      ),
    ),
    records,
  )).value;
  print(
    'annotate: ${(annotations as List).map((row) => (row as Map).keys.toList()).toList()}',
  );
  final recognized = (await engine.recognize(
    DefinedQuestion(
      value: RecognizeQuestion(
        recognize: RecognitionSpec(
          kinds: const Presence.present({'person': 'Person.'}),
        ),
      ),
    ),
    InputText(text: 'Maria Chen'),
  )).value;
  print(
    'recognize: ${(recognized as RecognizeEntities).entities.length} results',
  );
  final relation = (await engine.relate(
    DefinedQuestion(
      value: RelateQuestion(
        relate: RelationSpec(
          relations: [
            RelationDefinition(
              name: 'knows',
              source: const Presence.present('person'),
              target: const Presence.present('person'),
            ),
          ],
        ),
      ),
    ),
    InputEntities(
      items: [
        Item(
          original: Presence.present(
            OriginalJson(value: {'name': 'Ana', 'kind': 'person'}),
          ),
        ),
        Item(
          original: Presence.present(
            OriginalJson(value: {'name': 'Bob', 'kind': 'person'}),
          ),
        ),
      ],
    ),
  )).value;
  print(
    'relate: ${relation.map((edge) => '${edge.sourceName} -> ${edge.targetName}').toList()}',
  );
}

Future<void> main() async {
  final engine = Engine.open();
  try {
    await showAnswers(engine);
  } finally {
    engine.close();
  }
}

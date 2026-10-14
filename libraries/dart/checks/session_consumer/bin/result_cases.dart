import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

import 'fixture_values.dart';

void requireResult(bool value, String message) {
  if (!value) throw StateError(message);
}

Future<void> callResultCases() async {
  final canonical = decodeFixture(
    File(Platform.environment['TT_CALL_RESULT_FIXTURE']!).readAsStringSync(),
  ) as Map;
  final pair = (canonical['parity']['required_cases'] as List).singleWhere(
    (row) => row['id'] == 'library-call-result',
  ) as Map;
  final cases = canonical['cases'] as List;
  final expected = pair['expect']['calls'] as List;
  final exchanges = pair['exchanges'] as List;
  var sends = 0, responseIndex = 0;
  var responses = exchanges;
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  final serving = server.listen((request) async {
    await utf8.decoder.bind(request).join();
    sends++;
    final response = responses[responseIndex++]['response'];
    request.response.headers.contentType = ContentType.json;
    request.response.write(jsonEncode(response));
    await request.response.close();
  });
  final engine = Engine.open(
    settings: EngineSettings(
      backend: const Presence.present('typesafe'),
      baseUrl: Presence.present('http://127.0.0.1:${server.port}'),
      model: const Presence.present('jev-1.13.0'),
      cache: const Presence.present(CacheDocument.boolean(false)),
      maxRetries: Presence.present(BigInt.zero),
    ),
  );
  try {
    final references = pair['input']['case_refs'] as List;
    for (var i = 0; i < references.length; i++) {
      final fixture =
          cases.singleWhere((row) => row['id'] == references[i]) as Map;
      final question = DefinedQuestion(
        value: Definition.read(fixture['question']),
      );
      final input = InputText(text: exchanges[i]['evidence']);
      final before = sends;
      final result = fixture['verb'] == 'decide'
          ? await engine.decide(question, input)
          : await engine.choose(question, input);
      final probability = result.probability as ScalarProbability;
      requireResult(
        result.value == expected[i]['value'] &&
            probability.value == expected[i]['probability'] &&
            result.results.length == expected[i]['results_count'] &&
            result.facts.requestsSent ==
                BigInt.from(expected[i]['requests_sent']) &&
            sends == before + expected[i]['requests_sent'],
        'canonical paired result ${references[i]}',
      );
    }
    final recognizedFixture = cases.singleWhere(
      (row) => row['id'] == '41-offsets-past-an-accent-and-an-emoji',
    ) as Map;
    final recognizeQuestion = DefinedQuestion(
      value: Definition.read(recognizedFixture['question']),
    );
    final recognizeText = recognizedFixture['text'] as String;
    responses = recognizedFixture['exchanges'] as List;
    responseIndex = 0;
    final recognized = await engine.recognize(
      recognizeQuestion,
      InputText(text: recognizeText),
    );
    final entity = (recognized.value as RecognizeEntities).entities.single;
    final span =
        (recognized.results.single.answer as RecognitionOddsRecognizeAnswer)
            .proposals
            .singleWhere(
              (proposal) =>
                  proposal.kept &&
                  proposal.kind.value == entity.kind &&
                  proposal.selected.value?.start == entity.start &&
                  proposal.selected.value?.end == entity.end,
            )
            .spanProbability
            .toDouble();
    requireResult(
      entity.text == 'Maria Chen' &&
          (recognized.probability as ListProbability).value.single == span &&
          span != entity.strength,
      'recognition reports kept span probability instead of strength',
    );
    responseIndex = 0;
    final recognizedList = await engine.recognize(
      recognizeQuestion,
      InputRecords(
        items: [
          Item(original: Presence.present(OriginalText(text: recognizeText))),
        ],
      ),
    );
    requireResult(
      (recognizedList.value as List).length == 1 &&
          (recognizedList.probability as GroupProbability)
                  .value
                  .single
                  .single ==
              span,
      'recognition list probability retains row and entity cardinality',
    );
    responseIndex = 0;
    final boundaryDefinition = Map<String, Object?>.from(
      recognizedFixture['question'] as Map,
    )..remove('relation_threshold');
    boundaryDefinition['recognize'] = {
      ...Map<String, Object?>.from(boundaryDefinition['recognize'] as Map),
      'mode': 'boundary_only',
    };
    final boundary = await engine.recognize(
      DefinedQuestion(value: Definition.read(boundaryDefinition)),
      InputText(text: recognizeText),
    );
    final proposal =
        (boundary.value as RecognizeModeProposals).proposals.single;
    requireResult(
      proposal.text == 'Maria Chen' &&
          (boundary.probability as ListProbability).value.single ==
              proposal.probability.toDouble(),
      'boundary recognition reports proposal probability',
    );
    print(
      'PASS: library-call-result actual value probability results and facts',
    );
  } finally {
    engine.close();
    await serving.cancel();
    await server.close(force: true);
  }
}

Future<void> authoredValueCases(Engine engine) async {
  final wide = BigInt.parse('18446744073709551615');
  for (final reading in <Object?>[
    ['authored', null, false, wide],
    {'label': 'authored', 'null': null, 'false': false, 'wide': wide},
  ]) {
    final call = await engine.decide(
      DefinedQuestion(
        value: DecideQuestion(
          decide: const Wording.text('Is it?'),
          trueValue: Presence.present(Reading.read(reading)),
        ),
      ),
      InputJson(value: {'original': wide}),
    );
    requireResult(
      encodeFixture(call.value) == encodeFixture(reading) &&
          (call.probability as ScalarProbability).value == 0.9 &&
          (call.results.single.input.value as Map)['original'] == wide,
      'authored collection and wide original remain exact',
    );
  }
}

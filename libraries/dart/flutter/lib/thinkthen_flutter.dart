/// Flutter-facing facade for the separately installed ThinkThen C archive.
library;

import 'dart:ffi';

import 'package:thinkthen_dart/thinkthen_dart.dart';
export 'package:thinkthen_dart/thinkthen_dart.dart'
    show
        Door,
        DoorFailure,
        AnswerValue,
        Outcome,
        ErrorKind,
        AnnotatedField,
        UnresolvedField,
        AnswerField,
        FailedField,
        readField;

class ThinkThenFlutter {
  final Door door;
  ThinkThenFlutter(String nativeLibraryPath) : door = Door(nativeLibraryPath);

  ({AnswerValue value, Map<String, Object?> facts}) decide(
    String question,
    String text,
  ) {
    final Pointer<Void> engine = door.create();
    try {
      return door.decide(engine, question, text);
    } finally {
      door.engineFree(engine);
    }
  }
}

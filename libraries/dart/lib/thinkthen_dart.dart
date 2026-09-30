/// Thin Dart bindings to a separately installed ThinkThen C native archive.
library;

export 'src/door.dart' show Door, DoorFailure, AnswerValue, Answer;
export 'src/typed.dart'
    show
        Outcome,
        ErrorKind,
        AnnotatedField,
        UnresolvedField,
        AnswerField,
        FailedField,
        readField;

import 'views.dart';
import '../typed.dart' show ErrorKind;

/// Known fields are native typed copies. Only caller content can contain JSON.
final class CompleteResult<T> {
  final SummaryView summary;
  final List<T> rows;
  final List<ObservationView> observations;
  final List<DetailsView> details, observationDetails;
  final List<QuestionAuthorView> authors, observationAuthors;
  final List<List<QuestionAuthorView>> memberAuthors;
  final List<List<RankView>> rankMembers;
  final List<List<DetailsView>> rankMemberDetails;
  final List<SourceRecognitionView?> sourceRecognitions;
  final List<SourceRelationsView?> sourceRelations;
  CompleteResult(
      this.summary,
      List<T> rows,
      List<ObservationView> observations,
      List<DetailsView> details,
      List<QuestionAuthorView> authors,
      List<List<QuestionAuthorView>> memberAuthors,
      List<List<RankView>> rankMembers,
      List<DetailsView> observationDetails,
      List<QuestionAuthorView> observationAuthors,
      List<SourceRecognitionView?> sourceRecognitions,
      List<SourceRelationsView?> sourceRelations,
      [List<List<DetailsView>> rankMemberDetails = const []])
      : rows = List.unmodifiable(rows),
        observations = List.unmodifiable(observations),
        details = List.unmodifiable(details),
        authors = List.unmodifiable(authors),
        memberAuthors = List.unmodifiable(
            memberAuthors.map(List<QuestionAuthorView>.unmodifiable)),
        rankMembers =
            List.unmodifiable(rankMembers.map(List<RankView>.unmodifiable)),
        rankMemberDetails = List.unmodifiable(
            rankMemberDetails.map(List<DetailsView>.unmodifiable)),
        observationDetails = List.unmodifiable(observationDetails),
        observationAuthors = List.unmodifiable(observationAuthors),
        sourceRecognitions = List.unmodifiable(sourceRecognitions),
        sourceRelations = List.unmodifiable(sourceRelations);
}

final class CompleteFailure implements Exception {
  final SummaryView summary;
  const CompleteFailure(this.summary);
  ErrorKind get kind => ErrorKind.values[summary.error.value!.code - 1];
  String get message => summary.error.value!.message.data;
  bool get retryable => summary.error.value!.retryable != 0;
  @override
  String toString() => 'CompleteFailure($kind, $message, retryable=$retryable)';
}

import 'package:thinkthen_dart/thinkthen_complete.dart';

Object? encodeNative(Object? v) {
  if (v == null || v is String || v is int || v is double || v is bool)
    return v;
  if (v is BigInt) return v.toString();
  if (v is List) return v.map(encodeNative).toList();
  if (v is CompleteResult)
    return {
      'summary': encodeNative(v.summary),
      'rows': encodeNative(v.rows),
      'observations': encodeNative(v.observations),
      'details': encodeNative(v.details),
      'authors': encodeNative(v.authors),
      'memberAuthors': encodeNative(v.memberAuthors),
      'rankMembers': encodeNative(v.rankMembers),
      'observationDetails': encodeNative(v.observationDetails),
      'observationAuthors': encodeNative(v.observationAuthors),
      'sourceRecognitions': encodeNative(v.sourceRecognitions),
      'sourceRelations': encodeNative(v.sourceRelations)
    };
  if (v is LegacyAnswerView)
    return {
      'outcome': encodeNative(v.outcome),
      'probability': encodeNative(v.probability)
    };
  if (v is StringView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is StringsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is OptionalStringView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is OptionalSizeView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is OptionalU64View)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is OptionalU16View)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is OptionalDoubleView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is OptionalDiscriminatorView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is ContentView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is OptionalContentView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is RuleView)
    return {
      'kind': encodeNative(v.kind),
      'low': encodeNative(v.low),
      'high': encodeNative(v.high)
    };
  if (v is OptionalRuleView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is ChoiceView)
    return {
      'name': encodeNative(v.name),
      'description': encodeNative(v.description),
      'weight': encodeNative(v.weight)
    };
  if (v is ChoicesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is RelationView)
    return {
      'name': encodeNative(v.name),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'reads': encodeNative(v.reads),
      'either': encodeNative(v.either),
      'single': encodeNative(v.single)
    };
  if (v is RelationsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is QuestionMemberView)
    return {'name': encodeNative(v.name), 'question': encodeNative(v.question)};
  if (v is QuestionMembersView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is QuestionView)
    return {
      'kind': encodeNative(v.kind),
      'text': encodeNative(v.text),
      'yes': encodeNative(v.yes),
      'no': encodeNative(v.no),
      'choices': encodeNative(v.choices),
      'threshold': encodeNative(v.threshold),
      'relation_threshold': encodeNative(v.relation_threshold),
      'model': encodeNative(v.model),
      'profile': encodeNative(v.profile),
      'batch': encodeNative(v.batch),
      'batch_max': encodeNative(v.batch_max),
      'none': encodeNative(v.none),
      'on': encodeNative(v.on),
      'members': encodeNative(v.members),
      'kinds': encodeNative(v.kinds),
      'relations': encodeNative(v.relations),
      'name_pointer': encodeNative(v.name_pointer),
      'kind_pointer': encodeNative(v.kind_pointer)
    };
  if (v is OptionalQuestionView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is ImageView)
    return {
      'media': encodeNative(v.media),
      'bytes': v.bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join(),
      'bytes_len': encodeNative(v.bytes_len),
      'width': encodeNative(v.width),
      'height': encodeNative(v.height),
      'filename': encodeNative(v.filename)
    };
  if (v is ImageViewsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is OptionalImageViewsView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is SourceSpecView)
    return {
      'paths': encodeNative(v.paths),
      'unit': encodeNative(v.unit),
      'window': encodeNative(v.window)
    };
  if (v is DecideValueV1DataView)
    return {
      'boolean': encodeNative(v.boolean),
      'authored': encodeNative(v.authored)
    };
  if (v is DecideValueView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is ProbabilityView)
    return {
      'name': encodeNative(v.name),
      'probability': encodeNative(v.probability)
    };
  if (v is ProbabilitiesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is OptionalProbabilitiesView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is NamedAnswerView)
    return {
      'pick': encodeNative(v.pick),
      'probabilities': encodeNative(v.probabilities),
      'confidence': encodeNative(v.confidence)
    };
  if (v is ScoreAnswerView)
    return {
      'level': encodeNative(v.level),
      'probabilities': encodeNative(v.probabilities),
      'confidence': encodeNative(v.confidence)
    };
  if (v is AnswerV1DataView)
    return {
      'probability': encodeNative(v.probability),
      'choice': encodeNative(v.choice),
      'tag': encodeNative(v.tag),
      'score': encodeNative(v.score),
      'find': encodeNative(v.find)
    };
  if (v is AnswerView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is OptionalAnswerView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is LocationView)
    return {
      'file': encodeNative(v.file),
      'first_line': encodeNative(v.first_line),
      'last_line': encodeNative(v.last_line)
    };
  if (v is OptionalLocationView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is MemberValueV1DataView)
    return {
      'decide': encodeNative(v.decide),
      'choose': encodeNative(v.choose),
      'tag': encodeNative(v.tag),
      'score': encodeNative(v.score)
    };
  if (v is MemberValueView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is MemberFailureView)
    return {
      'failure_id': encodeNative(v.failure_id),
      'cause': encodeNative(v.cause)
    };
  if (v is MemberSuccessView)
    return {
      'answer_id': encodeNative(v.answer_id),
      'value': encodeNative(v.value),
      'answer': encodeNative(v.answer),
      'threshold': encodeNative(v.threshold)
    };
  if (v is MemberV1DataView)
    return {
      'success': encodeNative(v.success),
      'failure': encodeNative(v.failure)
    };
  if (v is MemberView)
    return {
      'name': encodeNative(v.name),
      'request': encodeNative(v.request),
      'question': encodeNative(v.question),
      'state': encodeNative(v.state),
      'data': encodeNative(v.data)
    };
  if (v is MembersView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is EntityView)
    return {
      'text': encodeNative(v.text),
      'start': encodeNative(v.start),
      'end': encodeNative(v.end),
      'length': encodeNative(v.length),
      'kind': encodeNative(v.kind),
      'strength': encodeNative(v.strength)
    };
  if (v is EntitiesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is EntityEdgeView)
    return {
      'relation': encodeNative(v.relation),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'probability': encodeNative(v.probability),
      'either': encodeNative(v.either)
    };
  if (v is EntityEdgesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is OptionalEntityEdgesView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is PlaceView)
    return {'start': encodeNative(v.start), 'end': encodeNative(v.end)};
  if (v is PieceView)
    return {
      'start': encodeNative(v.start),
      'end': encodeNative(v.end),
      'tags': encodeNative(v.tags)
    };
  if (v is PiecesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is NameView)
    return {
      'start': encodeNative(v.start),
      'end': encodeNative(v.end),
      'kinds': encodeNative(v.kinds),
      'edges': encodeNative(v.edges)
    };
  if (v is NamesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is PairView)
    return {
      'relation': encodeNative(v.relation),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'probability': encodeNative(v.probability)
    };
  if (v is PairsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is RecognizeValueView)
    return {
      'entities': encodeNative(v.entities),
      'relations': encodeNative(v.relations)
    };
  if (v is RecognizeAnswerView)
    return {
      'pieces': encodeNative(v.pieces),
      'names': encodeNative(v.names),
      'pairs': encodeNative(v.pairs)
    };
  if (v is EndpointView)
    return {'name': encodeNative(v.name), 'kind': encodeNative(v.kind)};
  if (v is OptionalEndpointView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is EdgeView)
    return {
      'relation': encodeNative(v.relation),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'probability': encodeNative(v.probability),
      'either': encodeNative(v.either)
    };
  if (v is EdgesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is RelationSuccessView)
    return {
      'answer_id': encodeNative(v.answer_id),
      'probability': encodeNative(v.probability),
      'accepted': encodeNative(v.accepted)
    };
  if (v is RelationAnswerV1DataView)
    return {
      'success': encodeNative(v.success),
      'failure': encodeNative(v.failure)
    };
  if (v is RelationAnswerView)
    return {
      'relation': encodeNative(v.relation),
      'reads': encodeNative(v.reads),
      'method': encodeNative(v.method),
      'direction': encodeNative(v.direction),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'request': encodeNative(v.request),
      'state': encodeNative(v.state),
      'data': encodeNative(v.data)
    };
  if (v is RelationAnswersView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is UsageView)
    return {
      'input_tokens': encodeNative(v.input_tokens),
      'output_tokens': encodeNative(v.output_tokens)
    };
  if (v is OptionalUsageView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is QuestionSourceView)
    return {
      'origin': encodeNative(v.origin),
      'answered_by': encodeNative(v.answered_by)
    };
  if (v is QuestionSourcesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is ObservationIdentityV1DataView)
    return {
      'observation_id': encodeNative(v.observation_id),
      'failure_id': encodeNative(v.failure_id)
    };
  if (v is ObservationIdentityView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is ObservationIdentitiesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is ProfileWarningView)
    return {
      'tuned_for': encodeNative(v.tuned_for),
      'running': encodeNative(v.running)
    };
  if (v is OptionalProfileWarningView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is BatchView)
    return {'kind': encodeNative(v.kind), 'records': encodeNative(v.records)};
  if (v is OptionalBatchView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is BatchWarningView)
    return {
      'tuned_for': encodeNative(v.tuned_for),
      'running': encodeNative(v.running)
    };
  if (v is OptionalBatchWarningView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is AttemptView)
    return {
      'ordinal': encodeNative(v.ordinal),
      'request_sha256': encodeNative(v.request_sha256),
      'wall_ms': encodeNative(v.wall_ms),
      'outcome': encodeNative(v.outcome),
      'sdk_request_id': encodeNative(v.sdk_request_id),
      'status': encodeNative(v.status),
      'server_ms': encodeNative(v.server_ms),
      'request_id': encodeNative(v.request_id)
    };
  if (v is AttemptsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is OptionalAttemptsView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is MetaView)
    return {
      'tool': encodeNative(v.tool),
      'question_sha256': encodeNative(v.question_sha256),
      'questions_sha256': encodeNative(v.questions_sha256),
      'url': encodeNative(v.url),
      'model': encodeNative(v.model),
      'usage': encodeNative(v.usage),
      'requests_sent': encodeNative(v.requests_sent),
      'cached': encodeNative(v.cached),
      'requests': encodeNative(v.requests),
      'failed_questions': encodeNative(v.failed_questions),
      'profile_warning': encodeNative(v.profile_warning),
      'batch_setting': encodeNative(v.batch_setting),
      'batch_warning': encodeNative(v.batch_warning),
      'context_sha256': encodeNative(v.context_sha256),
      'attempts': encodeNative(v.attempts),
      'origin': encodeNative(v.origin),
      'question_sources': encodeNative(v.question_sources),
      'observations': encodeNative(v.observations),
      'answered_by': encodeNative(v.answered_by)
    };
  if (v is OptionalMetaView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is FactsView)
    return {
      'call_id': encodeNative(v.call_id),
      'cache_answers': encodeNative(v.cache_answers),
      'estimated_cost_usd': encodeNative(v.estimated_cost_usd),
      'input_tokens': encodeNative(v.input_tokens),
      'model': encodeNative(v.model),
      'output_tokens': encodeNative(v.output_tokens),
      'records': encodeNative(v.records),
      'requests_sent': encodeNative(v.requests_sent),
      'seconds': encodeNative(v.seconds),
      'command_ms': encodeNative(v.command_ms)
    };
  if (v is OptionalFactsView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is StoppedView)
    return {
      'at': encodeNative(v.at),
      'cause': encodeNative(v.cause),
      'status': encodeNative(v.status),
      'retryable': encodeNative(v.retryable)
    };
  if (v is OptionalStoppedView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is ErrorView)
    return {
      'code': encodeNative(v.code),
      'message': encodeNative(v.message),
      'retryable': encodeNative(v.retryable),
      'stopped': encodeNative(v.stopped)
    };
  if (v is OptionalErrorView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is RowView)
    return {
      'answer_id': encodeNative(v.answer_id),
      'input': encodeNative(v.input),
      'question': encodeNative(v.question),
      'answer': encodeNative(v.answer),
      'threshold': encodeNative(v.threshold),
      'position': encodeNative(v.position),
      'input_file': encodeNative(v.input_file),
      'meta': encodeNative(v.meta),
      'images': encodeNative(v.images)
    };
  if (v is DecideView)
    return {'common': encodeNative(v.common), 'value': encodeNative(v.value)};
  if (v is ChooseView)
    return {'common': encodeNative(v.common), 'value': encodeNative(v.value)};
  if (v is TagView)
    return {'common': encodeNative(v.common), 'value': encodeNative(v.value)};
  if (v is ScoreView)
    return {'common': encodeNative(v.common), 'value': encodeNative(v.value)};
  if (v is FilterView)
    return {'common': encodeNative(v.common), 'value': encodeNative(v.value)};
  if (v is RankView)
    return {
      'common': encodeNative(v.common),
      'value': encodeNative(v.value),
      'question_name': encodeNative(v.question_name)
    };
  if (v is FindView)
    return {
      'common': encodeNative(v.common),
      'value': encodeNative(v.value),
      'index': encodeNative(v.index)
    };
  if (v is AnnotateView)
    return {
      'common': encodeNative(v.common),
      'answers': encodeNative(v.answers)
    };
  if (v is RecognizeView)
    return {
      'common': encodeNative(v.common),
      'value': encodeNative(v.value),
      'answer': encodeNative(v.answer)
    };
  if (v is RelateView)
    return {
      'common': encodeNative(v.common),
      'value': encodeNative(v.value),
      'questions': encodeNative(v.questions)
    };
  if (v is ObservedProbabilitiesV1DataView)
    return {'yes': encodeNative(v.yes), 'named': encodeNative(v.named)};
  if (v is ObservedProbabilitiesView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is ObservationSuccessView)
    return {
      'answer_id': encodeNative(v.answer_id),
      'observation_id': encodeNative(v.observation_id),
      'value': encodeNative(v.value),
      'probabilities': encodeNative(v.probabilities),
      'confidence': encodeNative(v.confidence)
    };
  if (v is QuestionObservationV1DataView)
    return {
      'success': encodeNative(v.success),
      'failure': encodeNative(v.failure)
    };
  if (v is QuestionObservationView)
    return {
      'index': encodeNative(v.index),
      'member': encodeNative(v.member),
      'stage': encodeNative(v.stage),
      'position': encodeNative(v.position),
      'question_sha256': encodeNative(v.question_sha256),
      'model': encodeNative(v.model),
      'url': encodeNative(v.url),
      'requests': encodeNative(v.requests),
      'requests_sent': encodeNative(v.requests_sent),
      'cached': encodeNative(v.cached),
      'failed_questions': encodeNative(v.failed_questions),
      'usage': encodeNative(v.usage),
      'question_sources': encodeNative(v.question_sources),
      'state': encodeNative(v.state),
      'data': encodeNative(v.data)
    };
  if (v is RowObservationV1DataView)
    return {
      'decide': encodeNative(v.decide),
      'choose': encodeNative(v.choose),
      'tag': encodeNative(v.tag),
      'score': encodeNative(v.score),
      'filter': encodeNative(v.filter),
      'rank': encodeNative(v.rank),
      'find': encodeNative(v.find),
      'annotate': encodeNative(v.annotate),
      'recognize': encodeNative(v.recognize),
      'relate': encodeNative(v.relate)
    };
  if (v is RowObservationView)
    return {
      'index': encodeNative(v.index),
      'function': encodeNative(v.function),
      'data': encodeNative(v.data)
    };
  if (v is ObservationV1DataView)
    return {'question': encodeNative(v.question), 'row': encodeNative(v.row)};
  if (v is ObservationView)
    return {'kind': encodeNative(v.kind), 'data': encodeNative(v.data)};
  if (v is SummaryView)
    return {
      'state': encodeNative(v.state),
      'schema': encodeNative(v.schema),
      'answer_id': encodeNative(v.answer_id),
      'function': encodeNative(v.function),
      'count': encodeNative(v.count),
      'observation_count': encodeNative(v.observation_count),
      'meta': encodeNative(v.meta),
      'facts': encodeNative(v.facts),
      'attempts': encodeNative(v.attempts),
      'error': encodeNative(v.error)
    };
  if (v is ReportedUsageView)
    return {
      'present': encodeNative(v.present),
      'input_tokens': encodeNative(v.input_tokens),
      'output_tokens': encodeNative(v.output_tokens)
    };
  if (v is SourceDetailView)
    return {
      'origin': encodeNative(v.origin),
      'answered_by': encodeNative(v.answered_by),
      'batch_size': encodeNative(v.batch_size)
    };
  if (v is SourceDetailsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is InputView)
    return {
      'original': encodeNative(v.original),
      'position': encodeNative(v.position),
      'images': encodeNative(v.images)
    };
  if (v is InputViewsView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is DetailsView)
    return {
      'question': encodeNative(v.question),
      'threshold': encodeNative(v.threshold),
      'raw_pick': encodeNative(v.raw_pick),
      'usage': encodeNative(v.usage),
      'question_sources': encodeNative(v.question_sources),
      'observations': encodeNative(v.observations),
      'inputs': encodeNative(v.inputs)
    };
  if (v is SourceEntityView)
    return {
      'entity': encodeNative(v.entity),
      'position': encodeNative(v.position)
    };
  if (v is SourceEntitiesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is SourceEntityEdgeView)
    return {
      'relation': encodeNative(v.relation),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'probability': encodeNative(v.probability),
      'either': encodeNative(v.either)
    };
  if (v is SourceEntityEdgesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is OptionalSourceEntityEdgesView)
    return {'present': encodeNative(v.present), 'value': encodeNative(v.value)};
  if (v is SourceRecognitionView)
    return {
      'present': encodeNative(v.present),
      'entities': encodeNative(v.entities),
      'relations': encodeNative(v.relations)
    };
  if (v is SourceEndpointView)
    return {
      'ordinal': encodeNative(v.ordinal),
      'endpoint': encodeNative(v.endpoint),
      'record': encodeNative(v.record),
      'position': encodeNative(v.position)
    };
  if (v is SourceEdgeView)
    return {
      'relation': encodeNative(v.relation),
      'source': encodeNative(v.source),
      'target': encodeNative(v.target),
      'probability': encodeNative(v.probability),
      'either': encodeNative(v.either)
    };
  if (v is SourceEdgesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is SourceRelationsView)
    return {'present': encodeNative(v.present), 'edges': encodeNative(v.edges)};
  if (v is InputPropertyView)
    return {'name': encodeNative(v.name), 'kind': encodeNative(v.kind)};
  if (v is InputPropertiesView)
    return {'data': encodeNative(v.data), 'len': encodeNative(v.len)};
  if (v is InputDeclarationView)
    return {
      'kind': encodeNative(v.kind),
      'properties': encodeNative(v.properties),
      'required': encodeNative(v.required)
    };
  if (v is QuestionAuthorView)
    return {
      'name': encodeNative(v.name),
      'wording_version': encodeNative(v.wording_version),
      'item_schema': encodeNative(v.item_schema),
      'context_schema': encodeNative(v.context_schema)
    };
  throw StateError('untyped native field ${v.runtimeType}');
}

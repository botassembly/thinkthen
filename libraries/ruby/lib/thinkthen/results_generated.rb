# Generated from the shared Rust result graph; do not edit.
module ThinkThen
  module Results
    class NativeAnnotation < Native::Result
      define_method('answer_id') { self['answer_id'] }
      define_method('answers') { self['answers'] }
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('last_line') { self['last_line'] }
      define_method('meta') { self['meta'] }
      define_method('position') { self['position'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('value') { self['value'] }
    end
    class NativeAnnotationMemberAnswerId < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('observations') { self['observations'] }
      define_method('question') { self['question'] }
      define_method('question_sources') { self['question_sources'] }
      define_method('request') { self['request'] }
      define_method('threshold') { self['threshold'] }
      define_method('usage') { self['usage'] }
      define_method('value') { self['value'] }
    end
    class NativeAnnotationMemberFailureId < Native::Result
      define_method('failure') { self['failure'] }
      define_method('failure_id') { self['failure_id'] }
      define_method('observations') { self['observations'] }
      define_method('question') { self['question'] }
      define_method('question_sources') { self['question_sources'] }
      define_method('request') { self['request'] }
      define_method('threshold') { self['threshold'] }
      define_method('usage') { self['usage'] }
    end
    class NativeAnnotationValueChoice < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeAnnotationValueDecision < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeAnnotationValueFailed < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeAnnotationValueScore < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeAnnotationValueTags < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeAnswers < Native::Result
      define_method('questions') { self['questions'] }
    end
    class NativeAtomicArrayOfString < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('members') { self['members'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('question_name') { self['question_name'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeAtomicDecideValue < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('members') { self['members'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('question_name') { self['question_name'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeAtomicNonZeroUsize < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('members') { self['members'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('question_name') { self['question_name'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeAtomicNullableString < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('members') { self['members'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('question_name') { self['question_name'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeAtomicBoolean < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('members') { self['members'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('question_name') { self['question_name'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeAtomicDouble < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('members') { self['members'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('question_name') { self['question_name'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeAttempt < Native::Result
      define_method('ordinal') { self['ordinal'] }
      define_method('outcome') { self['outcome'] }
      define_method('request_id') { self['request_id'] }
      define_method('request_sha256') { self['request_sha256'] }
      define_method('sdk_request_id') { self['sdk_request_id'] }
      define_method('server_ms') { self['server_ms'] }
      define_method('status') { self['status'] }
      define_method('wall_ms') { self['wall_ms'] }
    end
    class NativeBoundaryOdds < Native::Result
      define_method('pieces') { self['pieces'] }
      define_method('proposals') { self['proposals'] }
    end
    class NativeBoundaryProposal < Native::Result
      define_method('end') { self['end'] }
      define_method('length') { self['length'] }
      define_method('probability') { self['probability'] }
      define_method('start') { self['start'] }
      define_method('text') { self['text'] }
    end
    class NativeCallError < Native::Result
      define_method('error') { self['error'] }
      define_method('facts') { self['facts'] }
    end
    class NativeEntityDocument < Native::Result
      define_method('kind') { self['kind'] }
      define_method('name') { self['name'] }
    end
    class NativeError < Native::Result
      define_method('estimated_input_denial') { self['estimated_input_denial'] }
      define_method('kind') { self['kind'] }
      define_method('message') { self['message'] }
      define_method('retryable') { self['retryable'] }
      define_method('send_budget_denial') { self['send_budget_denial'] }
      define_method('stopped') { self['stopped'] }
    end
    class NativeEstimatedInputDenialAdditionalRequest < Native::Result
      define_method('kind') { self['kind'] }
      define_method('limit') { self['limit'] }
    end
    class NativeEstimatedInputDenialInitialRequest < Native::Result
      define_method('kind') { self['kind'] }
      define_method('limit') { self['limit'] }
    end
    class NativeEstimatedInputDenialRetry < Native::Result
      define_method('kind') { self['kind'] }
      define_method('last_status') { self['last_status'] }
      define_method('limit') { self['limit'] }
    end
    class NativeFacts < Native::Result
      define_method('attempts') { self['attempts'] }
      define_method('cache_answers') { self['cache_answers'] }
      define_method('call_id') { self['call_id'] }
      define_method('estimated_cost_usd') { self['estimated_cost_usd'] }
      define_method('held_model_mismatch') { self['held_model_mismatch'] }
      define_method('input_tokens') { self['input_tokens'] }
      define_method('largest_request_bytes') { self['largest_request_bytes'] }
      define_method('largest_request_estimated_input_tokens') { self['largest_request_estimated_input_tokens'] }
      define_method('model') { self['model'] }
      define_method('output_tokens') { self['output_tokens'] }
      define_method('records') { self['records'] }
      define_method('requests_sent') { self['requests_sent'] }
      define_method('seconds') { self['seconds'] }
      define_method('token_estimate_method') { self['token_estimate_method'] }
      define_method('usage_persistence') { self['usage_persistence'] }
    end
    class NativeFind < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('candidates') { self['candidates'] }
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('index') { self['index'] }
      define_method('last_line') { self['last_line'] }
      define_method('meta') { self['meta'] }
      define_method('position') { self['position'] }
      define_method('question') { self['question'] }
      define_method('schema') { self['schema'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeFindCandidate < Native::Result
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('probability') { self['probability'] }
      define_method('source') { self['source'] }
    end
    class NativeImage < Native::Result
      define_method('base64') { self['base64'] }
      define_method('height') { self['height'] }
      define_method('media') { self['media'] }
      define_method('width') { self['width'] }
    end
    class NativeInputDeclarationObject < Native::Result
      define_method('properties') { self['properties'] }
      define_method('required') { self['required'] }
      define_method('type') { self['type'] }
    end
    class NativeInputDeclarationString < Native::Result
      define_method('type') { self['type'] }
    end
    class NativeInputPropertyTypeArray < Native::Result
      define_method('items') { self['items'] }
      define_method('type') { self['type'] }
    end
    class NativeInputPropertyTypeBoolean < Native::Result
      define_method('type') { self['type'] }
    end
    class NativeInputPropertyTypeNumber < Native::Result
      define_method('type') { self['type'] }
    end
    class NativeInputPropertyTypeString < Native::Result
      define_method('type') { self['type'] }
    end
    class NativeLabel < Native::Result
      define_method('description') { self['description'] }
      define_method('name') { self['name'] }
    end
    class NativeMeta < Native::Result
      define_method('answered_by') { self['answered_by'] }
      define_method('attempts') { self['attempts'] }
      define_method('batch_setting') { self['batch_setting'] }
      define_method('batch_warning') { self['batch_warning'] }
      define_method('cached') { self['cached'] }
      define_method('context_sha256') { self['context_sha256'] }
      define_method('failed_questions') { self['failed_questions'] }
      define_method('model') { self['model'] }
      define_method('observations') { self['observations'] }
      define_method('origin') { self['origin'] }
      define_method('profile_warning') { self['profile_warning'] }
      define_method('question_sha256') { self['question_sha256'] }
      define_method('question_sources') { self['question_sources'] }
      define_method('questions_sha256') { self['questions_sha256'] }
      define_method('requests') { self['requests'] }
      define_method('requests_sent') { self['requests_sent'] }
      define_method('tool') { self['tool'] }
      define_method('url') { self['url'] }
      define_method('usage') { self['usage'] }
    end
    class NativeObjectRoot < Native::Result
      define_method('properties') { self['properties'] }
      define_method('required') { self['required'] }
      define_method('type') { self['type'] }
    end
    class NativeObservationFailureId < Native::Result
      define_method('failure_id') { self['failure_id'] }
    end
    class NativeObservationObservationId < Native::Result
      define_method('observation_id') { self['observation_id'] }
    end
    class NativePersistenceObservation < Native::Result
      define_method('advice') { self['advice'] }
      define_method('observed_at') { self['observed_at'] }
      define_method('state') { self['state'] }
    end
    class NativePhysicalSource < Native::Result
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('last_line') { self['last_line'] }
    end
    class NativePosition < Native::Result
      define_method('file') { self['file'] }
      define_method('first') { self['first'] }
      define_method('images') { self['images'] }
      define_method('last') { self['last'] }
    end
    class NativeQuestionSource < Native::Result
      define_method('answered_by') { self['answered_by'] }
      define_method('batch_size') { self['batch_size'] }
      define_method('origin') { self['origin'] }
    end
    class NativeRankMember < Native::Result
      define_method('name') { self['name'] }
      define_method('result') { self['result'] }
    end
    class NativeRankMemberResult < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('images') { self['images'] }
      define_method('meta') { self['meta'] }
      define_method('question') { self['question'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('threshold') { self['threshold'] }
      define_method('value') { self['value'] }
    end
    class NativeReadableQuestion2 < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('label_details') { self['label_details'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('none') { self['none'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('text') { self['text'] }
      define_method('verb') { self['verb'] }
      define_method('wording_version') { self['wording_version'] }
    end
    class NativeReadableQuestion3 < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('entity_definition') { self['entity_definition'] }
      define_method('instructions') { self['instructions'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('kinds') { self['kinds'] }
      define_method('label_details') { self['label_details'] }
      define_method('mode') { self['mode'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('relation_threshold') { self['relation_threshold'] }
      define_method('relations') { self['relations'] }
      define_method('snippet_pieces') { self['snippet_pieces'] }
      define_method('stage_context') { self['stage_context'] }
      define_method('threshold') { self['threshold'] }
      define_method('verb') { self['verb'] }
      define_method('wording_version') { self['wording_version'] }
    end
    class NativeReadableQuestion4 < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('fields') { self['fields'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('label_details') { self['label_details'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('relations') { self['relations'] }
      define_method('threshold') { self['threshold'] }
      define_method('verb') { self['verb'] }
      define_method('wording_version') { self['wording_version'] }
    end
    class NativeReadableQuestionChoose < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('label_details') { self['label_details'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('wording_version') { self['wording_version'] }
      define_method('options') { self['options'] }
      define_method('text') { self['text'] }
      define_method('verb') { self['verb'] }
    end
    class NativeReadableQuestionDecide < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('label_details') { self['label_details'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('wording_version') { self['wording_version'] }
      define_method('false') { self['false'] }
      define_method('text') { self['text'] }
      define_method('true') { self['true'] }
      define_method('verb') { self['verb'] }
    end
    class NativeReadableQuestionScore < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('label_details') { self['label_details'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('wording_version') { self['wording_version'] }
      define_method('levels') { self['levels'] }
      define_method('text') { self['text'] }
      define_method('verb') { self['verb'] }
    end
    class NativeReadableQuestionTag < Native::Result
      define_method('batch') { self['batch'] }
      define_method('context_schema') { self['context_schema'] }
      define_method('item_schema') { self['item_schema'] }
      define_method('label_details') { self['label_details'] }
      define_method('model') { self['model'] }
      define_method('name') { self['name'] }
      define_method('on') { self['on'] }
      define_method('profile') { self['profile'] }
      define_method('wording_version') { self['wording_version'] }
      define_method('labels') { self['labels'] }
      define_method('text') { self['text'] }
      define_method('verb') { self['verb'] }
    end
    class NativeRecognition < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('last_line') { self['last_line'] }
      define_method('meta') { self['meta'] }
      define_method('position') { self['position'] }
      define_method('question') { self['question'] }
      define_method('schema') { self['schema'] }
      define_method('source') { self['source'] }
      define_method('value') { self['value'] }
    end
    class NativeRecognitionEdgeDocument < Native::Result
      define_method('either') { self['either'] }
      define_method('probability') { self['probability'] }
      define_method('relation') { self['relation'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
    end
    class NativeRecognitionOddsFieldsNamesPairsPiecesProposals < Native::Result
      define_method('names') { self['names'] }
      define_method('pairs') { self['pairs'] }
      define_method('pieces') { self['pieces'] }
      define_method('proposals') { self['proposals'] }
    end
    class NativeRecognitionOddsFieldsPiecesProposals < Native::Result
      define_method('pieces') { self['pieces'] }
      define_method('proposals') { self['proposals'] }
    end
    class NativeRecognitionProposal < Native::Result
      define_method('end') { self['end'] }
      define_method('kept') { self['kept'] }
      define_method('kind') { self['kind'] }
      define_method('selected') { self['selected'] }
      define_method('span_probability') { self['span_probability'] }
      define_method('start') { self['start'] }
      define_method('strength') { self['strength'] }
    end
    class NativeRecognitionStageContext < Native::Result
      define_method('boundary') { self['boundary'] }
      define_method('kind_edge') { self['kind_edge'] }
      define_method('relation') { self['relation'] }
    end
    class NativeRelation < Native::Result
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('index') { self['index'] }
      define_method('input') { self['input'] }
      define_method('input_sources') { self['input_sources'] }
      define_method('last_line') { self['last_line'] }
      define_method('meta') { self['meta'] }
      define_method('position') { self['position'] }
      define_method('question') { self['question'] }
      define_method('schema') { self['schema'] }
      define_method('value') { self['value'] }
    end
    class NativeRelationMemberAnswerId < Native::Result
      define_method('direction') { self['direction'] }
      define_method('method') { self['method'] }
      define_method('observations') { self['observations'] }
      define_method('question') { self['question'] }
      define_method('question_sources') { self['question_sources'] }
      define_method('reads') { self['reads'] }
      define_method('relation') { self['relation'] }
      define_method('request') { self['request'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
      define_method('threshold') { self['threshold'] }
      define_method('usage') { self['usage'] }
      define_method('accepted') { self['accepted'] }
      define_method('answer') { self['answer'] }
      define_method('answer_id') { self['answer_id'] }
      define_method('probability') { self['probability'] }
    end
    class NativeRelationMemberFailureId < Native::Result
      define_method('direction') { self['direction'] }
      define_method('method') { self['method'] }
      define_method('observations') { self['observations'] }
      define_method('question') { self['question'] }
      define_method('question_sources') { self['question_sources'] }
      define_method('reads') { self['reads'] }
      define_method('relation') { self['relation'] }
      define_method('request') { self['request'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
      define_method('threshold') { self['threshold'] }
      define_method('usage') { self['usage'] }
      define_method('failure') { self['failure'] }
      define_method('failure_id') { self['failure_id'] }
    end
    class NativeSendBudgetDenialBeforeAdditionalSend < Native::Result
      define_method('kind') { self['kind'] }
    end
    class NativeSendBudgetDenialBeforeFirstSend < Native::Result
      define_method('kind') { self['kind'] }
    end
    class NativeSendBudgetDenialBeforeRetry < Native::Result
      define_method('kind') { self['kind'] }
      define_method('last_status') { self['last_status'] }
    end
    class NativeStopped < Native::Result
      define_method('at') { self['at'] }
      define_method('cause') { self['cause'] }
      define_method('retryable') { self['retryable'] }
      define_method('status') { self['status'] }
    end
    class NativeStringRoot < Native::Result
      define_method('type') { self['type'] }
    end
    class NativeUsage < Native::Result
      define_method('input_tokens') { self['input_tokens'] }
      define_method('output_tokens') { self['output_tokens'] }
    end
    class NativeAnswerChoice < Native::Result
      define_method('confidence') { self['confidence'] }
      define_method('kind') { self['kind'] }
      define_method('pick') { self['pick'] }
      define_method('probabilities') { self['probabilities'] }
    end
    class NativeAnswerScore < Native::Result
      define_method('confidence') { self['confidence'] }
      define_method('kind') { self['kind'] }
      define_method('level') { self['level'] }
      define_method('probabilities') { self['probabilities'] }
    end
    class NativeAnswerTag < Native::Result
      define_method('kind') { self['kind'] }
      define_method('probabilities') { self['probabilities'] }
    end
    class NativeAnswerYesNo < Native::Result
      define_method('kind') { self['kind'] }
      define_method('probability') { self['probability'] }
    end
    class NativeBatchWarning < Native::Result
      define_method('running') { self['running'] }
      define_method('tuned_for') { self['tuned_for'] }
    end
    class NativeEntity < Native::Result
      define_method('end') { self['end'] }
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('kind') { self['kind'] }
      define_method('last_line') { self['last_line'] }
      define_method('length') { self['length'] }
      define_method('start') { self['start'] }
      define_method('strength') { self['strength'] }
      define_method('text') { self['text'] }
    end
    class NativeEntityEdge < Native::Result
      define_method('either') { self['either'] }
      define_method('probability') { self['probability'] }
      define_method('relation') { self['relation'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
    end
    class NativeFailed < Native::Result
      define_method('failed') { self['failed'] }
    end
    class NativeFailure < Native::Result
      define_method('cause') { self['cause'] }
      define_method('kind') { self['kind'] }
    end
    class NativeFindAnswer < Native::Result
      define_method('confidence') { self['confidence'] }
      define_method('kind') { self['kind'] }
      define_method('pick') { self['pick'] }
      define_method('probabilities') { self['probabilities'] }
    end
    class NativeNameOdds < Native::Result
      define_method('edges') { self['edges'] }
      define_method('end') { self['end'] }
      define_method('kinds') { self['kinds'] }
      define_method('start') { self['start'] }
    end
    class NativePairOdds < Native::Result
      define_method('probability') { self['probability'] }
      define_method('relation') { self['relation'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
    end
    class NativePieceOdds < Native::Result
      define_method('end') { self['end'] }
      define_method('start') { self['start'] }
      define_method('tags') { self['tags'] }
    end
    class NativePlace < Native::Result
      define_method('end') { self['end'] }
      define_method('start') { self['start'] }
    end
    class NativeProfileWarning < Native::Result
      define_method('running') { self['running'] }
      define_method('tuned_for') { self['tuned_for'] }
    end
    class NativeRecognizeAnswer < Native::Result
      define_method('names') { self['names'] }
      define_method('pairs') { self['pairs'] }
      define_method('pieces') { self['pieces'] }
      define_method('proposals') { self['proposals'] }
    end
    class NativeRecognizeFieldsEntities < Native::Result
      define_method('entities') { self['entities'] }
      define_method('relations') { self['relations'] }
    end
    class NativeRecognizeFieldsModeProposals < Native::Result
      define_method('mode') { self['mode'] }
      define_method('proposals') { self['proposals'] }
    end
    class NativeRelateFields < Native::Result
      define_method('kind') { self['kind'] }
      define_method('name') { self['name'] }
    end
    class NativeRelatedEntity < Native::Result
      define_method('kind') { self['kind'] }
      define_method('name') { self['name'] }
    end
    class NativeRelatedEntityEdge < Native::Result
      define_method('either') { self['either'] }
      define_method('probability') { self['probability'] }
      define_method('relation') { self['relation'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
    end
    class NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord < Native::Result
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('kind') { self['kind'] }
      define_method('last_line') { self['last_line'] }
      define_method('name') { self['name'] }
      define_method('ordinal') { self['ordinal'] }
      define_method('record') { self['record'] }
    end
    class NativeRelatedEntityEdgePropertiesSourceFieldsKindName < Native::Result
      define_method('kind') { self['kind'] }
      define_method('name') { self['name'] }
    end
    class NativeRelationRule < Native::Result
      define_method('either') { self['either'] }
      define_method('name') { self['name'] }
      define_method('reads') { self['reads'] }
      define_method('single') { self['single'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
    end
    class NativeSessionAnnotation < Native::Result
      define_method('name') { self['name'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionInputSource < Native::Result
      define_method('index') { self['index'] }
      define_method('source') { self['source'] }
    end
    class NativeSessionJudgmentChoice < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionJudgmentDecision < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionJudgmentScore < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionJudgmentTags < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionNamedProbability < Native::Result
      define_method('name') { self['name'] }
      define_method('probability') { self['probability'] }
    end
    class NativeSessionObservationQuestion < Native::Result
      define_method('detail') { self['detail'] }
      define_method('index') { self['index'] }
      define_method('kind') { self['kind'] }
      define_method('member') { self['member'] }
      define_method('position') { self['position'] }
      define_method('stage') { self['stage'] }
    end
    class NativeSessionObservationRow < Native::Result
      define_method('index') { self['index'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionObservedRowAnnotated < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionObservedRowFind < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionObservedRowJudgment < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionObservedRowRecognized < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionObservedRowRelations < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketAnnotateAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketAnnotateRow < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketChooseAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketChooseRow < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketDecideAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketDecideRow < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketFilterAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketFilterRow < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketFindAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketObservation < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketRankAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketRecognizeAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketRelateAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketScoreAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketScoreRow < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketTagAggregate < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketTagRow < Native::Result
      define_method('function') { self['function'] }
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionPacketTerminal < Native::Result
      define_method('facts') { self['facts'] }
      define_method('failure') { self['failure'] }
      define_method('kind') { self['kind'] }
    end
    class NativeSessionProbabilitiesNamed < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionProbabilitiesYesNo < Native::Result
      define_method('kind') { self['kind'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionQuestionDetail < Native::Result
      define_method('answer_id') { self['answer_id'] }
      define_method('cached') { self['cached'] }
      define_method('confidence') { self['confidence'] }
      define_method('failed_questions') { self['failed_questions'] }
      define_method('failure') { self['failure'] }
      define_method('failure_id') { self['failure_id'] }
      define_method('input') { self['input'] }
      define_method('input_source') { self['input_source'] }
      define_method('input_sources') { self['input_sources'] }
      define_method('inputs') { self['inputs'] }
      define_method('model') { self['model'] }
      define_method('observations') { self['observations'] }
      define_method('probabilities') { self['probabilities'] }
      define_method('question') { self['question'] }
      define_method('question_sha256') { self['question_sha256'] }
      define_method('question_sources') { self['question_sources'] }
      define_method('raw_pick') { self['raw_pick'] }
      define_method('reported_usage') { self['reported_usage'] }
      define_method('requests') { self['requests'] }
      define_method('requests_sent') { self['requests_sent'] }
      define_method('threshold') { self['threshold'] }
      define_method('url') { self['url'] }
      define_method('usage') { self['usage'] }
      define_method('value') { self['value'] }
    end
    class NativeSessionRecognition < Native::Result
      define_method('entities') { self['entities'] }
      define_method('mode') { self['mode'] }
      define_method('proposals') { self['proposals'] }
      define_method('relations') { self['relations'] }
    end
    class NativeSessionRelationEdge < Native::Result
      define_method('either') { self['either'] }
      define_method('probability') { self['probability'] }
      define_method('relation') { self['relation'] }
      define_method('source') { self['source'] }
      define_method('target') { self['target'] }
    end
    class NativeSourceRelationEndpoint < Native::Result
      define_method('file') { self['file'] }
      define_method('first_line') { self['first_line'] }
      define_method('kind') { self['kind'] }
      define_method('last_line') { self['last_line'] }
      define_method('name') { self['name'] }
      define_method('ordinal') { self['ordinal'] }
      define_method('record') { self['record'] }
    end
    class NativeTokenUsage < Native::Result
      define_method('input_tokens') { self['input_tokens'] }
      define_method('output_tokens') { self['output_tokens'] }
    end
  end
end
module ThinkThen
  class Client
    REQUEST_VERSION = 'thinkthen.request/1'.freeze
  end
end

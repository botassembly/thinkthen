# frozen_string_literal: true
require 'thinkthen'
document = JSON.parse(STDIN.gets)
prefix=[]
begin
  settings = document.fetch('settings').transform_keys(&:to_sym)
  engine = ThinkThen::Engine.new(**settings)
  cancel = ThinkThen::Cancel.new
  cancel.cancel if document['cancel']
  Thread.new { sleep 0.15;cancel.cancel } if document['held_cancel']
  functions = engine.complete
  method = %w[decide choose tag score filter rank find annotate recognize relate].find { |v| v == document['verb'] }
  raise 'unknown function' unless method
  if document['incremental']
    batch=functions.public_send(method+'_batch',document['question'],document['input'],attempts:true,cancel:cancel,deadline_ms:document['deadline_ms'])
    if document["batch_probe"]
      puts "ready";STDOUT.flush;STDIN.gets
    end
    batch.each { |row| prefix << row }
    done=ThinkThen::Complete::Completed.new(results:prefix.map(&:result),facts:batch.facts,ordinals:prefix.map(&:ordinal),inputs:prefix.map(&:input))
  else
    done = functions.public_send(method, document['question'], document['input'], cancel: cancel, deadline_ms: document['deadline_ms'], attempts: true,context:document['shared_context'])
  end
  raise 'native identity' unless done.facts.call_id.is_a?(ThinkThen.const_get(:Complete)::CallId)
  c = ThinkThen.const_get(:Complete)
  facts=done.facts
  encoded=c.to_json_value(facts)
  raise 'native request facts' unless facts.largest_request_bytes.is_a?(Integer) && (facts.largest_request_estimated_input_tokens.nil? || facts.largest_request_estimated_input_tokens.is_a?(Integer)) && facts.token_estimate_method.is_a?(String)
  %w[largest_request_bytes largest_request_estimated_input_tokens token_estimate_method].each { |key| raise 'lost request facts' unless encoded.fetch(key)==facts.public_send(key) }
  unless facts.usage_persistence.equal?(c::ABSENT)
    observation=facts.usage_persistence
    raise 'native persistence observation' unless observation.is_a?(c::PersistenceObservation) && %w[disabled pending written failed].include?(observation.state) && observation.observed_at=='facts_snapshot'
    raise 'lost persistence observation' unless encoded.fetch('usage_persistence')==c.to_json_value(observation)
  end
  done.results.each do |r|
    next unless r.is_a?(c::RankResult) && !r.members.equal?(c::ABSENT)
    r.members.each do |m|
      raise 'native rank member' unless m.is_a?(c::RankMember) && m.result.is_a?(c::RankMemberResult) && m.result.answer_id.is_a?(c::AnswerId) && m.result.value.positive? && m.result.question.is_a?(c::DecideQuestion) && m.result.answer.is_a?(c::YesNo) && m.result.meta.is_a?(c::Meta)
    end
  end
  puts JSON.generate({results:done.results.map { |r| c.to_json_value(r) }, facts:c.to_json_value(done.facts), ordinals:done.ordinals, inputs:done.inputs.map { |i| c.to_json_value(i) }})
rescue ThinkThen::Error => error
  if error.complete
    c=ThinkThen::Complete
    puts JSON.generate({completed:prefix.empty? ? nil : {results:prefix.map { |r| c.to_json_value(r.result) },facts:c.to_json_value(error.complete.facts),ordinals:prefix.map(&:ordinal),inputs:prefix.map { |r| c.to_json_value(r.input) }},error:c.to_json_value(error.complete),facts:error.complete.facts.equal?(c::ABSENT) ? nil : c.to_json_value(error.complete.facts)})
  else
  puts JSON.generate({error:{kind:error.kind, message:error.message, retryable:error.retryable}, facts:error.respond_to?(:facts) ? error.facts : nil})
  end
end

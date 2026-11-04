require "thinkthen"

messages = [
  {"id" => "a1", "text" => "Please refund my order."},
  {"id" => "a2", "text" => "Where is my parcel?"},
  {"id" => "a3", "text" => "I was charged twice for one invoice."},
]
question = "Does the customer request a refund?"

ThinkThen::Client.open do |client|
  # One message: true or false, raising when not sure or failed.
  is_refund = client.decide?(question, messages.first["text"])
  puts "first message: #{is_refund}"

  # Many messages in one call: one row each, in input order.
  client.decide_each(question, messages, field: "/text") do |refund_check|
    case refund_check
    in ThinkThen::Answered then puts "#{refund_check.id} #{refund_check.yes?}"
    in ThinkThen::Unsure then puts "#{refund_check.id} not sure: ask a person"
    in ThinkThen::Failed then
      puts "#{refund_check.id} failed: #{refund_check.error.code}"
    end
  end
end

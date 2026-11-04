require "thinkthen"

message = "Morgan works for Example Group.\nPlease refund my order."
works_for = ThinkThen::RelationRule.new(source: "person",
                                        target: "organization",
                                        relation: "works_for")
ThinkThen::Client.open do |client|
  relations = client.relate(message, rules: [works_for]).answer
  relations.each do |relation|
    puts "#{relation.source.text} --#{relation.relation}--> " \
         "#{relation.target.text}"
  end
end

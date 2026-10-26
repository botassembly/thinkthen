# frozen_string_literal: true

# Feature: saved entries restore typed Ruby questions and preserve execution.
def ruby_toolbox_json
  root = File.join(ENV.fetch("THINKTHEN_CASES"), "toolbox")
  json = File.read(File.join(root, "questions.json"))
  box = ThinkThen::Toolbox.from_json(json)
  assert(box["ready"].is_a?(ThinkThen::FilterQuestion), "loaded filter is typed")
  assert(box["topic"].is_a?(ThinkThen::ChooseQuestion), "loaded choice is typed")
  assert(box["names"].is_a?(ThinkThen::RelateQuestion), "loaded relation is typed")
  equal(JSON.parse(json), JSON.parse(box.to_json), "saved options survive")
  again = ThinkThen::Toolbox.from_json(box.to_json)
  ThinkThen::Client.open(backend: :stub) do |client|
    inline = ThinkThen::ChooseQuestion.new("Which topic?", choices: %w[billing shipping], threshold: 0.7, evidence: "/body")
    equal(client.choose(inline, {"body" => "billing inquiry"}).to_h,
          client.choose(again["topic"], {"body" => "billing inquiry"}).to_h, "loaded choice keeps execution")
    constructed = ThinkThen::Toolbox.new(questions: {"choice" => inline})
    equal(inline.value, ThinkThen::Toolbox.from_json(constructed.to_json)["choice"].value, "constructed toolbox saves")
  end
  JSON.parse(File.read(File.join(root, "malformed.json"))).each do |bad|
    error = raises(ThinkThen::UsageError) { ThinkThen::Toolbox.from_json(bad) }
    equal(:"usage.question", error.code, "shared malformed admission")
  end
end

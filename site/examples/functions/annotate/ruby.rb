require "thinkthen"

ThinkThen::Client.open do |client|
  report = "Steps: click Log in. Nobody gets in."
  triage = client.annotate(
    ThinkThen::Client.question_file("form.json"), [report]
  ).value
  raise unless triage[0]["steps"] == true
  raise unless triage[0]["area"] == "login"
  raise unless triage[0]["impact"] == 1.98
end

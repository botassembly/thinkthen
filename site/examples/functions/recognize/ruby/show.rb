require "thinkthen"

ThinkThen::Client.open do |client|
  names = client.recognize("Morgan works for Example Group.",
                           kinds: ["person", "organization"]).answer
  names.each { |name| puts "#{name.kind}: #{name.text}" }
end

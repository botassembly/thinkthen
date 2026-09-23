# Print the loaded module's public names as JSON, for
# scripts/check_public_names.py --ruby-runtime (run by check.sh): the
# constants, the public module methods, and the public methods a record
# Struct adds beyond its members. A source scan cannot see names the
# native half defines (surfaces-review-5: Native and _parse_question).
#
#   ruby -I lib tests/public_names.rb > names.json
require "json"
require "thinkthen"

names = ThinkThen.constants.map(&:to_s) + ThinkThen.singleton_methods(false).map(&:to_s)
ThinkThen.constants.each do |name|
  klass = ThinkThen.const_get(name)
  next unless klass.is_a?(Class) && klass < Struct

  own = klass.public_instance_methods(false).map(&:to_s)
  names += own - klass.members.flat_map { |member| [member.to_s, "#{member}="] }
end
puts JSON.generate(names.uniq.sort)

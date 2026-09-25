# The Ruby helper's proof, run by test.sh with sentinels in this process.
require_relative "children"

PROBE = 'test -z "${THINKTHEN_SENTINEL+x}" && test -z "${FAKE_SERVICE_API_KEY+x}" && ' \
        'test -z "${ABSENT_0127+x}" && test "$KEPT_0127" = kept && test "$SET_0127" = set'
REFUSED = %w[THINKTHEN_BASE_URL OPENAI_API_KEY GITHUB_TOKEN db_password AWS_SECRET_ACCESS_KEY].freeze

env = Children.env(keep: %w[KEPT_0127 ABSENT_0127], SET_0127: "set")
bad = system(env, "sh", "-c", PROBE, unsetenv_others: true) ? [] : ["the child"]
REFUSED.each do |name|
  Children.env(keep: [name])
  bad << name
rescue ArgumentError => e
  sentence = "a test child may not keep #{name} from the parent: set a THINKTHEN_ value or a fake key explicitly"
  bad << e.message unless e.message == sentence
end
puts "children ruby: #{bad.empty? ? 'ok' : "FAIL #{bad.join(', ')}"}"
exit(bad.empty? ? 0 : 1)
